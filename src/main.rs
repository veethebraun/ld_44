extern crate amethyst;

#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate log;

use amethyst::audio::AudioBundle;
use amethyst::core::transform::TransformBundle;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage, ColorMask, ALPHA, DepthMode};
use amethyst::ui::UiBundle;

mod game;
use crate::game::Game;

mod pausable_game_data;
use pausable_game_data::PausableGameDataBuilder;
mod audio;
mod pause_screen;
use audio::Music;
mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    use amethyst::utils::application_root_dir;
    let path = format!("{}/resources/display_config.ron", application_root_dir());

    let config = DisplayConfig::load(&path);

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
//            .clear_target([0.137, 0.068, 0.137, 1.0], 10.0)
            .clear_target([0.068, 0.034, 0.068, 1.0], 10.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(amethyst::ui::DrawUi::new()),
    );

    use amethyst::input::InputBundle;

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let game_data = PausableGameDataBuilder::default()
        .with_base_bundle(TransformBundle::new())?
        .with_base_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor().with_sprite_visibility_sorting(&["transform_system"]))?
        .with_base_bundle(AudioBundle::new(|music: &mut Music| music.music.next()))? //|music: &mut Music| music.music.next()))?
        .with_running_bundle(input_bundle)?
        .with_base_bundle(UiBundle::<String, String>::new())?
        .with_running(systems::MovePlayerSystem, "move_player", &["input_system"])
        .with_running(systems::GamePositionTransform, "gamepos_transform", &["move_player"])
//        .with_running(systems::PaddleSystem, "paddle_system", &["input_system"])
//        .with_running(systems::MoveBallSystem, "move_ball", &[])
//        .with_running(systems::SoundFxSystem, "sound_fx", &["input_system"]);
    ;

    let mut da_game = Application::new("./", Game, game_data)?;

    da_game.run();

    Ok(())
}
