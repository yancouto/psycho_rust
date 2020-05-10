//! Quickplay state

use amethyst::prelude::*;
use log::info;

use crate::{
    components::{Circle, Moving, Player, Transform},
    display::{HEIGHT as H, WIDTH as W},
};

fn initialize_balls(world: &mut World) {
    world
        .create_entity()
        .with(Circle {
            radius: 100.,
            color: [0.3, 0.4, 1.],
        })
        .with(Transform::new(W / 2., H / 2.))
        .with(Player)
        .build();
    world
        .create_entity()
        .with(Circle {
            radius: 10.,
            color: [0.8, 0.2, 0.],
        })
        .with(Transform::new(10., 10.))
        .with(Moving::new(10., 0.))
        .build();
}

pub struct Quickplay;

impl SimpleState for Quickplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Started quickplay!");
        let world = data.world;
        initialize_balls(world);
    }
}
