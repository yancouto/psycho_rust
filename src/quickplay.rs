//! Quickplay state

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
};

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

pub struct Circle {
    pub radius: f32,
}

impl Default for Circle {
    fn default() -> Self {
        Self { radius: 10. }
    }
}

impl Component for Circle {
    type Storage = DenseVecStorage<Self>;
}

fn initialize_player(world: &mut World) {
    let transform = Transform::from_xy(50., 50.);

    world
        .create_entity()
        .with(Circle::default())
        .with(transform)
        .build();
}

pub struct Quickplay;

impl SimpleState for Quickplay {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Started!");
        let world = data.world;
        world.register::<Circle>();
        initialize_player(world);
    }
}
