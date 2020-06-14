use amethyst::{
    core::{math::Point2, timing::Time},
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData},
    input::InputHandler,
    prelude::*,
};

use crate::{
    components::{Circle, Color, InScreen, Moving, Player, Shot, Transform},
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

const SHOT_INTERVAL: f64 = 0.18;
const SHOT_SPEED: f32 = 20.;
const SHOT_RADIUS: f32 = 5.;

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
        if time.absolute_time_seconds() - self.last_shot < SHOT_INTERVAL {
            return ();
        }
        for (_player, circle, transform) in (&player, &circles, &transforms).join() {
            if input.action_is_down(&ActionBinding::Shoot).unwrap() {
                self.last_shot = time.absolute_time_seconds();
                let r = circle.radius;
                let center = transform.0;
                let mouse = {
                    if let Some((mx, my)) = input.mouse_position() {
                        Point2::new(mx, my)
                    } else {
                        return;
                    }
                };

                let dir = (mouse - center).normalize();
                let shot_center = center + dir * (r - SHOT_RADIUS);

                lazy.create_entity(&entities)
                    .with(Transform::from(shot_center))
                    .with(Circle::with_radius(SHOT_RADIUS))
                    .with(Color::rgb(0.1, 0.1, 0.9))
                    .with(Moving::from(dir * SHOT_SPEED))
                    .with(Shot)
                    // It is fine to delete a shot as soon as it spawns
                    .with(InScreen)
                    .build();
            }
        }
    }
}
