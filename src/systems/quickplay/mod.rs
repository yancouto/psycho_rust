use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, LazyUpdate, Read, System, SystemData},
    prelude::*,
};

use crate::components::{Circle, Enemy, Moving, Transform};

#[derive(SystemDesc)]
pub struct EnemySpawnSystem {
    until_next_enemy: f32,
}

impl Default for EnemySpawnSystem {
    fn default() -> Self {
        Self {
            until_next_enemy: 3.,
        }
    }
}

impl<'s> System<'s> for EnemySpawnSystem {
    type SystemData = (Read<'s, Time>, Entities<'s>, Read<'s, LazyUpdate>);

    fn run(&mut self, (time, entities, lazy): Self::SystemData) {
        self.until_next_enemy -= time.delta_seconds();
        if self.until_next_enemy <= 0. {
            self.until_next_enemy = 3.;
            lazy.create_entity(&entities)
                .with(Transform::new(10., 10.))
                .with(Circle {
                    radius: 10.,
                    color: [0.9, 0.1, 0.1],
                })
                .with(Moving::new(10., 0.))
                .with(Enemy)
                .build();
        }
    }
}
