pub struct GamePositionTransform;

use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use crate::game::{ARENA_WIDTH, ARENA_HEIGHT, GamePosition, Player};
use amethyst::core::Transform;


impl<'a> System<'a> for GamePositionTransform {
    type SystemData = (
        ReadStorage<'a, GamePosition>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_poses, mut transforms, player_flag)  = data;

        let mut camera = [ARENA_WIDTH/2., ARENA_HEIGHT/2.];

        for (_, game_pos) in (&player_flag, &game_poses).join() {
            camera[0] = ARENA_WIDTH/2. - game_pos.0[0];
            camera[1] = ARENA_HEIGHT/2. - game_pos.0[1];
        }

        for (game_pos, transform) in (&game_poses, &mut transforms).join() {
            let x = game_pos.0[0];
            let y = game_pos.0[1];

            transform.set_x(x + camera[0]);
            transform.set_y(y + camera[1]);
        }
    }
}