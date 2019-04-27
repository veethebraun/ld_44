

use amethyst::input::InputHandler;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use crate::game::{Player, GamePosition};

pub struct MovePlayerSystem;

impl<'a> System<'a> for MovePlayerSystem {
    type SystemData = (

    WriteStorage<'a, GamePosition>,
    ReadStorage<'a, Player>,
    Read<'a, InputHandler<String, String>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut game_poses, player_flag, input) = data;
        for (_, game_pos) in (&player_flag, &mut game_poses).join() {
            let x_movement = input.axis_value("player_leftright");
            let y_movement = input.axis_value("player_updown");
            if let Some(mv_amount) = y_movement {
                let scaled_amount = 5.2 * mv_amount as f32;
                let player_y = game_pos.0[1];

                game_pos.0[1] = player_y + scaled_amount;
            }
            if let Some(mv_amount) = x_movement {
                let scaled_amount = 5.2 * mv_amount as f32;
                let player_x = game_pos.0[0];

                game_pos.0[0] = player_x + scaled_amount;
            }
            //println!("{:?}", input.action_is_down("hi"));
        }


    }
}