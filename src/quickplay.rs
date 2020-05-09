//! Quickplay state

use amethyst::{
    core::transform::Transform,
    prelude::*,
};

use crate::circle_drawer::Circle;

trait Transform2D {
    fn set_translation_xy(&mut self, x: f32, y: f32) -> &mut Self;
    fn from_xy(x: f32, y: f32) -> Self;
}

impl Transform2D for Transform {
    fn set_translation_xy(&mut self, x: f32, y: f32) -> &mut Self {
        self.set_translation_xyz(x, y, 0.)
    }

    fn from_xy(x: f32, y: f32) -> Self {
        let mut t = Self::default();
        t.set_translation_xy(x, y);
        t
    }
}

fn initialize_balls(world: &mut World) {
    world
        .create_entity()
        .with(Circle { radius: 25. })
        .with(Transform::from_xy(75., 25.))
        .build();
    world
        .create_entity()
        .with(Circle { radius: 25. })
        .with(Transform::from_xy(25., 75.))
        .build();
    world
        .create_entity()
        .with(Circle { radius: 25. })
        .with(Transform::from_xy(75., 75.))
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
