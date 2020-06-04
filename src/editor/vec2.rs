use crate::components::{Moving, Transform};
use amethyst::core::math::{Point2, Vector2};
use rlua::{Context, Error, FromLua, Result, Value};

/// Our wrapper for Vector2<f32> and Point2<f32>, since we need but can't
/// implement some traits on it
#[derive(Debug, Copy, Clone)]
pub struct Vec2(pub f32, pub f32);

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

impl<'s> FromLua<'s> for Vec2 {
    fn from_lua(val: Value<'s>, _: Context<'s>) -> Result<Self> {
        let err = || {
            Err(Error::FromLuaConversionError {
                from: "value",
                to: "Vec2",
                message: Some("Could not convert".to_owned()),
            })
        };
        match val {
            Value::Table(t) => {
                if t.raw_len() == 2 {
                    Ok(Self(t.raw_get::<_, f32>(1)?, t.raw_get::<_, f32>(2)?))
                } else {
                    err()
                }
            }
            _ => err(),
        }
    }
}
