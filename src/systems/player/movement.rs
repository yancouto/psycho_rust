use amethyst::{
    core::math::{Point2, Vector2},
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
    input::InputHandler,
};

use crate::{
    components::{Player, Transform},
    input::{AxisBinding, PsychoBindingTypes},
};

#[derive(SystemDesc, Default)]
pub struct MoveSystem;

const PSYCHO_SPEED: f32 = 10.;

#[derive(Debug, Clone)]
pub struct PlayerPosition(Point2<f32>);

impl Default for PlayerPosition {
    fn default() -> Self {
        Self(Point2::new(0., 0.))
    }
}

impl<'s> System<'s> for MoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<PsychoBindingTypes>>,
        Write<'s, PlayerPosition>,
    );

    fn run(&mut self, (mut transforms, player, input, mut player_pos): Self::SystemData) {
        for (_player, transform) in (&player, &mut transforms).join() {
            let dir = Vector2::new(
                input.axis_value(&AxisBinding::Horizontal).unwrap(),
                -input.axis_value(&AxisBinding::Vertical).unwrap(),
            );
            if dir.x != 0. || dir.y != 0. {
                transform.0 += PSYCHO_SPEED * dir.normalize();
            }
            player_pos.0 = transform.0;
        }
    }
}
