use derive_more::Display;
use failure::{bail, Error, Fail, ResultExt};
use rlua::{
    Context, Error as LuaErrorInner, Function, Lua, RegistryKey, Result as LuaResult, Table,
    Thread, ThreadStatus, UserData,
};
use std::{fs, iter::Iterator, path::Path};

#[derive(Debug, Clone)]
pub enum LevelEvent {
    Wait(f32),
    CreateEnemy,
}

impl UserData for LevelEvent {}

fn create_level_manager(ctx: Context) -> LuaResult<Table> {
    let t = ctx.create_table()?;
    t.set(
        "wait",
        ctx.create_function(|_, val: f32| Ok(LevelEvent::Wait(val)))?,
    )?;
    t.set(
        "create_enemy",
        ctx.create_function(|_, _: ()| Ok(LevelEvent::CreateEnemy))?,
    )?;
    ctx.load(include_str!("level_manager_wrapper.lua"))
        .eval::<Function>()?
        .call::<_, Table>(t)
}

pub trait Level: Iterator<Item = LevelEvent> {}

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
            let lm = create_level_manager(ctx)?;
            ctx.globals().set("LevelManager", lm)?;
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
