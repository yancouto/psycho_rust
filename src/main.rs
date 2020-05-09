//! Psycho: The Ball

mod circle_drawer;
mod quickplay;

use amethyst::{
    prelude::*,
    renderer::{plugins::RenderToWindow, types::DefaultBackend, RenderingBundle},
    utils::application_root_dir,
};
use circle_drawer::RenderCircles;
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
        )?;
    let mut game = Application::new(app_root.join("assets"), Quickplay, game_data)?;
    game.run();
    Ok(())
}
