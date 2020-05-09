use amethyst::core::math::Point2;
use amethyst::ecs::{Component, DenseVecStorage};

pub type Position = Point2<f32>;

pub struct Transform(pub Position);

impl Component for Transform {
    type Storage = DenseVecStorage<Self>;
}

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
