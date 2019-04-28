use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;

use crate::pausable_game_data::PausableGameData;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::{Component, Join, NullStorage};
use amethyst::renderer::{VirtualKeyCode, SpriteRender, Transparent};
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use amethyst::core::Transform;

use crate::game::{Game, LoadedSpriteSheet, LoadedFonts, ARENA_WIDTH, ARENA_HEIGHT, FloorsVisited};

pub struct GameOverScreen;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for GameOverScreen {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let world = data.world;


        let game_over_sprite = SpriteRender {
            sprite_sheet: {
                let sprite_sheet = world.read_resource::<LoadedSpriteSheet>();
                sprite_sheet.0.clone().unwrap()
            },
            sprite_number: 49,
        };

        {
            let mut ui_store = world.write_storage::<UiText>();
            let entities = world.entities();

            for (ui_text, ent) in (&mut ui_store, &entities).join() {
                entities.delete(ent).unwrap();
            }
        }

        let mut transform = Transform::default();
        transform.set_xyz(ARENA_WIDTH/2., ARENA_HEIGHT/2., 0.9);
        //transform.set_z(0.9);

        world.create_entity()
            .with(transform)
            .with(game_over_sprite)
            .with(Transparent)
            .build();
        // Init Stuff

        let mut floors = world.write_resource::<FloorsVisited>().0;

        let transform = UiTransform::new(
            "timer".to_string(),
            Anchor::TopLeft,
            700.,
            -550.,
            0.95,
            500.,
            70.,
            0,
        );
        let uitext = UiText::new(
            {
                world.read_resource::<LoadedFonts>().0.clone()
            },
            format!("You Cleared {} Floors!",floors),
            [0., 0., 0., 1.],
            70.,
        );

        world.create_entity()
            .with(transform)
            .with(uitext)
            .build();

        let transform = UiTransform::new(
            "timer".to_string(),
            Anchor::TopLeft,
            702.,
            -551.,
            0.94,
            500.,
            70.,
            0,
        );
        let uitext = UiText::new(
            {
                world.read_resource::<LoadedFonts>().0.clone()
            },
            format!("You Cleared {} Floors!",floors),
            [1., 1., 1., 1.],
            70.,
        );

        world.create_entity()
            .with(transform)
            .with(uitext)
            .build();

        world.write_resource::<FloorsVisited>().0 = 0;


        crate::game::initialise_camera(world);
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
                Trans::Switch(Box::new(Game))
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
