use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Circle, Particle};

#[derive(SystemDesc, Default)]
pub struct FadeSystem;

impl<'s> System<'s> for FadeSystem {
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        ReadStorage<'s, Particle>,
        WriteStorage<'s, Circle>,
    );

    fn run(&mut self, (time, entities, particles, mut circles): Self::SystemData) {
        let now = time.absolute_time();
        for (p_id, particle, circle) in (&entities, &particles, &mut circles).join() {
            if particle.percent(now) == 1. {
                entities.delete(p_id).unwrap();
            }
        }
    }
}
