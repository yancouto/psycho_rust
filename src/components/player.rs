use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;