use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

use crate::components::{Color, Particle};

#[derive(SystemDesc, Default)]
pub struct FadeSystem;

impl<'s> System<'s> for FadeSystem {
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        ReadStorage<'s, Particle>,
        WriteStorage<'s, Color>,
    );

    fn run(&mut self, (time, entities, particles, mut colors): Self::SystemData) {
        let now = time.absolute_time();
        for (p_id, particle, color) in (&entities, &particles, &mut colors).join() {
            let p = particle.percent(now);
            color.0[3] = 1. - p;
            if p == 1. {
                entities.delete(p_id).unwrap();
            }
        }
    }
}
