//! Quickplay state

use amethyst::{
    core::ArcThreadPool,
    ecs::{Dispatcher, DispatcherBuilder, Entities, Entity, Join, ReadStorage},
    prelude::*,
    winit::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};
use log::info;

use crate::{
    components::{Circle, Color, Player, Transform, Triangle},
    display::{HEIGHT as H, WIDTH as W},
    editor::executor::LevelExecutorSystem,
    states::MainMenu,
    systems::{
        gameplay::{CollisionSystem, EnemySpawnerSystem, LeaveScreenSystem},
        particles::FadeSystem,
        player::{CollisionSystem as PlayerCollisionSystem, MoveSystem, ShootSystem},
    },
};

pub struct Quickplay<'a, 'b> {
    level_name: String,
    dispatcher: Option<Dispatcher<'a, 'b>>,
}

impl<'a, 'b> Quickplay<'a, 'b> {
    pub fn new(level_name: String) -> Self {
        Self {
            level_name,
            dispatcher: None,
        }
    }
}

impl<'a, 'b> Quickplay<'a, 'b> {
    fn initialize_balls(&mut self, world: &mut World) {
        world
            .create_entity()
            .with(Circle::with_radius(24.))
            .with(Color::rgb(0.3, 0.4, 1.))
            .with(Transform::new(W / 2., H / 2.))
            .with(Player)
            .build();
    }
}

impl<'a, 'b> SimpleState for Quickplay<'a, 'b> {
    fn on_start(&mut self, data: StateData<GameData>) {
        info!("Started quickplay on level {}!", self.level_name);
        let mut dispatch = DispatcherBuilder::new()
            .with_pool((*data.world.read_resource::<ArcThreadPool>()).clone())
            .with(MoveSystem::default(), "player_move", &[])
            .with_barrier()
            .with(
                LevelExecutorSystem::from_lua(&self.level_name),
                "level_exec",
                &[],
            )
            .with(LeaveScreenSystem::default(), "leave_screen", &[])
            .with(CollisionSystem::default(), "collision", &["leave_screen"])
            .with(
                PlayerCollisionSystem::default(),
                "player_collision",
                &["player_move"],
            )
            .with(
                ShootSystem::default(),
                "player_shoot",
                &["player_collision"],
            )
            .with(FadeSystem::default(), "particle_fade", &[])
            .with(EnemySpawnerSystem::default(), "enemy_spawner", &[])
            .build();
        let world = data.world;
        dispatch.setup(world);
        self.initialize_balls(world);
        self.dispatcher = Some(dispatch);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // Delete all circles
        let (entities, circles): (Entities, ReadStorage<'_, Circle>) = data.world.system_data();
        for (c_id, circle) in (&entities, &circles).join() {
            entities.delete(c_id).unwrap();
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
