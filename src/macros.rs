#[macro_export]
macro_rules! register {
    ($($comp:ident),+ -> $world:ident) => {
        $($world.register::<$comp>();)*
    }
}
