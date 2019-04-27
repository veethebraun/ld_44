use crate::game::{CollisionDetectionFlag, GameMap, GameMapTile, GamePosition, Player, TILE_SIZE, EnemyFlag, PlayerEntity};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

pub struct MovePlayerSystem;

impl<'a> System<'a> for MovePlayerSystem {
    type SystemData = (
        WriteStorage<'a, GamePosition>,
        ReadStorage<'a, Player>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, GameMap>,
        ReadStorage<'a, CollisionDetectionFlag>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_poses, player_flag, input, game_map, coll_flags) = data;
        for (_, game_pos, coll_flag) in (&player_flag, &mut game_poses, &coll_flags).join() {
            let x_movement = input.axis_value("player_leftright");
            let y_movement = input.axis_value("player_updown");
            if let Some(mv_amount) = y_movement {
                let scaled_amount = 5.2 * mv_amount as f32;
                let player_y = game_pos.0[1];

                try_move(game_pos, 1, scaled_amount, &game_map, coll_flag);
            }
            if let Some(mv_amount) = x_movement {
                let scaled_amount = 5.2 * mv_amount as f32;
                try_move(game_pos, 0, scaled_amount, &game_map, coll_flag);
            }
            //println!("{:?}", input.action_is_down("hi"));
        }
    }
}

fn try_move(
    game_pos: &mut GamePosition,
    idx: usize,
    amount: f32,
    game_map: &GameMap,
    coll_flag: &CollisionDetectionFlag,
)  {
    let mut new_game_pos = GamePosition(game_pos.0);
    let mut new_game_pos_coll = GamePosition(game_pos.0);
    new_game_pos.0[idx] += amount;
    new_game_pos_coll.0[idx] += amount;
    new_game_pos_coll.0[idx] += coll_flag.0[idx];

    let (x, y) = new_game_pos.to_tile();
    let (x_c, y_c) = new_game_pos_coll.to_tile();

    if game_map.0[x][y] == GameMapTile::Wall || game_map.0[x_c][y_c] == GameMapTile::Wall {
        let diff = if game_map.0[x][y] == GameMapTile::Wall {
            let wall_pos = GamePosition::from_tile((x,y));
            wall_pos.0[idx] - game_pos.0[idx] + TILE_SIZE as f32
        } else {
            let wall_pos = GamePosition::from_tile((x_c,y_c));
            wall_pos.0[idx] - game_pos.0[idx] - coll_flag.0[idx]
        };
        game_pos.0[idx] += diff;
    } else {
        game_pos.0[idx] += amount;
    }
}

pub struct MoveBadGuys;

impl<'a> System<'a> for MoveBadGuys {
    type SystemData = (
        WriteStorage<'a, GamePosition>,
        ReadStorage<'a, EnemyFlag>,
        Read<'a, GameMap>,
        ReadStorage<'a, CollisionDetectionFlag>,
        Read<'a, PlayerEntity>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_poses, enemy_flag, game_map, coll_flags, player_ent) = data;

        let player_pos = game_poses.get(player_ent.0.unwrap() ).unwrap().clone();

        for (_, pos, coll_flag) in (&enemy_flag, &mut game_poses, &coll_flags).join() {
            let pos_diff = [player_pos.0[0] - pos.0[0], player_pos.0[1] - pos.0[1] ];
            let r = (pos_diff[0]*pos_diff[0] + pos_diff[1]*pos_diff[1]).sqrt();

            try_move(pos, 0, 2.*pos_diff[0]/r, &game_map, coll_flag);
            try_move(pos, 1, 2.*pos_diff[1]/r, &game_map, coll_flag);
        }

    }
}