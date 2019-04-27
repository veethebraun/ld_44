use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;

use crate::pausable_game_data::PausableGameData;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::{Component, Join, NullStorage};
use amethyst::renderer::VirtualKeyCode;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};

pub struct Paused;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        // Init Stuff
    }

    fn handle_event(
        &mut self,
        data: StateData<PausableGameData>,
        event: StateEvent,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Space) {
                // Delete pause screen stuff
                Trans::Pop
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
        data.data.update(&data.world, false);
        Trans::None
    }
}
