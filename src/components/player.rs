use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug, Default)]
pub struct Player;

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}