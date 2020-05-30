pub mod lua;

use amethyst::core::math::{Point2, Vector2};
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub enum BallEnemyType {
    Simple,
}

#[derive(Debug, Clone)]
pub enum FormationEvent {
    Single {
        enemy: BallEnemyType,
        pos: Point2<f32>,
        speed: Vector2<f32>,
        radius: Option<f32>,
    },
}

#[derive(Debug, Clone)]
pub enum LevelEvent {
    Wait(f32),
    WaitUntilNoEnemies,
    Formation(FormationEvent),
}
pub trait Level: Iterator<Item = LevelEvent> {}
