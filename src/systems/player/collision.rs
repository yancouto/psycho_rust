use amethyst::{
    core::math::Vector2,
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
};

use crate::{
    components::{circle::collides, Circle, Enemy, InScreen, Player, Transform},
    input::{AxisBinding, PsychoBindingTypes},
};

#[derive(SystemDesc, Default)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, InScreen>,
    );

    fn run(
        &mut self,
        (entities, players, transforms, circles, enemies, in_screens): Self::SystemData,
    ) {
        let enemies = (&enemies, &in_screens, &circles, &transforms)
            .join()
            .map(|(.., circle, transform)| (circle, transform))
            .collect::<Vec<_>>();
        for (_player, p_id, p_c, p_t) in (&players, &entities, &circles, &transforms).join() {
            for (e_c, e_t) in enemies.iter() {
                if collides(p_t, p_c, e_t, e_c, 2.) {
                    // Do something prettier eventually
                    entities.delete(p_id).unwrap();
                    break;
                }
            }
        }
    }
}
