use amethyst::{
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Moving, Transform};

#[derive(SystemDesc)]
pub struct MovingSystem;

impl<'s> System<'s> for MovingSystem {
    type SystemData = (ReadStorage<'s, Moving>, WriteStorage<'s, Transform>);

    fn run(&mut self, (movings, mut transforms): Self::SystemData) {
        for (moving, transform) in (&movings, &mut transforms).join() {
            transform.0 += moving.0;
        }
    }
}
