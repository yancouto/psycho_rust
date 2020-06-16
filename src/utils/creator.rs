use amethyst::ecs::{
    world::{EntitiesRes, LazyBuilder},
    LazyUpdate,
};

pub struct LazyCreator<'s> {
    pub lazy: &'s LazyUpdate,
    pub entities: &'s EntitiesRes,
}

impl<'s> LazyCreator<'s> {
    pub fn create_entity(&mut self) -> LazyBuilder<'s> {
        self.lazy.create_entity(self.entities)
    }
}
