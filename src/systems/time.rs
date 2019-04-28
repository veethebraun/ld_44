use crate::game::{EnemyFlag, NumEnemiesLeft, Player, PlayerEntity, TimeLeft};
use amethyst::core::timing::Time;
use amethyst::ecs::{Entities, Join, Read, ReadStorage, System, WriteExpect, WriteStorage};
use std::time::Duration;

pub struct DecrementTime;

impl<'a> System<'a> for DecrementTime {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, TimeLeft>,
        WriteStorage<'a, Player>,
        Read<'a, PlayerEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_time, mut time_left, mut player_flag, player_ent) = data;

        let player = player_flag.get_mut(player_ent.0.unwrap()).unwrap();
        if player.invincible_time > Duration::from_secs(0) {
            if player.invincible_time > game_time.delta_time() {
                player.invincible_time -= game_time.delta_time();
            } else {
                player.invincible_time = Duration::from_secs(0);
            }
        }

        for (time) in (&mut time_left).join() {
            time.subtract(game_time.delta_time());
        }
    }
}

pub struct RemoveOutOfTimeBadGuys;

impl<'a> System<'a> for RemoveOutOfTimeBadGuys {
    type SystemData = (
        ReadStorage<'a, TimeLeft>,
        ReadStorage<'a, EnemyFlag>,
        Read<'a, PlayerEntity>,
        Entities<'a>,
        WriteExpect<'a, NumEnemiesLeft>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time_left, enemies, player_ent, entities, mut num_enemies) = data;

        let zero = Duration::from_secs(0);
        for (time, _, ent) in (&time_left, &enemies, &entities).join() {
            if time.remainaing == zero {
                entities.delete(ent).unwrap();
                num_enemies.0 -= 1;
            }
        }

        let player_health = time_left.get(player_ent.0.unwrap()).unwrap();

        if player_health.remainaing == zero {
            warn!("GAME OVER HERE");
        }
    }
}
