use amethyst::ecs::{Component, DenseVecStorage};

use crate::components::Transform;

use std::time::Duration;

/// Component for particles in the particle system
#[derive(Debug, Component)]
pub struct Particle {
    pub created: f32,
    pub lifetime: f32,
}

impl Particle {
    pub fn percent(&self, now: Duration) -> f32 {
        ((now.as_secs_f32() - self.created) / self.lifetime).min(1.)
    }
}
