use crate::game::{start_new_level, GameMap, GamePosition, LoadedSpriteSheet, NumEnemiesLeft, Player, PlayerEntity, RoomFlag, FloorsVisited};
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage,
};

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
        ) = data;

        let player = player_ent.0.unwrap();
        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();

        if num_enemies.0 == 0 && player_pos.to_tile() == gamemap.ending_spot {
            for (ent, _) in (&entities, &room_flag).join() {
                entities.delete(ent).unwrap();
            }
            start_new_level(
                &mut gamemap,
                game_poses,
                player,
                &sprite_sheet,
                &mut num_enemies,
                entities,
                &lazy,
            );

            floors.0 += 1;
            warn!("MADE IT: {:?}", *floors);
        }
    }
}
