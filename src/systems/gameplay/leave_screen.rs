use amethyst::{
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

use crate::{
    components::{Circle, Enemy, Shot, Transform},
    display::{HEIGHT as H, WIDTH as W},
};

#[derive(SystemDesc, Default)]
pub struct LeaveScreenSystem;

impl<'s> System<'s> for LeaveScreenSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Shot>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Circle>,
    );

    fn run(&mut self, (entities, shots, enemies, transforms, circles): Self::SystemData) {
        for (entity, _, transform, circle) in (
            &entities,
            shots.mask() | enemies.mask(),
            &transforms,
            &circles,
        )
            .join()
        {
            let c = transform.0;
            let r = circle.radius;
            if c.x - r > W || c.x + r < 0. || c.y + r < 0. || c.y - r > H {
                println!("Deleting {:?}", entity);
                entities.delete(entity).unwrap();
            }
        }
    }
}
