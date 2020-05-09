//! Quickplay state

use amethyst::{core::transform::Transform, prelude::*, renderer::Camera};

use crate::circle_drawer::Circle;

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

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

fn initialise_camera(world: &mut World) {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}

fn initialize_player(world: &mut World) {
    let transform = Transform::from_xy(0.1, 0.1);

    world
        .create_entity()
        .with(Circle { radius: 0.5 })
        .with(transform)
        .build();
}

pub struct Quickplay;

impl SimpleState for Quickplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Started!");
        let world = data.world;
        initialize_player(world);
        initialise_camera(world);
    }
}
