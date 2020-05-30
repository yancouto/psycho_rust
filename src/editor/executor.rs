use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, LazyUpdate, Read, ReadStorage, System, SystemData},
    prelude::*,
    utils::application_root_dir,
};

use log::debug;

use std::time::Duration;

use crate::{
    components::{Circle, Enemy, Moving, Transform},
    display::{HEIGHT, WIDTH},
    editor::reader::{
        lua::LuaLevel, BallEnemyType, FormationEvent, Level, LevelEvent, VerticalLinePlacement,
        VerticalLineSide,
    },
};

/// Indicates the current state of this level execution in the state machine
enum State {
    /// Will execute the next instruction ASAP
    ReadyForInstruction,
    /// Sleeping for some amount of time before continuing
    Sleeping { until: Duration },
    /// Sleep while there are enemies on screen
    WaitUntilNoEnemies,
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
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, Enemy>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let time = &data.0;
        let enemies = &data.3;
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
                // NoEnemies -- continue waiting if there are enemies
                State::WaitUntilNoEnemies => {
                    if enemies.is_empty() {
                        self.state = State::ReadyForInstruction;
                    } else {
                        return;
                    }
                }
                // ReadyForInstruction - execute an instruction
                State::ReadyForInstruction => {
                    let event = self.level.next();
                    debug!("Got event: {:?}", event);
                    self.state = self.handle_level_event(event, &data);
                }
            }
        }
    }
}

impl<'s> FormationEvent {
    fn create_formation(self, lazy: &LazyUpdate, entities: &Entities<'s>) {
        match self {
            FormationEvent::Single {
                enemy,
                pos,
                speed,
                radius,
            } => {
                lazy.create_entity(&entities)
                    .with(Transform::from(pos))
                    .with(Circle {
                        radius,
                        color: [0.9, 0.1, 0.1],
                    })
                    .with(Moving::from(speed))
                    .with(Enemy)
                    .build();
            }
            FormationEvent::VerticalLine {
                enemies,
                side,
                speed,
                radius,
                amount,
                placement,
            } => {
                let (speed, x) = if side == VerticalLineSide::Left {
                    (speed, -radius)
                } else {
                    (-speed, WIDTH + radius)
                };
                match placement {
                    VerticalLinePlacement::Distribute { margin } => {
                        for i in 0..amount {
                            lazy.create_entity(&entities)
                                .with(Transform::new(
                                    x,
                                    // This needs improvements
                                    // When amount = 2, they should be on the edges, even if the radius is small
                                    // Radius has to go here somewhere.
                                    (HEIGHT - 2. * margin) / (amount as f32) * ((i as f32) + 0.5) + margin,
                                ))
                                .with(Circle {
                                    radius,
                                    color: [0.9, 0.1, 0.1],
                                })
                                .with(Moving::new(speed, 0.))
                                .with(Enemy)
                                .build();
                        }
                    }
                }
            }
        }
    }
}

impl<L: Level> LevelExecutorSystem<L> {
    fn handle_level_event(
        &mut self,
        event: Option<LevelEvent>,
        (time, entities, lazy, ..): &<Self as System>::SystemData,
    ) -> State {
        match event {
            // Last event
            None => State::Finished,
            // Sleep for some amount of time
            Some(LevelEvent::Wait(amount)) => State::Sleeping {
                until: Duration::from_secs_f32(amount) + time.absolute_time(),
            },
            // Sleep until no enemies are on screen
            Some(LevelEvent::WaitUntilNoEnemies) => State::WaitUntilNoEnemies,
            // Create a formation then execute the next event
            Some(LevelEvent::Formation(f)) => {
                f.create_formation(lazy, entities);
                State::ReadyForInstruction
            }
        }
    }
}
