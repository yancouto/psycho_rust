//! Quickplay state

use amethyst::{
    core::ArcThreadPool,
    ecs::{world::EntitiesRes, Dispatcher, DispatcherBuilder, Entity},
    prelude::*,
    winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};
use log::info;

use crate::{
    components::{Circle, Player, Transform},
    display::{HEIGHT as H, WIDTH as W},
    editor::executor::LevelExecutorSystem,
    states::MainMenu,
    systems::{
        gameplay::{CollisionSystem, LeaveScreenSystem},
        player::{MoveSystem, ShootSystem},
    },
};

pub struct Quickplay<'a, 'b> {
    level_name: String,
    dispatcher: Option<Dispatcher<'a, 'b>>,
    player: Option<Entity>,
}

impl<'a, 'b> Quickplay<'a, 'b> {
    pub fn new(level_name: String) -> Self {
        Self {
            level_name,
            dispatcher: None,
            player: None,
        }
    }
}

impl<'a, 'b> Quickplay<'a, 'b> {
    fn initialize_balls(&mut self, world: &mut World) {
        self.player = Some(
            world
                .create_entity()
                .with(Circle {
                    radius: 30.,
                    color: [0.3, 0.4, 1.],
                })
                .with(Transform::new(W / 2., H / 2.))
                .with(Player)
                .build(),
        );
    }
}

impl<'a, 'b> SimpleState for Quickplay<'a, 'b> {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("Started quickplay on level {}!", self.level_name);
        let mut builder = DispatcherBuilder::new();
        builder.add(
            LevelExecutorSystem::from_lua(&self.level_name),
            "level_exec",
            &[],
        );
        builder.add(ShootSystem::default(), "shoot", &[]);
        builder.add(CollisionSystem::default(), "collision", &[]);
        builder.add(LeaveScreenSystem::default(), "leave_screen", &[]);
        builder.add(MoveSystem, "player_move", &[]);
        let mut dispatch = builder
            .with_pool((*data.world.read_resource::<ArcThreadPool>()).clone())
            .build();
        let world = data.world;
        dispatch.setup(world);
        self.initialize_balls(world);
        self.dispatcher = Some(dispatch);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        if let Some(player) = self.player {
            let es = data.world.fetch::<EntitiesRes>();
            es.delete(player).unwrap();
        }
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(dispatcher) = &mut self.dispatcher {
            dispatcher.dispatch(&data.world);
        }
        Trans::None
    }

    fn handle_event(&mut self, _data: StateData<GameData>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(Event::WindowEvent {
            event:
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                },
            ..
        }) = event
        {
            Trans::Switch(Box::new(MainMenu::default()))
        } else {
            Trans::None
        }
    }
}
