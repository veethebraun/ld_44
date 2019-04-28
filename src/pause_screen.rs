use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;

use crate::pausable_game_data::PausableGameData;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::{Component, Join, NullStorage};
use amethyst::renderer::{VirtualKeyCode, SpriteRender, Transparent};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use crate::game::{LoadedSpriteSheet, ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::core::Transform;

pub struct Paused;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let mut world = data.world;

        world.register::<PauseScreenFlag>();

        let pause_screen_sprite = SpriteRender {
            sprite_sheet: {
                let sprite_sheet = world.read_resource::<LoadedSpriteSheet>();
                sprite_sheet.0.clone().unwrap()
            },
            sprite_number: 50,
        };

        let mut transform = Transform::default();
        transform.set_xyz(ARENA_WIDTH/2., ARENA_HEIGHT/2., 0.9);
        //transform.set_z(0.9);

        let ent = world.create_entity()
            .with(transform)
            .with(pause_screen_sprite)
            .with(Transparent)
            .with(PauseScreenFlag)
            .build();
        // Init Stuff
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
            } else if is_key_down(&event, VirtualKeyCode::Escape) {

                let mut world = data.world;

                let pause_screen = world.read_storage::<PauseScreenFlag>();
                let entities = world.entities();

                for (ent, _) in (&entities, &pause_screen).join() {
                    entities.delete(ent).unwrap();
                }

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

#[derive(Component, Default)]
#[storage(NullStorage)]
struct PauseScreenFlag;