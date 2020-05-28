use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, LazyUpdate, Read, System, SystemData},
    prelude::*,
    utils::application_root_dir,
};

use log::debug;

use std::time::Duration;

use crate::{
    components::{Circle, Enemy, Moving, Transform},
    editor::reader::{lua::LuaLevel, EnemyType, FormationEvent, Level, LevelEvent},
};

/// Indicates the current state of this level execution in the state machine
enum State {
    /// Will execute the next instruction ASAP
    ReadyForInstruction,
    /// Sleeping for some amount of time before continuing
    Sleeping { until: Duration },
    /// Level execution is over
    Finished,
}

/// This system reads a level (usually from a lua file) and executes all of its
/// commands.
#[derive(SystemDesc)]
pub struct LevelExecutorSystem<L: Level> {
    level: L,
    state: State,
}

impl LevelExecutorSystem<LuaLevel> {
    pub fn from_lua(level_name: &str) -> Self {
        let level = LuaLevel::new(
            &application_root_dir()
                .unwrap()
                .join(format!("levels/{}.lua", level_name)),
        )
        .expect("Failed to load level");
        Self {
            level,
            state: State::ReadyForInstruction,
        }
    }
}

impl<'s, L: Level> System<'s> for LevelExecutorSystem<L> {
    type SystemData = (Read<'s, Time>, Entities<'s>, Read<'s, LazyUpdate>);

    fn run(&mut self, data: Self::SystemData) {
        let time = &data.0;
        loop {
            match self.state {
                // Finished -- do nothing
                State::Finished => return,
                // Sleeping -- either continue or execute the next instruction
                State::Sleeping { until } => {
                    if time.absolute_time() >= until {
                        self.state = State::ReadyForInstruction;
                    } else {
                        return;
                    }
                }
                // ReadyForInstruction - execute an instruction
                State::ReadyForInstruction => {
                    let event = self.level.next();
                    debug!("Got event: {:?}", event);
                    self.handle_level_event(event, &data);
                }
            }
        }
    }
}

impl<L: Level> LevelExecutorSystem<L> {
    fn handle_formation_event(
        &mut self,
        event: FormationEvent,
        (time, entities, lazy): &<Self as System>::SystemData,
    ) {
        match event {
            FormationEvent::Single { enemy, pos, speed } => {
                lazy.create_entity(&entities)
                    .with(Transform::from(pos))
                    .with(Circle {
                        radius: 10.,
                        color: [0.9, 0.1, 0.1],
                    })
                    .with(Moving::from(speed))
                    .with(Enemy)
                    .build();
            }
        }
    }

    fn handle_level_event(
        &mut self,
        event: Option<LevelEvent>,
        data: &<Self as System>::SystemData,
    ) {
        let time = &data.0;
        match event {
            // Last event
            None => {
                self.state = State::Finished;
            }
            // Sleep for some amount of time
            Some(LevelEvent::Wait(amount)) => {
                self.state = State::Sleeping {
                    until: Duration::from_secs_f32(amount) + time.absolute_time(),
                }
            }
            // Create a formation then execute the next event
            Some(LevelEvent::Formation(f)) => {
                self.handle_formation_event(f, data);
                self.state = State::ReadyForInstruction;
            }
        }
    }
}
