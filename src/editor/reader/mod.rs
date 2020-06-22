pub mod lua;

use std::iter::Iterator;

use crate::editor::Vec2;
use rlua::UserData;
use rlua_builders::{LuaBuilder, UserData};

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum BallEnemy {
    Simple,
    Double,
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
    V { margin: Option<f32>, spacing: f32 },
}

#[derive(Debug, Clone, Copy, UserData, LuaBuilder)]
pub enum HorizontalLinePlacement {
    Distribute { margin: Option<f32> },
    FromLeft { margin: Option<f32>, spacing: f32 },
    FromRight { margin: Option<f32>, spacing: f32 },
    V { margin: Option<f32>, spacing: f32 },
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
    Multiple {
        enemies: Vec<BallEnemy>,
        amount: u16,
        #[default = 5.]
        spacing: f32,
        pos: Vec2,
        speed: Vec2,
        #[default = 20.]
        radius: f32,
    },
    VerticalLine {
        enemies: Vec<BallEnemy>,
        amount: u8,
        #[default = 15.]
        speed: f32,
        #[default = 20.]
        radius: f32,
        side: VerticalLineSide,
        placement: VerticalLinePlacement,
    },
    HorizontalLine {
        enemies: Vec<BallEnemy>,
        amount: u8,
        #[default = 15.]
        speed: f32,
        #[default = 20.]
        radius: f32,
        side: HorizontalLineSide,
        placement: HorizontalLinePlacement,
    },
    Circle {
        enemies: Vec<BallEnemy>,
        amount: u8,
        #[default = 15.]
        speed: f32,
        #[default = 20.]
        enemy_radius: f32,
        formation_radius: Option<f32>,
        formation_center: Option<Vec2>,
    },
    Spiral {
        enemies: Vec<BallEnemy>,
        amount_in_circle: u16,
        amount: u16,
        spacing: f32,
        #[default = 20.]
        enemy_radius: f32,
        #[default = 10.]
        speed: f32,
    },
}

#[derive(Debug, Clone, UserData, LuaBuilder)]
pub enum LevelEvent {
    Wait(f32),
    WaitUntilNoEnemies(),
    Spawn(Formation),
    CustomSpawn {
        formation: Formation,
        indicator_duration: Option<f64>,
        #[default = false]
        follow_player: bool,
    },
    SetDefaultIndicatorDuration(f64),
}

pub trait Level: Iterator<Item = LevelEvent> {}
