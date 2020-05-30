use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ParJoin, ReadStorage, System, SystemData},
};
use rayon::prelude::*;

use crate::components::{Circle, Enemy, InScreen, Shot, Transform};

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
        let shots = (&entities, &shots, &transforms, &circles, &in_screens)
            .join()
            .collect::<Vec<_>>();
        (&entities, &enemies, &transforms, &circles, &in_screens)
            // Let's use multiple threads because why not
            // If this really becomes a problem, there are faster ways to
            // implement this collision
            .par_join()
            .for_each(|(e_id, _enemy, e_t, e_c, _in_screen)| {
                for (s_id, _shot, s_t, s_c, _in_screen) in shots.clone().into_iter() {
                    let radius = e_c.radius + s_c.radius;
                    if (e_t.0 - s_t.0).norm_squared() < radius * radius {
                        entities.delete(e_id).unwrap();
                        entities.delete(s_id).unwrap();
                    }
                }
            });
    }
}
