use crate::pausable_game_data::PausableGameData;
use amethyst::assets::{AssetStorage, Handle, Loader};
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
use amethyst::ui::FontAsset;
use amethyst::ui::{Anchor, TtfFormat, UiText, UiTransform};
use itertools::{iproduct, Itertools};
use rand::{
    distributions::{Distribution, Uniform},
    seq::SliceRandom,
    thread_rng,
};

use crate::audio::{initialise_audio, Music};
use crate::maps;
use crate::pause_screen::Paused;
use crate::game_over_screen::GameOverScreen;
use std::time::Duration;
use crate::game_scale::{gen_enemy_list, get_enemy_time, get_enemy_bullet_speed};
use std::iter::{Cycle, Iterator};
use std::slice::Iter;
use crate::maps::game_map_tiles;

pub const ARENA_HEIGHT: f32 = 768.0;
pub const ARENA_WIDTH: f32 = 1366.0;

pub struct Game;
impl<'a, 'b> State<PausableGameData<'a, 'b>, StateEvent> for Game {
    fn on_start(&mut self, data: StateData<PausableGameData>) {
        let world = data.world;

        world.delete_all();

        world.add_resource::<Music>(Default::default());
        world.add_resource::<NumEnemiesLeft>(NumEnemiesLeft(0));
        world.register::<EnemyFlag>();
        world.register::<Item>();
        world.add_resource(FloorsVisited(0));
        world.add_resource(GameOver(false));
        // Init Stuff
        let sprite_sheet = load_sprite_sheet(world);
        world.add_resource(LoadedSpriteSheet(Some(sprite_sheet.clone())));

        let (font1, font2) = load_fonts(world);
        world.add_resource(LoadedFonts(font1.clone(), font2.clone()));

        init_game_map(world, sprite_sheet.clone());
        init_player(world, sprite_sheet.clone());
        initialise_camera(world);
        init_enemy(world, sprite_sheet.clone());
        init_ui(world);
        initialise_audio(world);
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

        if data.world.read_resource::<GameOver>().0 {
            Trans::Switch(Box::new(GameOverScreen))
        } else {
            Trans::None
        }
    }
}

pub struct GameOver(pub bool);

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub current_direction: ShootDirection,
    pub speed: [f32; 2],
    pub invincible_time: Duration,
    pub damage: u64,
    pub speed_multi: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            current_direction: Default::default(),
            speed: Default::default(),
            invincible_time: Default::default(),
            damage: 10,
            speed_multi: 4.,
        }
    }
}

impl Player {
    pub fn apply_item(&mut self, item: &Item) {
        match item.kind {
            ItemType::PowerUp(PowerUps::Damage) => {
                self.damage += 10;
            }
            ItemType::PowerUp(PowerUps::Speed) => {
                self.speed_multi += 1.0;
                warn!("New speed: {:?}", self.speed_multi);
            }
            _ => (),
        };
        //
    }
}

#[derive(Default)]
pub struct PlayerEntity(pub Option<Entity>);

#[derive(Default, Debug)]
pub struct LoadedSpriteSheet(pub Option<SpriteSheetHandle>);

#[derive(Debug)]
pub struct LoadedFonts(pub Handle<FontAsset>, pub Handle<FontAsset>);

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

fn load_fonts(world: &mut World) -> (Handle<FontAsset>, Handle<FontAsset>) {
    let font1 = world.read_resource::<Loader>().load(
        "fonts/NanumPenScript-Regular.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );
    let font2 = world.read_resource::<Loader>().load(
        "fonts/ZCOOLKuaiLe-Regular.ttf",
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    );

    (font1, font2)
}

pub fn initialise_camera(world: &mut World) {
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
    upgrades: usize,
}

const SHOOTER_UPGRADES_MS: [u64; 11] = [200, 200, 100, 100, 100, 50, 50, 50, 25, 25, 25];

impl Shooter {
    pub fn new(rate: u64, speed: f32) -> Self {
        let mut rng = thread_rng();
        let dither = Uniform::new_inclusive(0, 300).sample(&mut rng);
        Self {
            shoot_rate: Duration::from_secs(rate),
            current_cooldown: Duration::from_secs(rate) + Duration::from_millis(dither),
            speed,
            upgrades: 0,
        }
    }

    pub fn apply_item(&mut self, item: &Item) {
        match item.kind {
            ItemType::PowerUp(PowerUps::ShootRate) => {
                if self.upgrades > SHOOTER_UPGRADES_MS.len() - 1 {
                    self.shoot_rate -= Duration::from_millis(SHOOTER_UPGRADES_MS[10]);
                } else {
                    self.shoot_rate -= Duration::from_millis(SHOOTER_UPGRADES_MS[self.upgrades]);
                    self.upgrades += 1;
                }
            }
            ItemType::PowerUp(PowerUps::ProjectileSpeed) => {
                self.speed += 2.;
            }
            _ => (),
        };
        //
    }

    pub fn set_cooldown(&mut self) {
        self.current_cooldown = self.shoot_rate;
    }
}

fn init_player(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    // Assign the sprites for the paddles
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 16, // paddle is the first sprite in the sprite_sheet
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
        .with(Animation::new(1000, PLAYER_FRAMES))
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
    pub powerups_spawned: bool,
}

impl GameMap {
    pub fn new(map: maps::game_map_tiles, current_level: usize) -> Self {
        Self {
            map,
            player_start: (GAME_MAP_X - 1, GAME_MAP_Y - 1),
            valid_enemy_spawns: Vec::new(),
            ending_spot: (GAME_MAP_X - 1, GAME_MAP_Y - 1),
            current_level,
            powerups_spawned: false,
        }
    }
}

fn init_game_map(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let mut truemap = GameMap::new(maps::turn_into_game_map(maps::LEVEL2), 1);

    let sprite_wall = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 43, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_floor = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 13, // paddle is the first sprite in the sprite_sheet
    };
    let sprite_other_floor = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 14, // paddle is the first sprite in the sprite_sheet
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

                let (s_num, flip) = dbg!(get_wall_sprite_num((x,y), &truemap));

                let sprite = SpriteRender {
                    sprite_sheet: sprite_sheet.clone(),
                    sprite_number: s_num, // paddle is the first sprite in the sprite_sheet
                };

                builder = builder
                    .with(sprite.clone())
                    .with(CollisionDetectionFlag([60.0, 60.0]))
                    .with(WallFlag);

                if flip {
                    builder = builder.with(Flipped::Horizontal);
                }
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
                builder = builder.with(sprite_other_floor.clone());
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

    pub fn add(&mut self, time: Duration) {
        if self.remainaing + time > self.max {
            self.remainaing = self.max;
        } else {
            self.remainaing += time;
        }
    }

    pub fn apply_item(&mut self, item: &Item) {
        match item.kind {
            ItemType::PlusTime(x) => {
                self.add(Duration::from_secs(x));
            }
            _ => (),
        };

        self.subtract(Duration::from_secs(item.cost));
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct EnemyFlag {
    pub moves: bool,
}

impl EnemyFlag {
    pub fn new_stationary() -> Self {
        Self {
            moves: false,
        }
    }
}

impl Default for EnemyFlag {
    fn default() -> Self {
        Self {
            moves: true,
        }
    }
}

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
        .with(EnemyFlag::default())
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
    floors: &FloorsVisited,
) {
    let sprite_wall = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: 43, // paddle is the first sprite in the sprite_sheet
    };

    let sprite_floor = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: 13, // paddle is the first sprite in the sprite_sheet
    };
    let sprite_other_floor = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: 14, // paddle is the first sprite in the sprite_sheet
    };

    let mut rng = thread_rng();
    let mut choice = Uniform::new(0, maps::NUM_LEVELS).sample(&mut rng);
    while choice == gamemap.current_level {
        choice = Uniform::new(0, maps::NUM_LEVELS).sample(&mut rng);
    }

    let level = maps::turn_into_game_map(maps::LEVELS[choice]);

    gamemap.current_level = choice;
    gamemap.map = level;
    gamemap.valid_enemy_spawns.clear();
    gamemap.powerups_spawned = false;

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
                let (s_num, flip) = dbg!(get_wall_sprite_num((x,y), &gamemap));

                let sprite = SpriteRender {
                    sprite_sheet: sprite_sheet.0.clone().unwrap(),
                    sprite_number: s_num, // paddle is the first sprite in the sprite_sheet
                };

                builder = builder
                    .with(sprite.clone())
                    .with(CollisionDetectionFlag([60.0, 60.0]))
                    .with(WallFlag);

                if flip {
                    builder = builder.with(Flipped::Horizontal);
                }
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
                builder = builder.with(sprite_other_floor.clone());
                gamemap.ending_spot = (x, y);
            }
            _ => (),
        };

        builder.build();
    }

    Enemies::spawn(&entities, lazy, sprite_sheet, gamemap, num_enemies, floors);
}

pub enum Enemies {
    Stationary,
    NoShoot,
    Full,
}

impl Enemies {
    pub fn spawn(
        entities: &Entities,
        lazy: &LazyUpdate,
        sprite_sheet: &LoadedSpriteSheet,
        gamemap: &GameMap,
        num_enemies: &mut NumEnemiesLeft,
        floors: &FloorsVisited,
    ) {


        for enem in gen_enemy_list(floors.0).iter() {
            let mut transform = Transform::default();
            transform.set_z(-0.5);

            let mut rng = thread_rng();

            let start = *gamemap
                .valid_enemy_spawns
                .as_slice()
                .choose(&mut rng)
                .unwrap();

            warn!("Spawn enemy here: {:?}", start);
            let mut game_pos = GamePosition::from_tile(start); //TODO: Start according to map

            let mut builder = lazy.create_entity(entities)
                .with(game_pos)
                .with(transform)
                .with(Transparent)

                .with(TimeLeft::new(get_enemy_time(floors.0)));

            use self::Enemies::*;
            match enem {
                Stationary => {
                    let sprite_render = SpriteRender {
                        sprite_sheet: sprite_sheet.0.clone().unwrap(),
                        sprite_number: 45, // paddle is the first sprite in the sprite_sheet
                    };

                    builder.with(EnemyFlag::new_stationary())
                        .with(sprite_render.clone())
                        .with(CollisionDetectionFlag([60., 100.]))
                        .with(Shooter::new(2, get_enemy_bullet_speed(floors.0)))
                        .with(Animation::new(500, WALL_CLOCK_FRAMES))
                        .build();
                }
                NoShoot => {
                    let sprite_render = SpriteRender {
                        sprite_sheet: sprite_sheet.0.clone().unwrap(),
                        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
                    };
                    builder.with(EnemyFlag::default())
                        .with(sprite_render.clone())
                        .with(CollisionDetectionFlag([45., 45.]))
                        .build();
                }
                Full => {
                    let sprite_render = SpriteRender {
                        sprite_sheet: sprite_sheet.0.clone().unwrap(),
                        sprite_number: 0, // paddle is the first sprite in the sprite_sheet
                    };
                    builder.with(EnemyFlag::default())
                        .with(sprite_render.clone())
                        .with(CollisionDetectionFlag([45., 45.]))
                        .with(Shooter::new(2, get_enemy_bullet_speed(floors.0))).build();
                }
            }


            num_enemies.0 += 1;
        }
    }
}

pub struct UIEnts(pub Entity);

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct UIFlag;

fn init_ui(world: &mut World) {
    let (transform, uitext) = {
        let loaded_fonts = world.read_resource::<LoadedFonts>();
        let sprite_sheet = world.read_resource::<LoadedSpriteSheet>();

        let transform = UiTransform::new(
            "timer".to_string(),
            Anchor::TopLeft,
            700.,
            -20.,
            1.,
            200.,
            50.,
            0,
        );

        let uitext = UiText::new(
            loaded_fonts.0.clone(),
            "0.0".to_string(),
            [1., 1., 1., 1.],
            50.,
        );

        (transform, uitext)
    };

    let ent = world
        .create_entity()
        //.with(UIFlag)
        .with(transform)
        .with(uitext)
        .build();

    world.add_resource(UIEnts(ent));
}

#[derive(Clone, Copy, Debug)]
pub enum PowerUps {
    Speed,
    ShootRate,
    ProjectileSpeed,
    Damage,
}

impl PowerUps {
    pub fn get_random() -> Self {
        let mut rng = thread_rng();
        use self::PowerUps::*;
        *[Speed, ShootRate, ProjectileSpeed, Damage]
            .choose(&mut rng)
            .unwrap()
    }
}

#[derive(Component, Clone, Copy, Debug)]
#[storage(VecStorage)]
pub struct Item {
    pub location: (usize, usize),
    pub kind: ItemType,
    pub cost: u64,
}

impl Item {
    pub fn new(location: (usize, usize), kind: ItemType, cost: u64) -> Self {
        Self { location, kind, cost }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ItemType {
    PowerUp(PowerUps),
    PlusTime(u64),
}

impl ItemType {
    pub fn to_sprite_sheet_num(&self) -> usize {
        use self::ItemType::*;
        match self {
            PowerUp(PowerType) => {
                match PowerType {
                    PowerUps::Speed => 36,
                    PowerUps::Damage => 1,
                    PowerUps::ProjectileSpeed => 24,
                    PowerUps::ShootRate => 30,
                }
            }
            PlusTime(_) => {
                11
            }
        }
    }

    pub fn get_anim(&self) -> Animation {
        use self::ItemType::*;
        use self::PowerUps::*;
        match self {
            PowerUp(Speed) => Animation::new(200, SPEED_POWER_FRAMES),
            PowerUp(Damage) => Animation::new(200, DAMAGE_POWER_FRAMES),
            PowerUp(ProjectileSpeed) => Animation::new(200, PROJ_POWER_FRAMES),
            PowerUp(ShootRate) => Animation::new(200, SHOOT_FAST_POWER_FRAMES),
            PlusTime(_) => Animation::new(200, MOAR_TIME_FRAMES)
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Animation {
    pub time_per_frame: Duration,
    pub cooldown: Duration,
    pub sprites: Cycle<Iter<'static, usize>>,
}

impl Animation {
    pub fn new(time_per_frame: u64, sprites: &'static [usize]) -> Self {
        let cooldown = Duration::from_millis(
            Uniform::new(0,time_per_frame).sample(&mut thread_rng())
        );

        Self {
            time_per_frame: Duration::from_millis(time_per_frame),
            cooldown,
            sprites: sprites.iter().cycle()
        }
    }
}

pub const PLAYER_FRAMES: &[usize] = &[16, 17, 18, 18];
pub const DAMAGE_POWER_FRAMES: &[usize] = &[1,2,3,4,5,6];
pub const DEAD_ENEMY_FRAMES: &[usize] = &[7,8,9];
pub const PLAYER_INVUL_FRAMES: &[usize] = &[20,21,22,23];
pub const PROJ_POWER_FRAMES: &[usize] = &[24,25,26,27,28,29];
pub const SHOOT_FAST_POWER_FRAMES: &[usize] = &[30,31,32,33,34,35];
pub const SPEED_POWER_FRAMES: &[usize] = &[36,37,38,39,40,41];
pub const WALL_CLOCK_FRAMES: &[usize] = &[45, 46, 47, 48];
pub const MOAR_TIME_FRAMES: &[usize] = &[11,11,12];


fn get_wall_sprite_num((x,y): (usize, usize), gamemap: &GameMap) -> (usize, bool) {
    if y - 1 != 0 && y - 1 < GAME_MAP_Y && gamemap.map[x][y-1] == GameMapTile::Floor {
        (43, false)
    } else if x + 1 < GAME_MAP_X && gamemap.map[x+1][y] == GameMapTile::Floor {
        (42, false)
    } else if x - 1 != 0 && x - 1 < GAME_MAP_X && gamemap.map[x-1][y] == GameMapTile::Floor {
        (42, true)
    } else {
        (52, false)
    }
}