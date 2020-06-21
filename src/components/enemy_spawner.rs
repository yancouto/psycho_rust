use amethyst::ecs::{Component, DenseVecStorage};

use amethyst::core::math::{Point2, Vector2};

use crate::utils::creator::LazyCreator;

pub trait EnemySpawnerLogic: Send + Sync + std::fmt::Debug {
    fn do_spawn(&self, creator: &LazyCreator, pos: Point2<f32>, speed: Vector2<f32>) -> ();
}

#[derive(Debug)]
pub enum SpawnSpeed {
    Fixed(Vector2<f32>),
    AimAtPlayer { speed: f32 },
}

impl EnemySpawner {
    pub fn calc_speed(&self, player_pos: Point2<f32>) -> Vector2<f32> {
        match self.spawn_speed {
            SpawnSpeed::Fixed(s) => s,
            SpawnSpeed::AimAtPlayer { speed } => (player_pos - self.position).normalize() * speed,
        }
    }

    pub fn do_spawn(&self, creator: &LazyCreator, player_pos: Point2<f32>) {
        self.logic
            .do_spawn(creator, self.position, self.calc_speed(player_pos));
    }
}

// Component that in the future spawns some enemies on screen
// Used to display indicators
#[derive(Debug, Component)]
pub struct EnemySpawner {
    pub position: Point2<f32>,
    pub spawn_speed: SpawnSpeed,
    pub logic: Box<dyn EnemySpawnerLogic>,
    pub spawn_at: f64,
}
