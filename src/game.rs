use crate::pausable_game_data::PausableGameData;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, };
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, VirtualKeyCode,
};

use crate::pause_screen::Paused;
use crate::audio::{initialise_audio, Music};

pub struct Game;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Game {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let world = data.world;

        // Init Stuff

    }

    fn handle_event(
        &mut self,
        _: StateData<PausableGameData>,
        event: StateEvent,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                Trans::Push(Box::new(Paused))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        data: StateData<PausableGameData>,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, true);
        Trans::None
    }
}