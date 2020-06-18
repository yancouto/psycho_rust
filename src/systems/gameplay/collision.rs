use amethyst::{
    core::math::Rotation2,
    core::timing::Time,
    derive::SystemDesc,
    ecs::{
        world::Builder, Entities, Join, LazyUpdate, ParJoin, Read, ReadStorage, System, SystemData,
    },
};
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::{
    components::{circle::collides, BallEnemy, Circle, InScreen, Moving, Shot, Transform},
    systems::particles::create_explosion,
    utils::creator::LazyCreator,
};

impl BallEnemy {
    fn on_destroy(
        &self,
        time: &Time,
        creator: &LazyCreator,
        transform: &Transform,
        moving: &Moving,
        circle: &Circle,
    ) {
        create_explosion(&time, &creator, transform.0, circle.radius, 25);
        match self {
            BallEnemy::Simple => {}
            BallEnemy::Double => {
                for i in 0..2 {
                    let rot = Rotation2::new((30. * (i as f32) - 15.).to_radians());
                    creator.create_enemy(
                        BallEnemy::Simple,
                        Circle {
                            radius: circle.radius * 0.6,
                        },
                        Transform::from(transform.0 + moving.0 * time.fixed_seconds()),
                        Moving::from(rot * moving.0),
                    );
                }
            }
        }
    }
}

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
        ReadStorage<'s, Moving>,
        ReadStorage<'s, Circle>,
        ReadStorage<'s, InScreen>,
    );

    fn run(
        &mut self,
        (time, lazy, entities, shots, enemies, transforms, movings, circles, in_screens): Self::SystemData,
    ) {
        let enemies = (
            &entities,
            &enemies,
            &transforms,
            &movings,
            &circles,
            &in_screens,
        )
            .join()
            .map(|x| (AtomicBool::new(false), x))
            .collect::<Vec<_>>();
        let creator = LazyCreator::new(&lazy, &entities);
        (&entities, &shots, &transforms, &circles, &in_screens)
            // Let's use multiple threads because why not
            // If this really becomes a problem, there are faster ways to
            // implement this collision
            .par_join()
            .for_each(|(s_id, _shot, s_t, s_c, _in_screen)| {
                for (dead, (e_id, _enemy, e_t, _e_m, e_c, _in_screen)) in enemies.iter() {
                    if collides(e_t, e_c, s_t, s_c, 0.) {
                        entities.delete(s_id).unwrap();
                        create_explosion(&time, &creator, s_t.0, s_c.radius, 10);
                        dead.store(true, Ordering::Relaxed);
                        break;
                    }
                }
            });
        for (dead, (e_id, enemy, e_t, e_m, e_c, _)) in enemies {
            if dead.into_inner() {
                entities.delete(e_id).unwrap();
                enemy.on_destroy(&time, &creator, e_t, e_m, e_c);
            }
        }
    }
}
