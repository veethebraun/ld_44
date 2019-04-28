use crate::pausable_game_data::PausableGameData;
use amethyst::assets::{AssetStorage, Loader};
use amethyst::core::nalgebra::{Point2, Vector2};
use amethyst::core::transform::Transform;
use amethyst::ecs::prelude::{
    Component, DenseVecStorage, Entities, Entity, LazyUpdate, NullStorage, VecStorage, WriteStorage,
};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Flipped, PngFormat, Projection, SpriteRender, SpriteSheet, SpriteSheetFormat,
    SpriteSheetHandle, Texture, TextureMetadata, Transparent, VirtualKeyCode,
};
use itertools::{iproduct, Itertools};
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};

use crate::audio::{initialise_audio, Music};
use crate::maps;
use crate::pause_screen::Paused;
use std::time::Duration;

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1366.0;

pub struct Game;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Game {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let world = data.world;

        world.add_resource::<Music>(Default::default());
        world.add_resource::<NumEnemiesLeft>(NumEnemiesLeft(0));
        world.register::<EnemyFlag>();
        world.register::<TimeLeft>();
        world.add_resource(FloorsVisited(0));
        // Init Stuff
        let sprite_sheet = load_sprite_sheet(world);
        world.add_resource(LoadedSpriteSheet(Some(sprite_sheet.clone())));
        init_game_map(world, sprite_sheet.clone());
        init_player(world, sprite_sheet.clone());
        initialise_camera(world);
        init_enemy(world, sprite_sheet.clone());
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
#[storage(VecStorage)]
pub struct Player {
    pub current_direction: ShootDirection,
    pub speed: [f32; 2],
    pub invincible_time: Duration,
}

#[derive(Default)]
pub struct PlayerEntity(pub Option<Entity>);

#[derive(Default, Debug)]
pub struct LoadedSpriteSheet(pub Option<SpriteSheetHandle>);

#[derive(Default, Debug)]
pub struct FloorsVisited(pub u32);

#[derive(Component, Debug, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct GamePosition(pub [f32; 2]);

impl GamePosition {
    pub fn to_tile(&self) -> (usize, usize) {
        (
            (self.0[0] / TILE_SIZE as f32) as usize,
            (self.0[1] / TILE_SIZE as f32) as usize,
        )
    }

    pub fn from_tile((x, y): (usize, usize)) -> Self {
        let x_gpos = (x as f32) * TILE_SIZE as f32;
        let y_gpos = (y as f32) * TILE_SIZE as f32;
        Self([x_gpos, y_gpos])
    }
}

impl Default for GamePosition {
    fn default() -> Self {
        GamePosition([0., 0.])
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

#[derive(Component)]
#[storage(VecStorage)]
pub struct Shooter {
    pub shoot_rate: Duration,
    pub current_cooldown: Duration,
    pub speed: f32,
}

impl Shooter {
    pub fn new(rate: u64, speed: f32) -> Self {
        Self {
            shoot_rate: Duration::from_secs(rate),
            current_cooldown: Duration::from_secs(rate),
            speed,
        }
    }
}

fn init_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let transform = Transform::default();

    let start = world.read_resource::<GameMap>().player_start;

    let mut game_pos = GamePosition::from_tile(start); //TODO: Start according to map

    // Create a left plank entity.
    let ent = world
        .create_entity()
        .with(Player::default())
        .with(Shooter::new(1, 5.))
        .with(game_pos)
        .with(transform)
        .with(sprite_render.clone())
        .with(Transparent)
        .with(CollisionDetectionFlag([45., 45.]))
        .with(TimeLeft::new(180))
        .build();

    world.add_resource(PlayerEntity(Some(ent)));

    // Create right plank entity.
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMapTile {
    Wall,
    Floor,
    Nothing,
    PlayerStart,
    Teleport,
}

impl Default for GameMapTile {
    fn default() -> Self {
        GameMapTile::Nothing
    }
}

pub const GAME_MAP_X: usize = 32;
pub const GAME_MAP_Y: usize = 32;
pub const TILE_SIZE: u32 = 60;

#[derive(Debug, Default)]
pub struct GameMap {
    pub map: maps::game_map_tiles,
    pub player_start: (usize, usize),
    pub valid_enemy_spawns: Vec<(usize, usize)>,
    pub ending_spot: (usize, usize),
    pub current_level: usize,
}

impl GameMap {
    pub fn new(map: maps::game_map_tiles, current_level: usize) -> Self {
        Self {
            map,
            player_start: (GAME_MAP_X - 1, GAME_MAP_Y - 1),
            valid_enemy_spawns: Vec::new(),
            ending_spot: (GAME_MAP_X - 1, GAME_MAP_Y - 1),
            current_level,
        }
    }
}

fn init_game_map(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let map = "WWWWWWWWWWWWWWWWWWWWW\n\
               WFFFFFFFFFFFFFFFFFFFF\n\
               WFFFFFFFFFFFFFFFFFFFF";

    let mut truemap = GameMap::new(maps::turn_into_game_map(maps::LEVEL2), 1);

    let sprite_wall = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 2, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_floor = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 3, // paddle is the first sprite in the sprite_sheet
    };

    for (x, y) in iproduct!(0..GAME_MAP_X, 0..GAME_MAP_Y) {
        let game_pos = GamePosition::from_tile((x, y));
        let mut transform = Transform::default();
        transform.set_z(-1.0);
        let mut builder = world
            .create_entity()
            .with(transform)
            .with(game_pos)
            .with(RoomFlag);

        match truemap.map[x][y] {
            GameMapTile::Wall => {
                builder = builder
                    .with(sprite_wall.clone())
                    .with(CollisionDetectionFlag([60.0, 60.0]))
                    .with(WallFlag);
            }
            GameMapTile::Floor => {
                builder = builder.with(sprite_floor.clone());
                truemap.valid_enemy_spawns.push((x, y));
            }
            GameMapTile::PlayerStart => {
                builder = builder.with(sprite_floor.clone());
                truemap.player_start = (x, y);
            }
            GameMapTile::Teleport => {
                builder = builder.with(sprite_floor.clone());
                truemap.ending_spot = (x, y);
            }
            _ => (),
        };

        builder.build();
    }

    warn!("{:?}", truemap.player_start);

    world.add_resource(truemap);
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct WallFlag;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct RoomFlag;

#[derive(Component, Default, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct CollisionDetectionFlag(pub [f32; 2]);

#[derive(Component, Default, Debug)]
#[storage(VecStorage)]
pub struct TimeLeft {
    pub remainaing: Duration,
    pub max: Duration,
}

impl TimeLeft {
    pub fn new(time: u64) -> Self {
        Self {
            remainaing: Duration::from_secs(time),
            max: Duration::from_secs(time),
        }
    }
    pub fn subtract(&mut self, time: Duration) {
        if self.remainaing > time {
            self.remainaing -= time;
        } else {
            self.remainaing = Duration::from_secs(0);
        }
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct EnemyFlag;

pub struct NumEnemiesLeft(pub u32);

fn init_enemy(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
    };

    let mut transform = Transform::default();
    transform.set_z(-0.5);

    let start = world.read_resource::<GameMap>().valid_enemy_spawns[0];

    let mut game_pos = GamePosition::from_tile(start); //TODO: Start according to map

    world
        .create_entity()
        .with(EnemyFlag)
        .with(game_pos)
        .with(transform)
        .with(sprite_render.clone())
        .with(Transparent)
        .with(CollisionDetectionFlag([45., 45.]))
        .with(TimeLeft::new(30))
        .with(Shooter::new(2, 10.))
        .build();

    world.write_resource::<NumEnemiesLeft>().0 += 1;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ShootDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
    None,
}

impl Default for ShootDirection {
    fn default() -> Self {
        ShootDirection::None
    }
}

pub fn start_new_level(
    mut gamemap: &mut GameMap,
    mut game_poses: WriteStorage<GamePosition>,
    player: Entity,
    sprite_sheet: &LoadedSpriteSheet,
    num_enemies: &mut NumEnemiesLeft,
    entities: Entities,
    lazy: &LazyUpdate,
) {
    let sprite_wall = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: 2, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_floor = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: 3, // paddle is the first sprite in the sprite_sheet
    };

    let mut rng = thread_rng();
    let mut choice = Uniform::new(0, maps::NUM_LEVELS).sample(&mut rng);
    while choice == gamemap.current_level {
        choice = Uniform::new(0, maps::NUM_LEVELS).sample(&mut rng);
    }

    let level = maps::turn_into_game_map(maps::LEVELS[choice]);

    gamemap.current_level = choice;
    gamemap.map = level;

    for (x, y) in iproduct!(0..GAME_MAP_X, 0..GAME_MAP_Y) {
        let game_pos = GamePosition::from_tile((x, y));
        let mut transform = Transform::default();
        transform.set_z(-1.0);
        let mut builder = lazy
            .create_entity(&entities)
            .with(transform)
            .with(game_pos)
            .with(RoomFlag);

        match gamemap.map[x][y] {
            GameMapTile::Wall => {
                builder = builder
                    .with(sprite_wall.clone())
                    .with(CollisionDetectionFlag([60.0, 60.0]))
                    .with(WallFlag);
            }
            GameMapTile::Floor => {
                builder = builder.with(sprite_floor.clone());
                gamemap.valid_enemy_spawns.push((x, y));
            }
            GameMapTile::PlayerStart => {
                builder = builder.with(sprite_floor.clone());
                gamemap.player_start = (x, y);

                let player_pos = game_poses.get_mut(player).unwrap();

                warn!("Setting start: {:?} {:?}", (x, y), choice);
                warn!("{:?}", player_pos);

                let new_pos = GamePosition::from_tile((x, y));
                player_pos.0[0] = new_pos.0[0];
                player_pos.0[1] = new_pos.0[1];
                warn!("{:?}", player_pos);
            }
            GameMapTile::Teleport => {
                builder = builder.with(sprite_floor.clone());
                gamemap.ending_spot = (x, y);
            }
            _ => (),
        };

        builder.build();

    }
    let enemy = Enemies::Basic;
    enemy.spawn(&entities, lazy, sprite_sheet, gamemap, num_enemies);
}

enum Enemies {
    Basic,
    Hard,
}

impl Enemies {
    pub fn spawn(
        &self,
        entities: &Entities,
        lazy: &LazyUpdate,
        sprite_sheet: &LoadedSpriteSheet,
        gamemap: &GameMap,
        num_enemies: &mut NumEnemiesLeft,
    ) {
        let sprite_render = SpriteRender {
            sprite_sheet: sprite_sheet.0.clone().unwrap(),
            sprite_number: 0, // paddle is the first sprite in the sprite_sheet
        };

        let mut transform = Transform::default();
        transform.set_z(-0.5);

        let mut rng = thread_rng();

        let start = *gamemap.valid_enemy_spawns.as_slice().choose(&mut rng).unwrap();

        warn!("Spawn enemy here: {:?}", start);
        let mut game_pos = GamePosition::from_tile(start); //TODO: Start according to map

        lazy
            .create_entity(entities)
            .with(EnemyFlag)
            .with(game_pos)
            .with(transform)
            .with(sprite_render.clone())
            .with(Transparent)
            .with(CollisionDetectionFlag([45., 45.]))
            .with(TimeLeft::new(30))
            .with(Shooter::new(2, 10.))
            .build();

        num_enemies.0 += 1;
    }
}
