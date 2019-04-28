use crate::game::{
    start_new_level, FloorsVisited, GameMap, GamePosition, Item, ItemType, LoadedSpriteSheet,
    NumEnemiesLeft, Player, PlayerEntity, PowerUps, RoomFlag, Shooter, TimeLeft,
};
use amethyst::core::Transform;
use amethyst::ecs::{
    Builder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect,
    WriteStorage,
};
use amethyst::renderer::{SpriteRender, Transparent};
use std::time::Duration;

pub struct CheckForNextRoom;

impl<'a> System<'a> for CheckForNextRoom {
    type SystemData = (
        WriteExpect<'a, GameMap>,
        WriteStorage<'a, GamePosition>,
        ReadStorage<'a, Player>,
        Read<'a, PlayerEntity>,
        Read<'a, LoadedSpriteSheet>,
        WriteExpect<'a, NumEnemiesLeft>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, RoomFlag>,
        WriteExpect<'a, FloorsVisited>,
        WriteStorage<'a, TimeLeft>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut gamemap,
            mut game_poses,
            player_flag,
            player_ent,
            sprite_sheet,
            mut num_enemies,
            entities,
            lazy,
            room_flag,
            mut floors,
            mut time_left_store
        ) = data;

        let player = player_ent.0.unwrap();
        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();
        let time_left = time_left_store.get_mut(player_ent.0.unwrap()).unwrap();

        if num_enemies.0 == 0 && player_pos.to_tile() == gamemap.ending_spot {
            for (ent, _) in (&entities, &room_flag).join() {

                entities.delete(ent).unwrap();
            }
            time_left.add(Duration::from_secs(10));
            start_new_level(
                &mut gamemap,
                game_poses,
                player,
                &sprite_sheet,
                &mut num_enemies,
                entities,
                &lazy,
                &floors,
            );

            floors.0 += 1;
            warn!("MADE IT: {:?}", *floors);
        }
    }
}

pub struct CreatePowerUps;

impl<'a> System<'a> for CreatePowerUps {
    type SystemData = (
        ReadExpect<'a, NumEnemiesLeft>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadExpect<'a, LoadedSpriteSheet>,
        WriteExpect<'a, GameMap>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (num_enemies, entities, lazy, sprite_sheet, mut game_map) = data;

        if num_enemies.0 == 0 && !game_map.powerups_spawned {
            let (x, y) = game_map.ending_spot;
            game_map.powerups_spawned = true;

            spawn_powerup((x + 1, y), &lazy, &entities, &sprite_sheet,10);
            spawn_powerup((x - 1, y), &lazy, &entities, &sprite_sheet,10);
        }
    }
}

pub fn spawn_powerup(
    location: (usize, usize),
    lazy: &LazyUpdate,
    entities: &Entities,
    sprite_sheet: &LoadedSpriteSheet,
    cost: u64,
) {
    spawn_item(
        location,
        lazy,
        entities,
        sprite_sheet,
        ItemType::PowerUp(PowerUps::get_random()),
        cost,
    );
}

pub fn spawn_item(
    location: (usize, usize),
    lazy: &LazyUpdate,
    entities: &Entities,
    sprite_sheet: &LoadedSpriteSheet,
    item_type: ItemType,
    cost: u64
) {
    let sprite_other_floor = SpriteRender {
        sprite_sheet: sprite_sheet.0.clone().unwrap(),
        sprite_number: item_type.to_sprite_sheet_num(), // paddle is the first sprite in the sprite_sheet
    };

    let anim = item_type.get_anim();

    let mut transform = Transform::default();

    lazy.create_entity(entities)
        .with(Item::new(location, item_type, cost))
        .with(GamePosition::from_tile(location))
        .with(transform)
        .with(sprite_other_floor)
        .with(Transparent)
        .with(RoomFlag)
        .with(anim)
        .build();
}

pub struct PickupItem;

impl<'a> System<'a> for PickupItem {
    type SystemData = (
        ReadStorage<'a, Item>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Shooter>,
        WriteStorage<'a, TimeLeft>,
        Read<'a, PlayerEntity>,
        Entities<'a>,
        ReadStorage<'a, GamePosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            item_store,
            mut player_flag,
            mut shooter_flag,
            mut time_left_store,
            player_ent,
            entities,
            game_poses,
        ) = data;

        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();
        let player = player_flag.get_mut(player_ent.0.unwrap()).unwrap();
        let shooter = shooter_flag.get_mut(player_ent.0.unwrap()).unwrap();
        let time_left = time_left_store.get_mut(player_ent.0.unwrap()).unwrap();

        for (ent, item) in (&entities, &item_store).join() {
            if item.location == player_pos.to_tile() {
                warn!("Picked up: {:?}", item);
                player.apply_item(item);
                shooter.apply_item(item);
                time_left.apply_item(item);
                entities.delete(ent).unwrap();
            }
        }
    }
}
