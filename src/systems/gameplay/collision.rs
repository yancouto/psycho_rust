use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ParJoin, ReadStorage, System, SystemData},
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::components::{circle::collides, Circle, Enemy, InScreen, Shot, Transform};

#[derive(SystemDesc, Default)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, InScreen>,
    );

    fn run(
        &mut self,
        (entities, shots, enemies, transforms, circles, in_screens): Self::SystemData,
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
                        dead.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            });
        for (dead, (e_id, ..)) in enemies {
            if dead.into_inner() {
                entities.delete(e_id).unwrap();
            }
        }
    }
}
