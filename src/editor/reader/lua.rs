use crate::{
    display::{HEIGHT, WIDTH},
    editor::reader::{
        BallEnemyType, FormationEvent, Level, LevelEvent, VerticalLinePlacement, VerticalLineSide,
    },
};
use amethyst::core::math::{Point2, Vector2};
use derive_more::Display;
use failure::{bail, Error, Fail, ResultExt};
use rlua::{
    Context, Error as LuaErrorInner, Function, Lua, RegistryKey, Result as LuaResult, Table,
    Thread, ThreadStatus, UserData,
};
use std::{fmt::Display, fs, iter::Iterator, path::Path};
use rlua_builders::LuaBuilder;
use rlua_builders_derive::{UserData, LuaBuilder};
use crate::components::{Transform, Moving};

#[derive(Debug, Copy, Clone)]
pub struct Vec2(f32, f32);

impl From<Vec2> for Vector2<f32> {
    fn from(v: Vec2) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<Vec2> for Point2<f32> {
    fn from(v: Vec2) -> Self {
        Self::new(v.0, v.1)
    }
}

impl From<Vec2> for Transform {
    fn from(v: Vec2) -> Self {
        Self(v.into())
    }
}

impl From<Vec2> for Moving {
    fn from(v: Vec2) -> Self {
        Self(v.into())
    }
}

impl UserData for Vec2 {}

fn create_level_manager(ctx: Context) -> LuaResult<Table> {
    let t = ctx.create_table()?;
    t.set(
        "wait",
        ctx.create_function(|_, val: f32| Ok(LevelEvent::Wait(val)))?,
    )?;
    t.set(
        "wait_until_no_enemies",
        ctx.create_function(|_, _: ()| Ok(LevelEvent::WaitUntilNoEnemies))?,
    )?;
    t.set(
        "spawn",
        ctx.create_function(|_, f: FormationEvent| Ok(LevelEvent::Formation(f)))?,
    )?;
    ctx.load(include_str!("coroutine_wrapper.lua"))
        .eval::<Function>()?
        .call::<_, Table>(t)
}

trait TryClampForLua: Ord + Sized + Display {
    fn try_clamp(self, name: &str, min: Self, max: Self) -> Result<Self, LuaErrorInner> {
        if self < min || self > max {
            Err(LuaErrorInner::RuntimeError(format!(
                "{} must be between {} and {}",
                name, min, max
            )))
        } else {
            Ok(self)
        }
    }
}
impl<T: Ord + Sized + Display> TryClampForLua for T {}


pub struct LuaLevel {
    lua: Lua,
    level_thread: RegistryKey,
}

#[derive(Debug, Display)]
struct LuaError(LuaErrorInner);

impl Fail for LuaError {}

impl LuaLevel {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let lua = Lua::new();
        let level_thread = lua.context::<_, Result<RegistryKey, Error>>(|ctx| {
            let globals = ctx.globals();
            globals.set("LevelManager", create_level_manager(ctx)?)?;
            globals.set("BallEnemy", BallEnemyType::builder(ctx)?)?;
            globals.set("Formations", FormationEvent::builder(ctx)?)?;
            globals.set("VerticalLinePlacement", VerticalLinePlacement::builder(ctx)?)?;
            globals.set("VerticalLineSide", VerticalLineSide::builder(ctx)?)?;
            globals.set(
                "vec2",
                ctx.create_function(|_, (x, y): (f32, f32)| Ok(Vec2(x, y)))?,
            )?;
            globals.set("WIDTH", WIDTH)?;
            globals.set("HEIGHT", HEIGHT)?;

            let fun = ctx
                .load(
                    &fs::read_to_string(path)
                        .context(format!("Failed to read level {}", path.display()))?,
                )
                .into_function()?;

            Ok(ctx.create_registry_value(ctx.create_thread(fun)?)?)
        })?;
        Ok(Self { lua, level_thread })
    }
}

impl Iterator for LuaLevel {
    type Item = LevelEvent;

    fn next(&mut self) -> Option<Self::Item> {
        self.lua
            .context::<_, Result<Option<Self::Item>, Error>>(|ctx| {
                let thread = ctx.registry_value::<Thread>(&self.level_thread)?;
                match thread.status() {
                    // We need option here since the last return value will be "nil"
                    // and not a LevelEvent. We're trusting no intermediate nils are returned,
                    // otherwise the iteration will just stop
                    ThreadStatus::Resumable => Ok(thread.resume::<_, Option<LevelEvent>>(())?),
                    ThreadStatus::Unresumable => Ok(None),
                    ThreadStatus::Error => bail!("Thread errored"),
                }
            })
            .expect("Lua thread errored while interpreting level")
    }
}

impl Level for LuaLevel {}
