use crate::game::{EnemyFlag, GamePosition, ItemType, LoadedSpriteSheet, NumEnemiesLeft, Player, PlayerEntity, TimeLeft, Animation, DEAD_ENEMY_FRAMES, RoomFlag, GameOver};
use crate::systems::check_next_room::spawn_item;
use amethyst::core::{timing::Time, Transform};
use amethyst::ecs::{
    Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage, Builder
};
use amethyst::renderer::{SpriteRender, Transparent};
use rand::{thread_rng, Rng};
use std::time::Duration;
use crate::audio::{AudioSystemData, play_enemy_die};

pub struct DecrementTime;

impl<'a> System<'a> for DecrementTime {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, TimeLeft>,
        WriteStorage<'a, Player>,
        Read<'a, PlayerEntity>,
        WriteStorage<'a, Animation>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_time, mut time_left, mut player_flag, player_ent, mut anim_store) = data;

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

        for anim in (&mut anim_store).join() {
            if anim.cooldown > Duration::from_secs(0) {
                if anim.cooldown > game_time.delta_time() {
                    anim.cooldown -= game_time.delta_time();
                } else {
                    anim.cooldown = Duration::from_secs(0);
                }
            }
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
        Read<'a, LazyUpdate>,
        ReadStorage<'a, GamePosition>,
        ReadExpect<'a, LoadedSpriteSheet>,
        WriteExpect<'a, GameOver>,
        AudioSystemData<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            time_left,
            enemies,
            player_ent,
            entities,
            mut num_enemies,
            lazy,
            game_poses,
            sprite_sheet,
            mut game_over,
            audio,
        ) = data;

        let zero = Duration::from_secs(0);
        for (time, _, ent, game_pos) in (&time_left, &enemies, &entities, &game_poses).join() {
            if time.remainaing == zero {
                play_enemy_die(&audio);
                entities.delete(ent).unwrap();
                num_enemies.0 -= 1;

                if thread_rng().gen_bool(0.25) {
                    spawn_item(
                        game_pos.to_tile(),
                        &lazy,
                        &entities,
                        &sprite_sheet,
                        ItemType::PlusTime(10),
                        0,
                    );
                }

                lazy.create_entity(&entities)
                    .with(SpriteRender {
                        sprite_sheet: sprite_sheet.0.clone().unwrap(),
                        sprite_number: 7,
                    })
                    .with(Animation::new(200, DEAD_ENEMY_FRAMES))
                    .with(RoomFlag)
                    .with(game_pos.clone())
                    .with(Transform::default())
                    .with(Transparent)
                    .build();
            }
        }

        let player_health = time_left.get(player_ent.0.unwrap()).unwrap();

        if player_health.remainaing == zero {
            game_over.0 = true;
            warn!("GAME OVER HERE");
        }
    }
}
