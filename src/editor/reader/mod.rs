pub mod lua;

use amethyst::core::math::{Point2, Vector2};
use std::iter::Iterator;

#[derive(Debug, Clone, Copy)]
pub enum BallEnemyType {
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalLineSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum VerticalLinePlacement {
    Distribute { margin: f32 },
    FromBottom { margin: f32, spacing: f32 },
    FromTop { margin: f32, spacing: f32 },
}

#[derive(Debug, Clone)]
pub enum FormationEvent {
    Single {
        enemy: BallEnemyType,
        pos: Point2<f32>,
        speed: Vector2<f32>,
        radius: f32,
    },
    VerticalLine {
        enemies: Vec<BallEnemyType>,
        speed: f32,
        radius: f32,
        side: VerticalLineSide,
        amount: u8,
        placement: VerticalLinePlacement,
    },
}

#[derive(Debug, Clone)]
pub enum LevelEvent {
    Wait(f32),
    WaitUntilNoEnemies,
    Formation(FormationEvent),
}
pub trait Level: Iterator<Item = LevelEvent> {}
