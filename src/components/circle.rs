use amethyst::ecs::{Component, DenseVecStorage};

use crate::components::Transform;

/// Component for the circles we wish to draw to the screen
#[derive(Debug, Default, Component, Clone)]
pub struct Circle {
    pub radius: f32,
}

impl Circle {
    pub fn with_radius(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn collides(t1: &Transform, c1: &Circle, t2: &Transform, c2: &Circle, threshold: f32) -> bool {
    let r = (c1.radius + c2.radius - threshold).max(0.);
    (t1.0 - t2.0).norm_squared() <= r * r
}
