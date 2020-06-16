//! Psycho: The Ball
#![allow(warnings)]

#[macro_use]
mod macros;
mod components;
mod display;
mod editor;
mod input;
mod states;
mod systems;
mod utils;

use amethyst::{
    input::InputBundle,
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
    LogLevelFilter, LoggerConfig,
};
use display::circle_drawer::RenderCircles;
use input::PsychoBindingTypes;
use states::MainMenu;
use systems::MovingSystem;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Debug,
        ..Default::default()
    });
    let app_root = application_root_dir().expect("Failed to get app root dir");
    let game_data = GameDataBuilder::default()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(app_root.join("config/display.ron"))
                        .expect("Failed to read display config")
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderCircles),
        )?
        .with_bundle(
            InputBundle::<PsychoBindingTypes>::new()
                .with_bindings_from_file(app_root.join("config/bindings.ron"))
                .expect("Failed to read bindings"),
        )?
        .with(MovingSystem, "moving", &[]);
    let mut game = Application::new(app_root.join("assets"), MainMenu, game_data)?;
    game.run();
    Ok(())
}
