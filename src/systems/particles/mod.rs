mod fade;
pub use fade::FadeSystem;

use amethyst::{
    core::{
        math::{Point2, RealField, Rotation2, Vector2},
        timing::Time,
    },
    ecs::{
        world::{Builder, EntitiesRes},
        LazyUpdate,
    },
};

use rand::Rng;

use crate::{
    components::{Circle, Color, Moving, Particle, Transform},
    utils::creator::LazyCreator,
};

pub fn create_explosion(
    time: &Time,
    creator: &LazyCreator,
    center: Point2<f32>,
    radius: f32,
    n: u8,
    color: &Color,
) {
    let now = time.absolute_time().as_secs_f32();
    let mut rng = rand::thread_rng();
    let mut rnd = || rng.gen::<f32>();
    for i in 0..n {
        let rnd_ang1 = rnd();
        let rnd_ang2 = rnd_ang1 + (rnd() * 0.5 - 0.25);
        let unit1 = Rotation2::new(f32::two_pi() * rnd_ang1) * Vector2::new(0., 1.);
        let unit2 = Rotation2::new(f32::two_pi() * rnd_ang2) * Vector2::new(0., 1.);
        creator
            .create_entity()
            .with(Transform::from(center + unit1 * radius * 0.75))
            .with(Moving::from(unit2 * (1.5 + rnd() * 1.5)))
            .with(Circle::with_radius(2.))
            .with(color.clone())
            .with(Particle {
                created: now,
                lifetime: 0.5 + rnd() * 1.,
            })
            .build();
    }
}
