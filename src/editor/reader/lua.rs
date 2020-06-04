use crate::{
    display::{HEIGHT, WIDTH},
    editor::reader::{
        BallEnemy, Formation, Level, LevelEvent, VerticalLinePlacement, VerticalLineSide,
    },
};
use failure::{bail, Error, ResultExt};
use rlua::{Context, Function, Lua, RegistryKey, Result as LuaResult, Table, Thread, ThreadStatus};
use rlua_builders::LuaBuilder;
use std::{fs, iter::Iterator, path::Path};

fn create_level_event(ctx: Context) -> LuaResult<Table> {
    let t = LevelEvent::builder(ctx)?;
    ctx.load(include_str!("coroutine_wrapper.lua"))
        .eval::<Function>()?
        .call::<_, Table>(t)
}

pub struct LuaLevel {
    lua: Lua,
    level_thread: RegistryKey,
}

macro_rules! copy_builders {
    ( $( $name: ident ),+ -> $ctx: ident ) => {
        $( $ctx.globals().set(stringify!($name), $name::builder($ctx)?)?; )+
    }
}

impl LuaLevel {
    pub fn new(path: &Path) -> Result<Self, Error> {
        let lua = Lua::new();
        let level_thread = lua.context::<_, Result<RegistryKey, Error>>(|ctx| {
            let globals = ctx.globals();
            globals.set("LevelEvent", create_level_event(ctx)?)?;
            copy_builders!(BallEnemy, Formation, VerticalLinePlacement, VerticalLineSide -> ctx );
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

#[test]
fn test_test_level_compiles() {
    let level = LuaLevel::new(&Path::new("levels/test.lua")).unwrap();
    // Test if iterator doesn't crash
    assert_eq!(level.collect::<Vec<_>>().is_empty(), false);
}
