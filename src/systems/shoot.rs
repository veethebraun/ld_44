use crate::game::{
    CollisionDetectionFlag, EnemyFlag, GamePosition, LoadedSpriteSheet, Player, PlayerEntity,
    ShootDirection, Shooter, TimeLeft, WallFlag,
};
use amethyst::core::timing::Time;
use amethyst::core::transform::Transform;
use amethyst::ecs::{
    Component, Entities, Join, LazyUpdate, NullStorage, Read, ReadStorage, System, VecStorage,
    WriteStorage,
};
use amethyst::input::InputHandler;
use std::time::Duration;

pub struct PlayerShoot;
use crate::systems::player_bad_guy_collide::check_collision;
use amethyst::assets::AssetStorage;
use amethyst::ecs::Builder;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};

impl<'a> System<'a> for PlayerShoot {
    type SystemData = (
        ReadStorage<'a, GamePosition>,
        WriteStorage<'a, Player>,
        Read<'a, PlayerEntity>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, LazyUpdate>,
        Read<'a, LoadedSpriteSheet>,
        Entities<'a>,
        WriteStorage<'a, Shooter>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            game_poses,
            mut player_flag,
            player_ent,
            input,
            lazy,
            sprite_sheet,
            entities,
            mut shooter_store,
        ) = data;
        //        let (game_poses, coll_flags, player_flag, player_ent, input, lazy) = data;

        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();
        let player_shooter = shooter_store.get_mut(player_ent.0.unwrap()).unwrap();
        let mut player = player_flag.get_mut(player_ent.0.unwrap()).unwrap();

        if player_shooter.current_cooldown == Duration::from_secs(0) {
            let shoot_direction = match (
                input.action_is_down("shoot_left"),
                input.action_is_down("shoot_right"),
                input.action_is_down("shoot_up"),
                input.action_is_down("shoot_down"),
            ) {
                (Some(true), Some(false), Some(false), Some(false)) => ShootDirection::Left,
                (Some(true), Some(false), Some(true), Some(false)) => ShootDirection::UpLeft,
                (Some(true), Some(false), Some(false), Some(true)) => ShootDirection::DownLeft,
                (Some(false), Some(true), Some(false), Some(false)) => ShootDirection::Right,
                (Some(false), Some(true), Some(true), Some(false)) => ShootDirection::UpRight,
                (Some(false), Some(true), Some(false), Some(true)) => ShootDirection::DownRight,
                (Some(false), Some(false), Some(false), Some(true)) => ShootDirection::Down,
                (Some(false), Some(false), Some(true), Some(false)) => ShootDirection::Up,
                _ => ShootDirection::None,
            };
            if shoot_direction != ShootDirection::None {
                player_shooter.set_cooldown();
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet.0.clone().unwrap(),
                    sprite_number: 15, // paddle is the first sprite in the sprite_sheet
                };

                let game_pos = player_pos.clone();
                let transform = Transform::default();

                let speed = player_shooter.speed;

                let speed = match shoot_direction {
                    ShootDirection::Left => [-1. * speed, 0.],
                    ShootDirection::Right => [1. * speed, 0.],
                    ShootDirection::Up => [0., 1. * speed],
                    ShootDirection::Down => [0., -1. * speed],
                    ShootDirection::UpLeft => [-0.707 * speed, 0.707 * speed],
                    ShootDirection::UpRight => [0.707 * speed, 0.707 * speed],
                    ShootDirection::DownLeft => [-0.707 * speed, -0.707 * speed],
                    ShootDirection::DownRight => [-0.707 * speed, -0.707 * speed],
                    _ => [0., 0.],
                };

                let player_speed_multi = 0.2;

                let speed = [
                    speed[0] + player_speed_multi * player.speed[0],
                    speed[1] + player_speed_multi * player.speed[1],
                ];

                let bullet = Bullet { speed };

                lazy.create_entity(&entities)
                    .with(sprite_render)
                    .with(game_pos)
                    .with(transform)
                    .with(bullet)
                    .with(PlayerBullet)
                    .with(CollisionDetectionFlag([10., 10.]))
                    .build();
            }
        }
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Bullet {
    speed: [f32; 2],
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct PlayerBullet;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct EnemyBullet;

pub struct DecrementPlayerCoolDowns;

impl<'a> System<'a> for DecrementPlayerCoolDowns {
    type SystemData = (WriteStorage<'a, Shooter>, Read<'a, Time>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut shooter_store, time) = data;
        for shooter in (&mut shooter_store).join() {
            if shooter.current_cooldown > time.delta_time() {
                shooter.current_cooldown -= time.delta_time();
            } else {
                shooter.current_cooldown = Duration::from_secs(0);
            }
        }
    }
}

pub struct MoveBullets;

impl<'a> System<'a> for MoveBullets {
    type SystemData = (ReadStorage<'a, Bullet>, WriteStorage<'a, GamePosition>);

    fn run(&mut self, data: Self::SystemData) {
        let (bullet_store, mut game_posses) = data;

        for (bullet, game_pos) in (&bullet_store, &mut game_posses).join() {
            game_pos.0[0] += bullet.speed[0];
            game_pos.0[1] += bullet.speed[1];
        }
    }
}

pub struct CheckBulletCollide;

impl<'a> System<'a> for CheckBulletCollide {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, GamePosition>,
        ReadStorage<'a, CollisionDetectionFlag>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, EnemyFlag>,
        WriteStorage<'a, TimeLeft>,
        WriteStorage<'a, Player>,
        Read<'a, PlayerEntity>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, WallFlag>,
        ReadStorage<'a, PlayerBullet>,
        ReadStorage<'a, EnemyBullet>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            game_poses,
            coll_flags,
            bullet_store,
            enemy_store,
            mut time_left,
            mut player_flag,
            player_ent,
            lazy,
            wall_flag,
            player_bullet_flag,
            enemy_bullet_flag,
        ) = data;

        let player = player_flag.get_mut(player_ent.0.unwrap()).unwrap();
        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();
        let player_coll = coll_flags.get(player_ent.0.unwrap()).unwrap().clone();

        for (ent, _, bullet_pos, bullet_coll, _) in (
            &entities,
            &bullet_store,
            &game_poses,
            &coll_flags,
            &player_bullet_flag,
        )
            .join()
        {
            for (_, enem_pos, health, enem_coll) in
                (&enemy_store, &game_poses, &mut time_left, &coll_flags).join()
            {
                if check_collision(*bullet_pos, *bullet_coll, *enem_pos, *enem_coll) {
                    entities.delete(ent).unwrap();

                    health.subtract(Duration::from_secs(player.damage));
                }
            }

            for (_, enem_pos, enem_coll) in (&wall_flag, &game_poses, &coll_flags).join() {
                if check_collision(*bullet_pos, *bullet_coll, *enem_pos, *enem_coll) {
                    entities.delete(ent).unwrap();
                }
            }
        }

        let player_time = time_left.get_mut(player_ent.0.unwrap()).unwrap();

        for (ent, _, bullet_pos, bullet_coll, _) in (
            &entities,
            &bullet_store,
            &game_poses,
            &coll_flags,
            &enemy_bullet_flag,
        )
            .join()
        {
            if check_collision(*bullet_pos, *bullet_coll, player_pos, player_coll) {
                entities.delete(ent).unwrap();

                if player.invincible_time == Duration::from_secs(0) {
                    player_time.subtract(Duration::from_secs(15));
                    player.invincible_time = Duration::from_secs(1);
                }
            }

            for (_, enem_pos, enem_coll) in (&wall_flag, &game_poses, &coll_flags).join() {
                if check_collision(*bullet_pos, *bullet_coll, *enem_pos, *enem_coll) {
                    entities.delete(ent).unwrap();
                }
            }
        }
    }
}

pub struct EnemyShoot;

impl<'a> System<'a> for EnemyShoot {
    type SystemData = (
        ReadStorage<'a, GamePosition>,
        Read<'a, PlayerEntity>,
        Read<'a, LazyUpdate>,
        Read<'a, LoadedSpriteSheet>,
        WriteStorage<'a, Shooter>,
        ReadStorage<'a, EnemyFlag>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_poses, player_ent, lazy, sprite_sheet, mut shooter_store, enemy_store, entities) =
            data;

        let player_pos = game_poses.get(player_ent.0.unwrap()).unwrap().clone();

        for (_, enemy_pos, shooter) in (&enemy_store, &game_poses, &mut shooter_store).join() {
            if shooter.current_cooldown == Duration::from_secs(0) {
                shooter.set_cooldown();
                let sprite_render = SpriteRender {
                    sprite_sheet: sprite_sheet.0.clone().unwrap(),
                    sprite_number: 10, // paddle is the first sprite in the sprite_sheet
                };

                let game_pos = enemy_pos.clone();
                let transform = Transform::default();

                let speed = shooter.speed;

                let pos_diff = [
                    -game_pos.0[0] + player_pos.0[0],
                    -game_pos.0[1] + player_pos.0[1],
                ];
                let r = (pos_diff[0] * pos_diff[0] + pos_diff[1] * pos_diff[1]).sqrt();

                let speed = [speed * pos_diff[0] / r, speed * pos_diff[1] / r];

                let bullet = Bullet { speed };

                lazy.create_entity(&entities)
                    .with(sprite_render)
                    .with(game_pos)
                    .with(transform)
                    .with(bullet)
                    .with(EnemyBullet)
                    .with(CollisionDetectionFlag([10., 10.]))
                    .build();
            }
        }
    }
}
