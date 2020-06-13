use amethyst::{
    core::math::{Point2, RealField, Rotation2, Vector2},
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Entities, LazyUpdate, Read, ReadStorage, System, SystemData},
    prelude::*,
    utils::application_root_dir,
};

use log::debug;

use std::time::Duration;

use crate::{
    components::{Circle, Color, Enemy, Moving, Transform},
    display::{HEIGHT, WIDTH},
    editor::reader::{
        lua::LuaLevel, Formation, HorizontalLinePlacement, HorizontalLineSide, Level, LevelEvent,
        VerticalLinePlacement, VerticalLineSide,
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
                    // Exit. It may be the case that an enemy was created lazily so we
                    // need to wait for the next iteration to be sure.
                    if matches!(self.state, State::WaitUntilNoEnemies) {
                        return;
                    }
                }
            }
        }
    }
}

/// Returns positions for the center of the circles when placing `amount` enemies on a
/// line of size `width`, using `placement` to decide how to place them.
fn line_enemy_positions(
    radius: f32,
    amount: u8,
    width: f32,
    placement: HorizontalLinePlacement,
) -> Vec<f32> {
    let mut ret = Vec::with_capacity(amount as usize);
    if amount == 0 {
        return ret;
    }
    match placement {
        HorizontalLinePlacement::Distribute { margin } => {
            if amount == 1 {
                return vec![width / 2.];
            }
            let margin = margin.unwrap_or(0.);
            ret.push(margin + radius);
            for i in 0..(amount - 2) {
                // Advanced maths to distance balls properly
                let w = width - 2. * margin - 4. * radius;
                let (n, d, i) = ((amount - 2) as f32, 2. * radius, i as f32);
                ret.push((w - d * n) / (n + 1.) * (i + 1.) + (i + 1.5) * d + margin);
            }
            ret.push(width - margin - radius);
        }
        HorizontalLinePlacement::FromLeft { margin, spacing } => {
            let margin = margin.unwrap_or(0.);
            for i in 0..amount {
                ret.push(margin + (i as f32) * (spacing + 2. * radius) + radius);
            }
        }
        HorizontalLinePlacement::FromRight { margin, spacing } => {
            let margin = margin.unwrap_or(0.);
            for i in 0..amount {
                ret.push(width - (margin + (i as f32) * (spacing + 2. * radius) + radius));
            }
        }
    }
    ret
}

#[test]
fn test_enemy_positions() {
    assert_eq!(
        line_enemy_positions(
            10.,
            2,
            100.,
            HorizontalLinePlacement::Distribute { margin: Some(10.) }
        ),
        vec![20., 80.],
    )
}

impl From<VerticalLinePlacement> for HorizontalLinePlacement {
    fn from(p: VerticalLinePlacement) -> Self {
        match p {
            VerticalLinePlacement::Distribute { margin } => {
                HorizontalLinePlacement::Distribute { margin }
            }
            VerticalLinePlacement::FromTop { margin, spacing } => {
                HorizontalLinePlacement::FromLeft { margin, spacing }
            }
            VerticalLinePlacement::FromBottom { margin, spacing } => {
                HorizontalLinePlacement::FromRight { margin, spacing }
            }
        }
    }
}

impl<'s> Formation {
    fn create_formation(self, lazy: &LazyUpdate, entities: &Entities<'s>) {
        match self {
            Formation::Single {
                enemy: _,
                pos,
                speed,
                radius,
            } => {
                lazy.create_entity(&entities)
                    .with(Transform::from(pos))
                    .with(Circle::with_radius(radius))
                    .with(Color::rgb(0.9, 0.1, 0.1))
                    .with(Moving::from(speed))
                    .with(Enemy)
                    .build();
            }
            Formation::VerticalLine {
                enemies: _,
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
                for y in line_enemy_positions(radius, amount, HEIGHT, placement.into()) {
                    lazy.create_entity(&entities)
                        .with(Transform::new(x, y))
                        .with(Moving::new(speed, 0.))
                        .with(Circle::with_radius(radius))
                        .with(Color::rgb(0.9, 0.1, 0.1))
                        .with(Enemy)
                        .build();
                }
            }
            Formation::HorizontalLine {
                enemies: _,
                side,
                speed,
                radius,
                amount,
                placement,
            } => {
                let (speed, y) = if side == HorizontalLineSide::Top {
                    (speed, -radius)
                } else {
                    (-speed, HEIGHT + radius)
                };
                for x in line_enemy_positions(radius, amount, WIDTH, placement) {
                    lazy.create_entity(&entities)
                        .with(Transform::new(x, y))
                        .with(Moving::new(0., speed))
                        .with(Circle::with_radius(radius))
                        .with(Color::rgb(0.9, 0.1, 0.1))
                        .with(Enemy)
                        .build();
                }
            }
            Formation::Circle {
                enemies: _,
                amount,
                speed,
                enemy_radius,
                formation_radius,
                formation_center,
            } => {
                if formation_center.is_some() && formation_radius.is_none() {
                    // TODO(#2): Not panic maybe?
                    panic!("Radius must be specified if center is");
                }
                let center: Point2<f32> = formation_center
                    .map(Point2::<f32>::from)
                    .unwrap_or(Point2::new(WIDTH / 2., HEIGHT / 2.));
                let r = enemy_radius;
                let R = formation_radius
                    .unwrap_or_else(|| (WIDTH * WIDTH + HEIGHT * HEIGHT).sqrt() / 2. + r);
                for i in 0..amount {
                    let unit = Rotation2::new(f32::two_pi() / (amount as f32) * (i as f32))
                        * Vector2::new(0., -1.);
                    lazy.create_entity(&entities)
                        .with(Transform::from(center + unit * R))
                        .with(Moving::from(-unit * speed))
                        .with(Circle::with_radius(enemy_radius))
                        .with(Color::rgb(0.9, 0.1, 0.1))
                        .with(Enemy)
                        .build();
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
            Some(LevelEvent::WaitUntilNoEnemies()) => State::WaitUntilNoEnemies,
            // Create a formation then execute the next event
            Some(LevelEvent::Spawn(f)) => {
                f.create_formation(lazy, entities);
                State::ReadyForInstruction
            }
        }
    }
}
