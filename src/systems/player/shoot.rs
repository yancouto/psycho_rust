use amethyst::{
    core::{math::Point2, timing::Time},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData},
    input::InputHandler,
    prelude::*,
};

use crate::{
    components::{Circle, InScreen, Moving, Player, Shot, Transform},
    input::{ActionBinding, PsychoBindingTypes},
};

#[derive(SystemDesc)]
pub struct ShootSystem {
    last_shot: f64,
}

impl Default for ShootSystem {
    fn default() -> Self {
        Self { last_shot: -100. }
    }
}

impl<'s> System<'s> for ShootSystem {
    type SystemData = (
        Read<'s, Time>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Circle>,
        Read<'s, InputHandler<PsychoBindingTypes>>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (time, transforms, player, circles, input, entities, lazy): Self::SystemData,
    ) {
        if time.absolute_time_seconds() - self.last_shot < 0.08 {
            return ();
        }
        for (_player, circle, transform) in (&player, &circles, &transforms).join() {
            if input.action_is_down(&ActionBinding::Shoot).unwrap() {
                self.last_shot = time.absolute_time_seconds();
                let shot_radius = 5.;
                let shot_speed = 20.;
                let r = circle.radius;
                let center = transform.0;
                let mouse = {
                    let (mx, my) = input.mouse_position().unwrap();
                    Point2::new(mx, my)
                };

                let dir = (mouse - center).normalize();
                let shot_center = center + dir * (r - shot_radius);

                lazy.create_entity(&entities)
                    .with(Transform::from(shot_center))
                    .with(Circle {
                        radius: shot_radius,
                        color: [0.1, 0.1, 0.9],
                    })
                    .with(Moving::from(dir * shot_speed))
                    .with(Shot)
                    // It is fine to delete a shot as soon as it spawns
                    .with(InScreen)
                    .build();
            }
        }
    }
}
