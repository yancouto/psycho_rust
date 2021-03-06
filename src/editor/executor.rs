use amethyst::{
    core::math::{Point2, RealField, Rotation2, Vector2},
    core::timing::Time,
    derive::SystemDesc,
    ecs::{world::Builder, Component, Entities, LazyUpdate, Read, ReadStorage, System, SystemData},
    prelude::*,
};

use log::debug;

use std::time::Duration;

use crate::{
    components::{
        enemy_spawner::{EnemySpawner, EnemySpawnerLogic, SpawnSpeed},
        Circle, Color, Moving, Transform, Triangle,
    },
    display::{HEIGHT, WIDTH},
    editor::{
        reader::{
            lua::LuaLevel, BallEnemy, Formation, HorizontalLinePlacement, HorizontalLineSide,
            Level, LevelEvent, VerticalLinePlacement, VerticalLineSide,
        },
        Vec2,
    },
    systems::player::movement::PlayerPosition,
    utils::{creator::LazyCreator, fs::root},
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
    indicator_duration: f64,
}

impl LevelExecutorSystem<LuaLevel> {
    pub fn from_lua(level_name: &str) -> Self {
        let level = LuaLevel::new(&root().join(format!("levels/{}.lua", level_name)))
            .expect("Failed to load level");
        Self {
            level,
            state: State::ReadyForInstruction,
            indicator_duration: 1.,
        }
    }
}

impl<'s, L: Level> System<'s> for LevelExecutorSystem<L> {
    type SystemData = (
        Read<'s, Time>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadStorage<'s, BallEnemy>,
        ReadStorage<'s, EnemySpawner>,
        Read<'s, PlayerPosition>,
    );

    fn setup(&mut self, world: &mut World) {
        world.register::<EnemySpawner>();
    }

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
                // NoEnemies -- continue waiting if there are enemies
                State::WaitUntilNoEnemies => {
                    let enemies = &data.3;
                    let spawners = &data.4;
                    if enemies.is_empty() && spawners.is_empty() {
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
    base_y: f32,
    radius: f32,
    amount: u8,
    width: f32,
    placement: HorizontalLinePlacement,
) -> Vec<(f32, f32)> {
    let mut ret = Vec::with_capacity(amount as usize);
    if amount == 0 {
        return ret;
    }
    let distribute = |margin| -> Vec<f32> {
        if amount == 1 {
            return vec![width / 2.];
        }
        let mut v = Vec::with_capacity(amount as usize);
        v.push(margin + radius);
        for i in 0..(amount - 2) {
            // Advanced maths to distance balls properly
            let w = width - 2. * margin - 4. * radius;
            let (n, d, i) = ((amount - 2) as f32, 2. * radius, i as f32);
            v.push((w - d * n) / (n + 1.) * (i + 1.) + (i + 1.5) * d + margin);
        }
        v.push(width - margin - radius);
        v
    };
    match placement {
        HorizontalLinePlacement::Distribute { margin } => {
            for x in distribute(margin.unwrap_or(0.)).into_iter() {
                ret.push((x, base_y));
            }
        }
        HorizontalLinePlacement::FromLeft { margin, spacing } => {
            let margin = margin.unwrap_or(0.);
            for i in 0..amount {
                ret.push((
                    margin + (i as f32) * (spacing + 2. * radius) + radius,
                    base_y,
                ));
            }
        }
        HorizontalLinePlacement::FromRight { margin, spacing } => {
            let margin = margin.unwrap_or(0.);
            for i in 0..amount {
                ret.push((
                    width - (margin + (i as f32) * (spacing + 2. * radius) + radius),
                    base_y,
                ));
            }
        }
        HorizontalLinePlacement::V { margin, spacing } => {
            if amount % 2 == 0 {
                panic!("With V formation, amount should be odd!");
            }
            for (i, x) in distribute(margin.unwrap_or(0.)).into_iter().enumerate() {
                ret.push((
                    x,
                    base_y - ((amount / 2) as i8 - (i as i8)).abs() as f32 * spacing,
                ));
            }
        }
    }
    ret
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
            VerticalLinePlacement::V { margin, spacing } => {
                HorizontalLinePlacement::V { margin, spacing }
            }
        }
    }
}

#[derive(Debug)]
struct SingleSpawnerLogic {
    enemy: BallEnemy,
    radius: f32,
}

impl EnemySpawnerLogic for SingleSpawnerLogic {
    fn do_spawn(&self, creator: &LazyCreator, pos: Point2<f32>, speed: Vector2<f32>) {
        creator.create_enemy(
            self.enemy,
            Circle::with_radius(self.radius),
            Transform::from(pos),
            Moving::from(speed),
        );
    }
}

#[derive(Debug)]
struct MultipleSpawnerLogic {
    enemies: Vec<BallEnemy>,
    amount: u16,
    spacing: f32,
    radius: f32,
}

impl EnemySpawnerLogic for MultipleSpawnerLogic {
    fn do_spawn(&self, creator: &LazyCreator, pos: Point2<f32>, speed: Vector2<f32>) {
        let mut enemies = self.enemies.iter().cycle();
        let dir = Vector2::from(speed).normalize();
        for i in 0..self.amount {
            creator.create_enemy(
                enemies.next().unwrap().clone(),
                Circle::with_radius(self.radius),
                Transform::from(pos - dir * (i as f32) * (self.spacing + 2. * self.radius)),
                Moving::from(speed),
            );
        }
    }
}

impl<'s> Formation {
    fn get_spawners(self) -> Vec<EnemySpawner> {
        match self {
            Formation::Single {
                enemy,
                pos,
                speed,
                radius,
            } => vec![EnemySpawner {
                position: pos.into(),
                spawn_speed: SpawnSpeed::Fixed(speed.into()),
                logic: Box::new(SingleSpawnerLogic { enemy, radius }),
                spawn_at: 0.,
            }],
            Formation::Multiple {
                enemies,
                amount,
                spacing,
                pos,
                speed,
                radius,
            } => vec![EnemySpawner {
                position: pos.into(),
                spawn_speed: SpawnSpeed::Fixed(speed.into()),
                logic: Box::new(MultipleSpawnerLogic {
                    enemies,
                    amount,
                    spacing,
                    radius,
                }),
                spawn_at: 0.,
            }],
            Formation::VerticalLine {
                enemies,
                side,
                speed,
                radius,
                amount,
                placement,
            } => {
                let mut enemies = enemies.into_iter().cycle();
                let (speed, x) = if side == VerticalLineSide::Left {
                    (speed, -radius)
                } else {
                    (-speed, WIDTH + radius)
                };
                line_enemy_positions(x, radius, amount, HEIGHT, placement.into())
                    .into_iter()
                    .map(|(y, x)| EnemySpawner {
                        position: Point2::new(x, y),
                        spawn_speed: SpawnSpeed::Fixed(Vector2::new(speed, 0.)),
                        logic: Box::new(SingleSpawnerLogic {
                            enemy: enemies.next().unwrap(),
                            radius,
                        }),
                        spawn_at: 0.,
                    })
                    .collect()
            }
            Formation::HorizontalLine {
                enemies,
                side,
                speed,
                radius,
                amount,
                placement,
            } => {
                let mut enemies = enemies.into_iter().cycle();
                let (speed, y) = if side == HorizontalLineSide::Top {
                    (speed, -radius)
                } else {
                    (-speed, HEIGHT + radius)
                };
                line_enemy_positions(y, radius, amount, WIDTH, placement)
                    .into_iter()
                    .map(|(x, y)| EnemySpawner {
                        position: Point2::new(x, y),
                        spawn_speed: SpawnSpeed::Fixed(Vector2::new(0., speed)),
                        logic: Box::new(SingleSpawnerLogic {
                            enemy: enemies.next().unwrap(),
                            radius,
                        }),
                        spawn_at: 0.,
                    })
                    .collect()
            }
            Formation::Circle {
                enemies,
                amount,
                speed,
                enemy_radius,
                starting_angle,
                formation_radius,
                formation_center,
            } => {
                let mut enemies = enemies.into_iter().cycle();
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
                (0..amount)
                    .map(|i| {
                        let unit = Rotation2::new(
                            starting_angle + f32::two_pi() / (amount as f32) * (i as f32),
                        ) * Vector2::new(0., -1.);
                        EnemySpawner {
                            position: center + unit * R,
                            spawn_speed: SpawnSpeed::Fixed(-unit * speed),
                            logic: Box::new(SingleSpawnerLogic {
                                enemy: enemies.next().unwrap(),
                                radius: enemy_radius,
                            }),
                            spawn_at: 0.,
                        }
                    })
                    .collect()
            }
            Formation::Spiral {
                enemies,
                amount_in_circle,
                amount,
                spacing,
                speed,
                enemy_radius,
            } => {
                let mut enemies = enemies.into_iter().cycle();
                let center = Point2::new(WIDTH / 2., HEIGHT / 2.);
                let r = enemy_radius;
                let R = (WIDTH * WIDTH + HEIGHT * HEIGHT).sqrt() / 2. + r;
                (0..amount)
                    .map(|i| {
                        let unit =
                            Rotation2::new(f32::two_pi() / (amount_in_circle as f32) * (i as f32))
                                * Vector2::new(1., 0.);
                        EnemySpawner {
                            position: center + unit * (R + (i as f32) * spacing),
                            spawn_speed: SpawnSpeed::Fixed(-unit * speed),
                            logic: Box::new(SingleSpawnerLogic {
                                enemy: enemies.next().unwrap(),
                                radius: enemy_radius,
                            }),
                            spawn_at: 0.,
                        }
                    })
                    .collect()
            }
        }
    }
}

impl<L: Level> LevelExecutorSystem<L> {
    fn handle_level_event(
        &mut self,
        event: Option<LevelEvent>,
        data: &<Self as System>::SystemData,
    ) -> State {
        let (time, entities, lazy, ..) = data;
        match event {
            // Last event
            None => State::Finished,
            // Sleep for some amount of time
            Some(LevelEvent::Wait(amount)) => State::Sleeping {
                until: Duration::from_secs_f32(amount) + time.absolute_time(),
            },
            // Sleep until no enemies are on screen
            Some(LevelEvent::WaitUntilNoEnemies()) => State::WaitUntilNoEnemies,
            // Change default indicator duration
            Some(LevelEvent::SetDefaultIndicatorDuration(duration)) => {
                self.indicator_duration = duration;
                State::ReadyForInstruction
            }
            // Create a formation using the default indicator duration
            Some(LevelEvent::Spawn(formation)) => self.handle_level_event(
                Some(LevelEvent::CustomSpawn {
                    formation,
                    indicator_duration: None,
                    follow_player: false,
                }),
                data,
            ),
            // Create a formation then execute the next event
            Some(LevelEvent::CustomSpawn {
                formation,
                indicator_duration,
                follow_player,
            }) => {
                let duration = indicator_duration.unwrap_or(self.indicator_duration);
                let creator = LazyCreator { lazy, entities };
                for mut spawner in formation.get_spawners() {
                    if follow_player {
                        spawner.spawn_speed = SpawnSpeed::AimAtPlayer {
                            speed: spawner.calc_speed(Point2::new(0., 0.)).norm(),
                        };
                    }
                    if duration <= 0. {
                        let player_pos: &PlayerPosition = &data.5;
                        spawner.do_spawn(&creator, player_pos.0);
                    } else {
                        creator
                            .create_entity()
                            .with(EnemySpawner {
                                spawn_at: time.absolute_time_seconds() + duration,
                                ..spawner
                            })
                            .with(Color::rgb(1., 1., 1.))
                            // This will be fixed by EnemySpawnerSystem
                            .with(Triangle::new([-1., -1.], [-1., -1.], [-1., -1.]))
                            .build();
                    }
                }
                State::ReadyForInstruction
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use amethyst::ecs::prelude::*;
    use amethyst::ecs::world::World;
    pub struct EmptyLevel;

    impl Level for EmptyLevel {}
    impl Iterator for EmptyLevel {
        type Item = LevelEvent;
        fn next(&mut self) -> Option<LevelEvent> {
            None
        }
    }

    impl LevelExecutorSystem<EmptyLevel> {
        pub fn new_test() -> Self {
            Self {
                level: EmptyLevel,
                state: State::ReadyForInstruction,
                indicator_duration: 0.,
            }
        }
        pub fn test_handle_event(&mut self, event: LevelEvent, world: &mut World) {
            self.handle_level_event(Some(event), &world.system_data());
            world.maintain();
        }
    }
    #[test]
    fn test_enemy_positions() {
        assert_eq!(
            line_enemy_positions(
                -200.,
                10.,
                2,
                100.,
                HorizontalLinePlacement::Distribute { margin: Some(10.) }
            ),
            vec![(20., -200.), (80., -200.)],
        );
        assert_eq!(
            line_enemy_positions(
                0.,
                10.,
                3,
                100.,
                HorizontalLinePlacement::V {
                    margin: Some(10.),
                    spacing: 10.
                }
            ),
            vec![(20., -10.), (50., 0.), (80., -10.)],
        );
    }

    pub fn get_world() -> World {
        let mut world = World::new();
        world.insert(Time::default());
        world.insert(PlayerPosition::default());
        register!(Transform, Circle, Color, Moving, BallEnemy, EnemySpawner, Triangle -> world);
        world
    }

    #[test]
    fn test_create_single() {
        let mut world = get_world();
        let spawners = Formation::Single {
            enemy: BallEnemy::Simple,
            pos: Vec2(0., 0.),
            speed: Vec2(10., 0.),
            radius: 10.,
        }
        .get_spawners();
        for spawner in spawners {
            spawner.do_spawn(
                &LazyCreator {
                    lazy: &world.fetch(),
                    entities: &world.fetch(),
                },
                Point2::new(0., 0.),
            );
        }
        world.maintain();
        let (ts, cs, ms, es) = (
            world.read_storage::<Transform>(),
            world.read_storage::<Circle>(),
            world.read_storage::<Moving>(),
            world.read_storage::<BallEnemy>(),
        );
        let all = (&world.entities(), &ts, &cs, &ms, &es)
            .join()
            .collect::<Vec<_>>();
        assert_eq!(all.len(), 1);
        let (_, t, c, m, e) = all[0];
        assert_eq!(t.0, Point2::new(0., 0.));
        assert_eq!(c.radius, 10.);
        assert_eq!(m.0, Vector2::new(10., 0.));
        assert_eq!(matches!(e, BallEnemy::Simple), true);
    }
}
