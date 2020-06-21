use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData},
};

use crate::{
    components::{BallEnemy, Circle, EnemySpawner, InScreen, Shot, Transform},
    display::{HEIGHT as H, WIDTH as W},
    utils::creator::LazyCreator,
};

#[derive(SystemDesc, Default)]
pub struct EnemySpawnerSystem;

/// Spawns enemies set up with the EnemySpawner
impl<'s> System<'s> for EnemySpawnerSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        ReadStorage<'s, EnemySpawner>,
    );

    fn run(&mut self, (time, lazy, entities, spawners): Self::SystemData) {
        let creator = LazyCreator {
            lazy: &lazy,
            entities: &entities,
        };
        for (id, spawner) in (&entities, &spawners).join() {
            if spawner.spawn_at <= time.absolute_time_seconds() {
                spawner.do_spawn(&creator);
                entities.delete(id);
            }
        }
    }
}
