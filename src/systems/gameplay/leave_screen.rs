use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, SystemData},
};

use crate::{
    components::{Circle, Enemy, InScreen, Shot, Transform},
    display::{HEIGHT as H, WIDTH as W},
};

#[derive(SystemDesc, Default)]
pub struct LeaveScreenSystem;

impl<'s> System<'s> for LeaveScreenSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, InScreen>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
        Read<'s, LazyUpdate>,
    );

    /// If the circle (enemy or shot) leaves the screen, kills it, but only if
    /// it has entered the screen in the past (this allows enemies being spawned
    /// outside the screen)
    fn run(
        &mut self,
        (entities, shots, enemies, in_screens, transforms, circles, lazy): Self::SystemData,
    ) {
        for (entity, _, in_screen, transform, circle) in (
            &entities,
            shots.mask() | enemies.mask(),
            (&in_screens).maybe(),
            &transforms,
            &circles,
        )
            .join()
        {
            let c = transform.0;
            let r = circle.radius;
            let outside_screen = c.x - r > W || c.x + r < 0. || c.y + r < 0. || c.y - r > H;
            if in_screen.is_some() {
                // kill if outside screen
                if outside_screen {
                    entities.delete(entity).unwrap();
                }
            } else {
                // mark if inside screen
                if !outside_screen {
                    lazy.insert(entity, InScreen);
                }
            }
        }
    }
}
