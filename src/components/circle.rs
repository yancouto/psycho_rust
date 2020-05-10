use amethyst::ecs::{Component, DenseVecStorage};

/// Component for the circles we wish to draw to the screen
#[derive(Debug, Default)]
pub struct Circle {
    pub radius: f32,
    pub color: [f32; 3],
}

impl Component for Circle {
    type Storage = DenseVecStorage<Self>;
}
