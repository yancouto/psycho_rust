//! Quickplay state

use amethyst::prelude::*;

use crate::circle_drawer::Circle;
use crate::transform::Transform;

use crate::screen::{WIDTH as W, HEIGHT as H};

fn initialize_balls(world: &mut World) {
    world
        .create_entity()
        .with(Circle { radius: 100. })
        .with(Transform::new(W / 2., H / 2.))
        .build();
}

pub struct Quickplay;

impl SimpleState for Quickplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Started!");
        let world = data.world;
        initialize_balls(world);
    }
}
