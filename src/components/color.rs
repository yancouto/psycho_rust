use amethyst::ecs::{Component, DenseVecStorage};

/// Components that have a single color and are drawn on screen
#[derive(Debug, Component, Clone)]
pub struct Color(pub [f32; 4]);

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b, 1.0])
    }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

    pub fn inner(&self) -> [f32; 4] {
        self.0
    }
}
