use amethyst::core::math::{Point2, Vector2};
use derive_more::Display;
use failure::{bail, Error, Fail, ResultExt};
use rlua::{
    Context, Error as LuaErrorInner, Function, Lua, RegistryKey, Result as LuaResult, Table,
    Thread, ThreadStatus, UserData,
};
use std::{fs, iter::Iterator, path::Path};

#[derive(Debug, Clone)]
pub enum EnemyType {
    SimpleBall,
}

impl UserData for EnemyType {}

#[derive(Debug, Clone)]
pub enum LevelEvent {
    Wait(f32),
    Formation(FormationEvent),
}

#[derive(Debug, Clone)]
pub enum FormationEvent {
    Single {
        enemy: EnemyType,
        pos: Point2<f32>,
        speed: Vector2<f32>,
    },
}

#[derive(Debug, Copy, Clone)]
struct Vec2(f32, f32);

impl UserData for Vec2 {}

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

impl UserData for LevelEvent {}

pub trait Level: Iterator<Item = LevelEvent> {}

fn create_level_manager(ctx: Context) -> LuaResult<Table> {
    let t = ctx.create_table()?;
    t.set(
        "wait",
        ctx.create_function(|_, val: f32| Ok(LevelEvent::Wait(val)))?,
    )?;
    ctx.load(include_str!("coroutine_wrapper.lua"))
        .eval::<Function>()?
        .call::<_, Table>(t)
}

fn create_formations(ctx: Context) -> LuaResult<Table> {
    let f = ctx.create_table()?;
    f.set(
        "single",
        ctx.create_function(|_, data: Table| {
            Ok(LevelEvent::Formation(FormationEvent::Single {
                enemy: data.get::<_, EnemyType>("enemy")?,
                pos: data.get::<_, Vec2>("pos")?.into(),
                speed: data.get::<_, Vec2>("speed")?.into(),
            }))
        })?,
    )?;
    ctx.load(include_str!("coroutine_wrapper.lua"))
        .eval::<Function>()?
        .call::<_, Table>(f)
}

fn create_enemies(ctx: Context) -> LuaResult<Table> {
    let e = ctx.create_table()?;
    e.set("SimpleBall", EnemyType::SimpleBall)?;
    Ok(e)
}

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
            globals.set("Formations", create_formations(ctx)?)?;
            globals.set("Enemies", create_enemies(ctx)?)?;
            globals.set(
                "vec2",
                ctx.create_function(|_, (x, y): (f32, f32)| Ok(Vec2(x, y)))?,
            )?;

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
