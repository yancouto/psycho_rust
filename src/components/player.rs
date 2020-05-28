use amethyst::ecs::{Component, NullStorage};

/// Creates a component that's just a tag, that is, it has no data inside.
macro_rules! tag_components {
    ($($name:ident),*) => {
        $(
        #[derive(Debug, Default, Component)]
        #[storage(NullStorage)]
        pub struct $name;
        )*
    };
}

tag_components!(Player, Enemy, Shot);
