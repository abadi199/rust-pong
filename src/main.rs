extern crate amethyst;

mod pong;
mod systems;

use amethyst::core::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat, Pipeline, PosTex, RenderBundle, Stage};
use pong::Pong;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let path = format!(
        "{}/resources/display_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let config = DisplayConfig::load(&path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat::<PosTex>::new()),
    );

    let binding_path = format!(
        "{}/resources/binding_config.ron",
        env!("CARGO_MANIFEST_DIR")
    );

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)))?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::PaddleSystem, "paddle_system", &["input_system"]);
    let mut game = Application::build("./", Pong)?.build(game_data)?;
    game.run();
    Ok(())
}
