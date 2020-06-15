use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, ParJoin, Read, ReadStorage, System, SystemData},
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    components::{circle::collides, BallEnemy, Circle, InScreen, Shot, Transform},
    systems::particles::create_explosion,
};

#[derive(SystemDesc, Default)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, BallEnemy>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, InScreen>,
    );

    fn run(
        &mut self,
        (time, lazy, entities, shots, enemies, transforms, circles, in_screens): Self::SystemData,
    ) {
        let enemies = (&entities, &enemies, &transforms, &circles, &in_screens)
            .join()
            .map(|x| (AtomicBool::new(false), x))
            .collect::<Vec<_>>();
        (&entities, &shots, &transforms, &circles, &in_screens)
            // Let's use multiple threads because why not
            // If this really becomes a problem, there are faster ways to
            // implement this collision
            .par_join()
            .for_each(|(s_id, _shot, s_t, s_c, _in_screen)| {
                for (dead, (e_id, _enemy, e_t, e_c, _in_screen)) in enemies.iter() {
                    if collides(e_t, e_c, s_t, s_c, 0.) {
                        entities.delete(s_id).unwrap();
                        create_explosion(&time, &lazy, &entities, s_t.0, s_c.radius, 10);
                        dead.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            });
        for (dead, (e_id, _, e_t, e_c, _)) in enemies {
            if dead.into_inner() {
                entities.delete(e_id).unwrap();
                create_explosion(&time, &lazy, &entities, e_t.0, e_c.radius, 25);
            }
        }
    }
}
