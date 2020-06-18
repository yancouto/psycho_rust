use amethyst::{
    core::math::{Point2, RealField, Rotation2, Vector2},
    core::timing::Time,
    derive::SystemDesc,
    ecs::{world::Builder, Component, Entities, LazyUpdate, Read, ReadStorage, System, SystemData},
    prelude::*,
    utils::application_root_dir,
};

use log::debug;

use std::time::Duration;

use crate::{
    components::{Circle, Color, Moving, Transform},
    display::{HEIGHT, WIDTH},
    editor::{
        reader::{
            lua::LuaLevel, BallEnemy, Formation, HorizontalLinePlacement, HorizontalLineSide,
            Level, LevelEvent, VerticalLinePlacement, VerticalLineSide,
        },
        Vec2,
    },
    utils::creator::LazyCreator,
};

impl BallEnemy {
    pub fn default_color(&self) -> Color {
        match self {
            BallEnemy::Simple => Color::rgb(0.1, 0.1, 0.9),
            BallEnemy::Double => Color::rgb(0.95, 0.3, 0.1),
        }
    }
}

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
        ReadStorage<'s, BallEnemy>,
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

impl<'s> Formation {
    fn create_formation(self, mut creator: LazyCreator) {
        let mut create_enemy = |enemy: BallEnemy| {
            creator
                .create_entity()
                .with(enemy)
                .with(enemy.default_color())
        };
        match self {
            Formation::Single {
                enemy,
                pos,
                speed,
                radius,
            } => {
                create_enemy(enemy)
                    .with(Transform::from(pos))
                    .with(Circle::with_radius(radius))
                    .with(Moving::from(speed))
                    .build();
            }
            Formation::Multiple {
                enemies,
                amount,
                spacing,
                pos,
                speed,
                radius,
            } => {
                let mut enemies = enemies.into_iter().cycle();
                let dir = Vector2::from(speed).normalize();
                let pos = Into::<Point2<f32>>::into(pos);
                for i in 0..amount {
                    create_enemy(enemies.next().unwrap())
                        .with(Transform::from(
                            pos - dir * (i as f32) * (spacing + 2. * radius),
                        ))
                        .with(Circle::with_radius(radius))
                        .with(Moving::from(speed))
                        .build();
                }
            }
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
                for (y, x) in line_enemy_positions(x, radius, amount, HEIGHT, placement.into()) {
                    create_enemy(enemies.next().unwrap())
                        .with(Transform::new(x, y))
                        .with(Moving::new(speed, 0.))
                        .with(Circle::with_radius(radius))
                        .build();
                }
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
                for (x, y) in line_enemy_positions(y, radius, amount, WIDTH, placement) {
                    create_enemy(enemies.next().unwrap())
                        .with(Transform::new(x, y))
                        .with(Moving::new(0., speed))
                        .with(Circle::with_radius(radius))
                        .build();
                }
            }
            Formation::Circle {
                enemies,
                amount,
                speed,
                enemy_radius,
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
                for i in 0..amount {
                    let unit = Rotation2::new(f32::two_pi() / (amount as f32) * (i as f32))
                        * Vector2::new(0., -1.);
                    create_enemy(enemies.next().unwrap())
                        .with(Transform::from(center + unit * R))
                        .with(Moving::from(-unit * speed))
                        .with(Circle::with_radius(enemy_radius))
                        .build();
                }
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
                for i in 0..amount {
                    let unit =
                        Rotation2::new(f32::two_pi() / (amount_in_circle as f32) * (i as f32))
                            * Vector2::new(1., 0.);
                    create_enemy(enemies.next().unwrap())
                        .with(Transform::from(center + unit * (R + (i as f32) * spacing)))
                        .with(Moving::from(-unit * speed))
                        .with(Circle::with_radius(enemy_radius))
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
                f.create_formation(LazyCreator { lazy, entities });
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
        register!(Transform, Circle, Color, Moving, BallEnemy -> world);
        world
    }

    #[test]
    fn test_create_formation() {
        let mut world = get_world();
        Formation::Single {
            enemy: BallEnemy::Simple,
            pos: Vec2(0., 0.),
            speed: Vec2(10., 0.),
            radius: 10.,
        }
        .create_formation(LazyCreator {
            lazy: &world.fetch(),
            entities: &world.fetch(),
        });
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
