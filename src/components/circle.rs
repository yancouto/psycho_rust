use amethyst::ecs::{Component, DenseVecStorage};

/// Component for the circles we wish to draw to the screen
#[derive(Debug, Default, Component)]
pub struct Circle {
    pub radius: f32,
    pub color: [f32; 3],
}
