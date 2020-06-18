use amethyst::{
    core::{math::Vector2, timing::Time},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
};

use crate::{
    components::{circle::collides, BallEnemy, Circle, InScreen, Player, Transform},
    input::{AxisBinding, PsychoBindingTypes},
    systems::particles::create_explosion,
    utils::creator::LazyCreator,
};

#[derive(SystemDesc, Default)]
pub struct CollisionSystem;

impl<'s> System<'s> for CollisionSystem {
    type SystemData = (
        Read<'s, Time>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, BallEnemy>,
        ReadStorage<'s, InScreen>,
    );

    fn run(
        &mut self,
        (time, lazy, entities, players, transforms, circles, enemies, in_screens): Self::SystemData,
    ) {
        let enemies = (&enemies, &in_screens, &circles, &transforms)
            .join()
            .map(|(.., circle, transform)| (circle, transform))
            .collect::<Vec<_>>();
        let creator = LazyCreator::new(&lazy, &entities);
        for (_player, p_id, p_c, p_t) in (&players, &entities, &circles, &transforms).join() {
            for (e_c, e_t) in enemies.iter() {
                if collides(p_t, p_c, e_t, e_c, 2.) {
                    // Do something prettier eventually
                    entities.delete(p_id).unwrap();
                    create_explosion(&time, &creator, p_t.0, p_c.radius, 50);
                    break;
                }
            }
        }
    }
}
