use amethyst::ecs::{Component, DenseVecStorage};

use amethyst::core::math::Point2;

/// Component for the triangles we wish to draw to the screen
#[derive(Debug, Component, Clone)]
pub struct Triangle {
    pub vertices: [Point2<f32>; 3],
}

impl Triangle {
    pub fn new<P: Into<Point2<f32>>>(a: P, b: P, c: P) -> Self {
        Self {
            vertices: [a.into(), b.into(), c.into()],
        }
    }
}
