//! I don't understand most of this code. It was adapted from https://bit.ly/2WDuRFT
//! I understand a bit the shaders and how the vertices are generated

//! The idea of how to draw the circles comes from https://bit.ly/2YJg5QJ
//! For each circle, we draw two triangles, to form a square. As well as giving
//! the position of each vertex and color to the vertex shader, we also tag
//! the four vertices with (-1, -1), (-1, 1), (1, -1) and (1, 1).
//! This way, when they are interpolated in the fragment shader, we can
//! calculate whether a pixel is inside or outside the circle.
//! For now we just have a hard boundary, but this allows nicer things
//! like borders and rings in the future.

//! This is a bit inefficient for two reasons: We are using 6 vertices for
//! each circle, two of them are clearly repeated. Other than that, the color
//! is always the same for these 6 vertices, so it is unnecessarily copied 5 times!
//! I don't know if there's an easy way to fix this (or if it matters that much).
use amethyst::{
    core::ecs::{DispatcherBuilder, Join, ReadStorage, SystemData, World},
    core::math::{Point2, Vector2},
    prelude::*,
    renderer::{
        bundle::{RenderOrder, RenderPlan, RenderPlugin, Target},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        rendy::{
            command::{QueueId, RenderPassEncoder},
            factory::Factory,
            graph::{
                render::{PrepareResult, RenderGroup, RenderGroupDesc},
                GraphContext, NodeBuffer, NodeImage,
            },
            hal::{self, device::Device, format::Format, pso, pso::ShaderStageFlags},
            mesh::{AsVertex, VertexFormat},
            shader::{Shader, SpirvShader},
        },
        submodules::DynamicVertexBuffer,
        types::Backend,
        util, ChangeDetection,
    },
};
use glsl_layout::*;
use lazy_static::lazy_static;

use crate::{
    components::{Circle, Color, Transform, Triangle},
    display::{HEIGHT, WIDTH},
};

fn compile_shader(code: &'static str, kind: shaderc::ShaderKind) -> shaderc::CompilationArtifact {
    let mut compiler = shaderc::Compiler::new().unwrap();
    compiler
        .compile_into_spirv(code, kind, "shader.glsl", "main", None)
        .expect("Failed to compile shader")
}

lazy_static! {
    static ref VERTEX_CIRCLE: SpirvShader = SpirvShader::from_bytes(
        compile_shader(
            include_str!("../../assets/shaders/circle.vert"),
            shaderc::ShaderKind::Vertex
        )
        .as_binary_u8(),
        ShaderStageFlags::VERTEX,
        "main",
    )
    .unwrap();
    static ref FRAGMENT_CIRCLE: SpirvShader = SpirvShader::from_bytes(
        compile_shader(
            include_str!("../../assets/shaders/circle.frag"),
            shaderc::ShaderKind::Fragment
        )
        .as_binary_u8(),
        ShaderStageFlags::FRAGMENT,
        "main",
    )
    .unwrap();
    static ref VERTEX_SIMPLE: SpirvShader = SpirvShader::from_bytes(
        compile_shader(
            include_str!("../../assets/shaders/simple.vert"),
            shaderc::ShaderKind::Vertex
        )
        .as_binary_u8(),
        ShaderStageFlags::VERTEX,
        "main",
    )
    .unwrap();
    static ref FRAGMENT_SIMPLE: SpirvShader = SpirvShader::from_bytes(
        compile_shader(
            include_str!("../../assets/shaders/simple.frag"),
            shaderc::ShaderKind::Fragment
        )
        .as_binary_u8(),
        ShaderStageFlags::FRAGMENT,
        "main",
    )
    .unwrap();
}

fn build_custom_pipeline<B: Backend>(
    factory: &Factory<B>,
    subpass: hal::pass::Subpass<'_, B>,
    framebuffer_width: u32,
    framebuffer_height: u32,
    layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(B::GraphicsPipeline, B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe {
        factory
            .device()
            .create_pipeline_layout(layouts, None as Option<(_, _)>)
    }?;

    let vertex_circle = unsafe { VERTEX_CIRCLE.module(factory).unwrap() };
    let fragment_circle = unsafe { FRAGMENT_CIRCLE.module(factory).unwrap() };
    let vertex_simple = unsafe { VERTEX_SIMPLE.module(factory).unwrap() };
    let fragment_simple = unsafe { FRAGMENT_SIMPLE.module(factory).unwrap() };

    let default_pipeline_builder = || {
        PipelineDescBuilder::new()
            .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleList))
            .with_layout(&pipeline_layout)
            .with_subpass(subpass)
            .with_framebuffer_size(framebuffer_width, framebuffer_height)
            .with_blend_targets(vec![pso::ColorBlendDesc {
                mask: pso::ColorMask::ALL,
                blend: Some(pso::BlendState::ALPHA),
            }])
    };

    let pipes = PipelinesBuilder::new()
        .with_pipeline(
            default_pipeline_builder()
                .with_vertex_desc(&[(CircleArgs::vertex(), pso::VertexInputRate::Vertex)])
                .with_shaders(util::simple_shader_set(
                    &vertex_circle,
                    Some(&fragment_circle),
                )),
        )
        .with_pipeline(
            default_pipeline_builder()
                .with_vertex_desc(&[(TriangleArgs::vertex(), pso::VertexInputRate::Vertex)])
                .with_shaders(util::simple_shader_set(
                    &vertex_simple,
                    Some(&fragment_simple),
                )),
        )
        .build(factory, None);

    unsafe {
        factory.destroy_shader_module(vertex_circle);
        factory.destroy_shader_module(fragment_circle);
        factory.destroy_shader_module(vertex_simple);
        factory.destroy_shader_module(fragment_simple);
    }

    match pipes {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(mut pipes) => {
            debug_assert!(pipes.len() == 2);
            let pipe2 = pipes.remove(1);
            let pipe1 = pipes.remove(0);
            Ok((pipe1, pipe2, pipeline_layout))
        }
    }
}

#[derive(Debug)]
pub struct DrawShapeDesc;

impl DrawShapeDesc {
    pub fn new() -> Self {
        Self
    }
}

impl<B: Backend> RenderGroupDesc<B, World> for DrawShapeDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _world: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: hal::pass::Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        let (pipeline_circle, pipeline_triangle, pipeline_layout) = build_custom_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            vec![],
        )?;

        Ok(Box::new(DrawShape::<B>::new(
            pipeline_circle,
            pipeline_triangle,
            pipeline_layout,
        )))
    }
}

#[derive(Debug)]
pub struct RenderCircles;

impl<B: Backend> RenderPlugin<B> for RenderCircles {
    fn on_build<'a, 'b>(
        &mut self,
        world: &mut World,
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> amethyst::Result<()> {
        register!(Color, Circle, Transform, Triangle -> world);
        Ok(())
    }

    fn on_plan(
        &mut self,
        plan: &mut RenderPlan<B>,
        _factory: &mut Factory<B>,
        _world: &World,
    ) -> amethyst::Result<()> {
        plan.extend_target(Target::Main, |ctx| {
            ctx.add(RenderOrder::Transparent, DrawShapeDesc::new().builder())?;
            Ok(())
        });
        Ok(())
    }
}

/// Draws circles to the screen.
#[derive(Debug)]
pub struct DrawShape<B: Backend> {
    pipeline_circle: B::GraphicsPipeline,
    pipeline_triangle: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    circle_vertex: DynamicVertexBuffer<B, CircleArgs>,
    circle_count: usize,
    triangle_vertex: DynamicVertexBuffer<B, TriangleArgs>,
    triangle_count: usize,
    change: ChangeDetection,
}

impl<B: Backend> DrawShape<B> {
    fn new(
        pipeline_circle: B::GraphicsPipeline,
        pipeline_triangle: B::GraphicsPipeline,
        pipeline_layout: B::PipelineLayout,
    ) -> Self {
        Self {
            pipeline_circle,
            pipeline_triangle,
            pipeline_layout,
            circle_vertex: DynamicVertexBuffer::new(),
            circle_count: 0,
            triangle_vertex: DynamicVertexBuffer::new(),
            triangle_count: 0,
            change: Default::default(),
        }
    }
}

impl<B: Backend> RenderGroup<B, World> for DrawShape<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        let (circles, triangles, transforms, colors) = <(
            ReadStorage<'_, Circle>,
            ReadStorage<'_, Triangle>,
            ReadStorage<'_, Transform>,
            ReadStorage<'_, Color>,
        )>::fetch(world);

        // Create all vertices
        let circle_data = (&circles, &transforms, &colors)
            .join()
            .flat_map(|(circle, t, color)| circle.get_vertices(t, color))
            .collect::<Box<[CircleArgs]>>();

        let triangle_data = (&triangles, &colors)
            .join()
            .flat_map(|(triangle, color)| triangle.get_vertices(color))
            .collect::<Box<[TriangleArgs]>>();

        //Update vertex count and see if it has changed
        let old_vertex_count = self.circle_count;
        self.circle_count = circle_data.len();
        let changed = old_vertex_count != self.circle_count;

        let old_vertex_count = self.triangle_count;
        self.triangle_count = triangle_data.len();
        let changed = changed || old_vertex_count != self.triangle_count;

        // Write the vector to a Vertex buffer
        self.circle_vertex
            .write(factory, index, self.circle_count as u64, Some(circle_data));
        self.triangle_vertex.write(
            factory,
            index,
            self.triangle_count as u64,
            Some(triangle_data),
        );

        // Return with we can reuse the draw buffers using the utility struct ChangeDetection
        self.change.prepare_result(index, changed)
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        _world: &World,
    ) {
        // Don't worry about drawing if there are no vertices. Like before the state adds them to the screen.
        if self.circle_count == 0 && self.triangle_count == 0 {
            return;
        }

        // Drawing triangles
        encoder.bind_graphics_pipeline(&self.pipeline_triangle);
        self.triangle_vertex.bind(index, 0, 0, &mut encoder);
        unsafe {
            encoder.draw(0..self.triangle_count as u32, 0..1);
        }

        // Bind the pipeline to the the encoder
        encoder.bind_graphics_pipeline(&self.pipeline_circle);

        // Bind the vertex buffer to the encoder
        self.circle_vertex.bind(index, 0, 0, &mut encoder);

        // Draw the vertices
        unsafe {
            encoder.draw(0..self.circle_count as u32, 0..1);
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _world: &World) {
        unsafe {
            factory
                .device()
                .destroy_graphics_pipeline(self.pipeline_circle);
            factory
                .device()
                .destroy_graphics_pipeline(self.pipeline_triangle);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

/// Vertex Arguments to pass into shader.
/// Check the shader at assets/shaders/circle.vert
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct CircleArgs {
    /// Position of the vertex, see top of file
    pub pos: vec2,
    /// Color of the circle
    pub color: vec4,
    /// Represents whether it is top/bottom left/right, using +1/-1
    pub rel: vec2,
}

/// Required to send data into the shader.
/// These names must match the shader.
impl AsVertex for CircleArgs {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            // vec2 pos;
            (Format::Rg32Sfloat, "pos"),
            // vec4 color;
            (Format::Rgba32Sfloat, "color"),
            // vec2 rel;
            (Format::Rg32Sfloat, "rel"),
        ))
    }
}

// Transformation from (0, 0) -> (W, H) to (-1, -1) -> (1, 1)
fn transform_point(p: &Point2<f32>) -> vec2 {
    let p = (p - Vector2::new(WIDTH / 2., HEIGHT / 2.))
        .coords
        .component_div(&Vector2::new(WIDTH / 2., HEIGHT / 2.));
    [p.x, p.y].into()
}

impl Circle {
    /// Helper function to convert triangle into 3 vertices
    pub fn get_vertices(&self, t: &Transform, color: &Color) -> Vec<CircleArgs> {
        let (c, r) = (t.0, self.radius);
        let color = color.inner().into();
        let square = vec![
            Vector2::new(-1., -1.),
            Vector2::new(-1., 1.),
            Vector2::new(1., 1.),
            Vector2::new(1., -1.),
        ];
        let square = vec![
            square[0], square[1], square[2], square[0], square[2], square[3],
        ];
        square
            .into_iter()
            .map(|rel| {
                // Creating edge of the square
                let p = c + r * rel;
                CircleArgs {
                    pos: transform_point(&p),
                    color,
                    rel: [rel.x, rel.y].into(),
                }
            })
            .collect()
    }
}

/// Vertex Arguments to pass into shader.
/// Check the shader at assets/shaders/simple.vert
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, AsStd140)]
#[repr(C, align(4))]
pub struct TriangleArgs {
    pub pos: vec2,
    pub color: vec4,
}

/// Required to send data into the shader.
/// These names must match the shader.
impl AsVertex for TriangleArgs {
    fn vertex() -> VertexFormat {
        VertexFormat::new((
            // vec2 pos;
            (Format::Rg32Sfloat, "pos"),
            // vec4 color;
            (Format::Rgba32Sfloat, "color"),
        ))
    }
}

impl Triangle {
    pub fn get_vertices(&self, color: &Color) -> Vec<TriangleArgs> {
        let color = color.inner().into();
        self.vertices
            .iter()
            .map(|v| TriangleArgs {
                pos: transform_point(&v),
                color,
            })
            .collect()
    }
}
