pub mod lua;

use amethyst::core::math::{Point2, Vector2};
use std::iter::Iterator;

#[derive(Debug, Clone)]
pub enum EnemyType {
    SimpleBall,
}

#[derive(Debug, Clone)]
pub enum FormationEvent {
    Single {
        enemy: EnemyType,
        pos: Point2<f32>,
        speed: Vector2<f32>,
    },
}

#[derive(Debug, Clone)]
pub enum LevelEvent {
    Wait(f32),
    Formation(FormationEvent),
}
pub trait Level: Iterator<Item = LevelEvent> {}
