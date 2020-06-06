pub mod lua;

use std::iter::Iterator;

use crate::editor::Vec2;
use rlua::UserData;
use rlua_builders::LuaBuilder;
use rlua_builders_derive::{LuaBuilder, UserData};

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum BallEnemy {
    Simple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, UserData, LuaBuilder)]
pub enum VerticalLineSide {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, UserData, LuaBuilder)]
pub enum HorizontalLineSide {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum VerticalLinePlacement {
    Distribute { margin: Option<f32> },
    FromBottom { margin: Option<f32>, spacing: f32 },
    FromTop { margin: Option<f32>, spacing: f32 },
}

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum HorizontalLinePlacement {
    Distribute { margin: Option<f32> },
    FromLeft { margin: Option<f32>, spacing: f32 },
    FromRight { margin: Option<f32>, spacing: f32 },
}

#[derive(Debug, Clone, UserData, LuaBuilder)]
pub enum Formation {
    Single {
        enemy: BallEnemy,
        pos: Vec2,
        speed: Vec2,
        #[default = 20.]
        radius: f32,
    },
    VerticalLine {
        enemies: Vec<BallEnemy>,
        #[default = 15.]
        speed: f32,
        #[default = 20.]
        radius: f32,
        side: VerticalLineSide,
        amount: u8,
        placement: VerticalLinePlacement,
    },
    HorizontalLine {
        enemies: Vec<BallEnemy>,
        #[default = 15.]
        speed: f32,
        #[default = 20.]
        radius: f32,
        side: HorizontalLineSide,
        amount: u8,
        placement: HorizontalLinePlacement,
    },
}

#[derive(Debug, Clone, UserData, LuaBuilder)]
pub enum LevelEvent {
    Wait(f32),
    WaitUntilNoEnemies(),
    Spawn(Formation),
}

pub trait Level: Iterator<Item = LevelEvent> {}
