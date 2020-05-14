use amethyst::ecs::{Component, NullStorage};

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Enemy;

#[derive(Debug, Default, Component)]
#[storage(NullStorage)]
pub struct Shot;