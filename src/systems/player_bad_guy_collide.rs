use crate::game::{CollisionDetectionFlag, GamePosition, Player, EnemyFlag, PlayerEntity, TimeLeft};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use std::time::Duration;

pub struct PlayerBadGuyCollide;

impl<'a> System<'a> for PlayerBadGuyCollide {
    type SystemData = (
        WriteStorage<'a, GamePosition>,
        ReadStorage<'a, EnemyFlag>,
        ReadStorage<'a, CollisionDetectionFlag>,
        Read<'a, PlayerEntity>,
        WriteStorage<'a, TimeLeft>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_poses, enemy_flag, coll_flags, player_ent, mut health_store) = data;

        let player_pos = game_poses.get(player_ent.0.unwrap() ).unwrap().clone();
        let player_coll = coll_flags.get(player_ent.0.unwrap() ).unwrap().clone();
        let mut player_time = health_store.get_mut(player_ent.0.unwrap()).unwrap();

        for (_, pos, coll_flag) in (&enemy_flag, &mut game_poses, &coll_flags).join() {
            if check_collision(player_pos, player_coll, *pos, *coll_flag) {
                player_time.subtract(Duration::from_secs(15));
                warn!("COLLISION!!");

                let player_center_x = player_pos.0[0] + player_coll.0[0]/2.;
                let player_center_y = player_pos.0[1] + player_coll.0[1]/2.;

                let guy_center_x = pos.0[0] + coll_flag.0[0]/2.;
                let guy_center_y = pos.0[1] + coll_flag.0[1]/2.;

                let diff_x = player_center_x - guy_center_x;
                let diff_y = player_center_y - guy_center_y;

                let r = (diff_x*diff_x + diff_y*diff_y).sqrt();

                pos.0[0] += -50.*diff_x/r;
                pos.0[1] += -50.*diff_y/r;
            }
        }

//        var rect1 = {x: 5, y: 5, width: 50, height: 50}
//        var rect2 = {x: 20, y: 10, width: 10, height: 10}
//
//        if (rect1.x < rect2.x + rect2.width &&
//            rect1.x + rect1.width > rect2.x &&
//            rect1.y < rect2.y + rect2.height &&
//            rect1.y + rect1.height > rect2.y) {
//            // collision detected!
//        }
    }
}

pub fn check_collision(game_pos1: GamePosition, coll_flag1: CollisionDetectionFlag, game_pos2: GamePosition, coll_flag2: CollisionDetectionFlag) -> bool {
    game_pos1.0[0] < game_pos2.0[0] + coll_flag2.0[0] &&
                game_pos1.0[0] + coll_flag1.0[0] > game_pos2.0[0] &&
                game_pos1.0[1] < game_pos2.0[1] + coll_flag2.0[1] &&
                game_pos1.0[1] + coll_flag1.0[1] > game_pos2.0[1]
}