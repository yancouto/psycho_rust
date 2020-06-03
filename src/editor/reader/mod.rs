pub mod lua;

use amethyst::core::math::{Point2, Vector2};
use std::iter::Iterator;

use rlua::UserData;
use rlua_builders::LuaBuilder;
use rlua_builders_derive::{UserData, LuaBuilder};
use lua::Vec2;

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum BallEnemy {
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, UserData, LuaBuilder)]
pub enum VerticalLineSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum VerticalLinePlacement {
    Distribute { margin: Option<f32> },
    FromBottom { margin: Option<f32>, spacing: f32 },
    FromTop { margin: Option<f32>, spacing: f32 },
}

#[derive(Debug, Clone, UserData, LuaBuilder)]
pub enum Formation {
    Single {
        enemy: BallEnemy,
        pos: Vec2,
        speed: Vec2,
        radius: Option<f32>,
    },
    VerticalLine {
        enemies: Vec<BallEnemy>,
        speed: Option<f32>,
        radius: Option<f32>,
        side: VerticalLineSide,
        amount: u8,
        placement: VerticalLinePlacement,
    },
}

#[derive(Debug, Clone, UserData, LuaBuilder)]
pub enum LevelEvent {
    Wait(f32),
    WaitUntilNoEnemies(),
    Spawn(Formation),
}

pub trait Level: Iterator<Item = LevelEvent> {}
