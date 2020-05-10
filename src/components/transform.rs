use amethyst::core::math::{Point2, Vector2};
use amethyst::ecs::{Component, DenseVecStorage};

pub type Position = Point2<f32>;

#[derive(Debug, Component)]
pub struct Transform(pub Position);

impl Transform {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Position::new(x, y))
    }
}

impl From<Position> for Transform {
    fn from(p: Position) -> Self {
        Self(p)
    }
}

#[derive(Debug, Component)]
pub struct Moving(pub Vector2<f32>);

impl Moving {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector2::new(x, y))
    }
}

impl From<Vector2<f32>> for Moving {
    fn from(d: Vector2<f32>) -> Self {
        Self(d)
    }
}
