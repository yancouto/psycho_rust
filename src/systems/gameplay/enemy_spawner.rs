use amethyst::{
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

impl EnemySpawner {
    fn adjust_indicator(&self, triangle: &mut Triangle) {
        *triangle = Triangle::new([10., 10.], [100., 100.], [100., 10.])
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
