use amethyst::ecs::{Component, DenseVecStorage};

pub use crate::editor::reader::BallEnemy;

impl Component for BallEnemy {
    type Storage = DenseVecStorage<BallEnemy>;
}
