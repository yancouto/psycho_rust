//! Quickplay state

use amethyst::{
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder},
    prelude::*,
};
use log::info;

use crate::{
    components::{Circle, Player, Transform},
    display::{HEIGHT as H, WIDTH as W},
    systems::quickplay::EnemySpawnSystem,
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
}

#[derive(Default)]
pub struct Quickplay<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> SimpleState for Quickplay<'a, 'b> {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        info!("Started quickplay!");
        let mut builder = DispatcherBuilder::new();
        builder.add(EnemySpawnSystem::default(), "quickplay_spawn", &[]);
        let mut dispatch = builder
            .with_pool((*data.world.read_resource::<ArcThreadPool>()).clone())
            .build();
        let world = data.world;
        dispatch.setup(world);
        initialize_balls(world);
        self.dispatcher = Some(dispatch);
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }
}
