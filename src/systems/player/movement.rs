use amethyst::{
    core::math::Vector2,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::InputHandler,
};

use crate::{
    components::{Player, Transform},
    input::{AxisBinding, PsychoBindingTypes},
};

#[derive(SystemDesc, Default)]
pub struct MoveSystem;

const PSYCHO_SPEED: f32 = 10.;

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<PsychoBindingTypes>>,
    );

    fn run(&mut self, (mut transforms, player, input): Self::SystemData) {
        for (_player, transform) in (&player, &mut transforms).join() {
            let dir = Vector2::new(
                input.axis_value(&AxisBinding::Horizontal).unwrap(),
                -input.axis_value(&AxisBinding::Vertical).unwrap(),
            );
            if dir.x != 0. || dir.y != 0. {
                transform.0 += PSYCHO_SPEED * dir.normalize();
            }
        }
    }
}
