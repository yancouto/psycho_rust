use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Read, System, SystemData, WriteStorage},
    prelude::*,
};

use crate::components::{Circle, Moving, Transform};

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
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Circle>,
        WriteStorage<'s, Moving>,
    );

    fn run(&mut self, (time, entities, mut transforms, mut circles, mut movings): Self::SystemData) {
        self.until_next_enemy -= time.delta_seconds();
        if self.until_next_enemy <= 0. {
            self.until_next_enemy = 3.;
            entities
                .build_entity()
                .with(Transform::new(10., 10.), &mut transforms)
                .with(
                    Circle {
                        radius: 10.,
                        color: [0.9, 0.1, 0.1],
                    },
                    &mut circles,
                )
                .with(Moving::new(10., 0.), &mut movings)
                .build();
        }
    }
}
