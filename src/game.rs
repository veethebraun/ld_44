use crate::pausable_game_data::PausableGameData;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{Component, DenseVecStorage, NullStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, VirtualKeyCode,Transparent,
};
use amethyst::core::nalgebra::{Vector2, Point2};
use itertools::{Itertools, iproduct};

use crate::audio::{initialise_audio, Music};
use crate::pause_screen::Paused;

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1366.0;

pub struct Game;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Game {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let world = data.world;

        world.add_resource::<Music>(Default::default());
        world.register::<Player>();
        // Init Stuff
        let sprite_sheet = load_sprite_sheet(world);
        init_player(world, sprite_sheet.clone());
        initialise_camera(world);
        init_game_map(world, sprite_sheet.clone());
    }

    fn handle_event(
        &mut self,
        _: StateData<PausableGameData>,
        event: StateEvent,
    ) -> Trans<PausableGameData<'a, 'b>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else if is_key_down(&event, VirtualKeyCode::Escape) {
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

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Player;

#[derive(Component, Debug)]
#[storage(DenseVecStorage)]
pub struct GamePosition(pub [f32;2]);

impl Default for GamePosition {
    fn default() -> Self {
        GamePosition(
            [0., 0.]
        )
    }
}

fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    // Load the sprite sheet necessary to render the graphics.
    // The texture is the pixel data
    // `texture_handle` is a cloneable reference to the texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/sheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/sheet.ron", // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn init_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let transform = Transform::default();

    let mut game_pos: GamePosition = Default::default();

    // Create a left plank entity.
    world
        .create_entity()
        .with(Player)
        .with(game_pos)
        .with(transform)
        .with(sprite_render.clone())
        .with(Transparent)
        .build();

    // Create right plank entity.
}

#[derive(Debug,Clone,Copy,PartialEq)]
pub enum GameMapTile{
    Wall,
    Floor,
    Nothing,
}

impl Default for GameMapTile {
    fn default() -> Self {
        GameMapTile::Nothing
    }
}


pub const GAME_MAP_X: usize = 20;
pub const GAME_MAP_Y: usize = 12;
pub const TILE_SIZE: u32 =  60;

#[derive(Debug, )]
pub struct GameMap(pub [[GameMapTile; GAME_MAP_Y]; GAME_MAP_X]);

fn init_game_map(world: &mut World, sprite_sheet: SpriteSheetHandle) {

    let map =
        "WWWWWWWWWWWWWWWWWWWWW\n\
         WFFFFFFFFFFFFFFFFFFFF\n\
         WFFFFFFFFFFFFFFFFFFFF";

    let mut truemap = GameMap([[GameMapTile::Floor; GAME_MAP_Y]; GAME_MAP_X]);
    for x in 0..GAME_MAP_X {
        truemap.0[x][0] = GameMapTile::Wall;
        truemap.0[x][GAME_MAP_Y-1] = GameMapTile::Wall;
    }

    for y in 0..GAME_MAP_Y {
        truemap.0[0][y] = GameMapTile::Wall;
        truemap.0[GAME_MAP_X-1][y] = GameMapTile::Wall;
    }

    let sprite_wall = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_floor = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 3, // paddle is the first sprite in the sprite_sheet
    };

    for (x, y) in iproduct!(0..GAME_MAP_X, 0..GAME_MAP_Y) {
        let x_gpos = (x as f32 - GAME_MAP_X as f32/2.)*TILE_SIZE as f32;
        let y_gpos = (y as f32 - GAME_MAP_Y as f32/2.)*TILE_SIZE as f32;

        let game_pos = GamePosition([x_gpos, y_gpos]);
        warn!("{:?}, {:?}, {:?}", x, y, game_pos);
        let mut transform = Transform::default();
        transform.set_z(-1.0);

        let sprite = match truemap.0[x][y]{
            GameMapTile::Wall => sprite_wall.clone(),
            GameMapTile::Floor => sprite_floor.clone(),
            _ => sprite_floor.clone(),
        };

        world
            .create_entity()
            .with(transform)
            .with(game_pos)
            .with(sprite)
            .build();
    }

    info!("{:?}", truemap);
}