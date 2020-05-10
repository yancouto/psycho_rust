//! Quickplay state

use amethyst::{
    ecs::{Component, DenseVecStorage},
    prelude::*,
};

use crate::circle_drawer::Circle;
use crate::transform::Transform;

use crate::screen::{HEIGHT as H, WIDTH as W};

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_balls(world: &mut World) {
    world
        .create_entity()
        .with(Circle { radius: 100. })
        .with(Transform::new(W / 2., H / 2.))
        .with(Player)
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
