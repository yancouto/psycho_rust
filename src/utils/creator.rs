use amethyst::ecs::{
    world::{Builder, EntitiesRes, Entity, LazyBuilder},
    LazyUpdate,
};

use crate::components::{BallEnemy, Circle, Color, Moving, Transform};

pub struct LazyCreator<'s> {
    pub lazy: &'s LazyUpdate,
    pub entities: &'s EntitiesRes,
}

impl BallEnemy {
    pub fn default_color(&self) -> Color {
        match self {
            BallEnemy::Simple => Color::rgb(0.1, 0.1, 0.9),
            BallEnemy::Double => Color::rgb(0.95, 0.3, 0.1),
        }
    }
}

impl<'s> LazyCreator<'s> {
    pub fn new(lazy: &'s LazyUpdate, entities: &'s EntitiesRes) -> Self {
        Self { lazy, entities }
    }

    pub fn create_entity(&self) -> LazyBuilder<'s> {
        self.lazy.create_entity(self.entities)
    }

    pub fn create_enemy(
        &self,
        enemy: BallEnemy,
        circle: Circle,
        transform: Transform,
        moving: Moving,
    ) -> Entity {
        self.create_entity()
            .with(enemy)
            .with(enemy.default_color())
            .with(circle)
            .with(transform)
            .with(moving)
            .build()
    }
}
