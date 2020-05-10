//! Psycho: The Ball

mod bindings;
mod circle_drawer;
mod player_move;
mod quickplay;
mod screen;
mod transform;

use amethyst::{
    input::InputBundle,
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
};
use bindings::PsychoBindingTypes;
use circle_drawer::RenderCircles;
use player_move::PlayerMoveSystem;
use quickplay::Quickplay;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display.ron"))?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderCircles),
        )?
        .with_bundle(
            InputBundle::<PsychoBindingTypes>::new()
                .with_bindings_from_file(app_root.join("config/bindings.ron"))?,
        )?
        .with(PlayerMoveSystem, "player_move_system", &["input_system"]);
    let mut game = Application::new(app_root.join("assets"), Quickplay, game_data)?;
    game.run();
    Ok(())
}
