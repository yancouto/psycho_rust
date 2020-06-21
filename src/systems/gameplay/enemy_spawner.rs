use amethyst::{
    core::math::Vector2,
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::{
    components::{EnemySpawner, Triangle},
    display::{HEIGHT as H, WIDTH as W},
    utils::creator::LazyCreator,
};

#[derive(SystemDesc, Default)]
pub struct EnemySpawnerSystem;

const MARGIN: f32 = 20.;
const SIZE: f32 = 18.;

impl EnemySpawner {
    fn adjust_indicator(&self, triangle: &mut Triangle) {
        let mut center = self.position;
        center.x = center.x.clamp(MARGIN, W - MARGIN);
        center.y = center.y.clamp(MARGIN, H - MARGIN);
        let unit = self.calc_speed(None).normalize();
        // Perpendicular to unit
        let perp = Vector2::new(-unit.y, unit.x);
        triangle.vertices = [
            center + unit * SIZE,
            center - unit * SIZE + perp * SIZE / 2.,
            center - unit * SIZE - perp * SIZE / 2.,
        ];
    }
}

/// Spawns enemies set up with the EnemySpawner
impl<'s> System<'s> for EnemySpawnerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        ReadStorage<'s, EnemySpawner>,
        WriteStorage<'s, Triangle>,
    );

    fn run(&mut self, (time, lazy, entities, spawners, mut triangles): Self::SystemData) {
        let creator = LazyCreator {
            lazy: &lazy,
            entities: &entities,
        };
        for (id, spawner, mut triangle) in (&entities, &spawners, &mut triangles).join() {
            if spawner.spawn_at <= time.absolute_time_seconds() {
                spawner.do_spawn(&creator);
                entities.delete(id);
            } else {
                spawner.adjust_indicator(&mut triangle);
            }
        }
    }
}
