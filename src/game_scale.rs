// Num Enemies
// Type Enemies
// Health Enemies
// Speed Enemies
// Cost Power ups




use rand::{thread_rng, distributions::{Distribution, Poisson}, Rng};

use crate::game::Enemies;

pub fn gen_enemy_list(floor: u32) -> Vec<Enemies> {
    let mut rng = thread_rng();
    let num_enemies: u64 = Poisson::new(floor as f64 + 0.5).sample(&mut rng).max((floor/2) as u64 + 1);

    let mut out = Vec::new();
    for _ in 0..num_enemies {
        let enemy_type = {
            if rng.gen_bool(0.35) {
                Enemies::Stationary
            } else if rng.gen_bool(0.35) {
                Enemies::NoShoot
            } else {
                Enemies::Full
            }
        };

       out.push(enemy_type)
    }
    out
}


pub fn get_enemy_time(floor: u32) -> u64 {
    (floor as u64) * 5 + 25
}

pub fn get_enemy_speed(floor: u32) -> f32 {
    if floor > 5 {
        2. + ( (floor-5) as f32 )*0.3
    } else {
        2.
    }
}

pub fn get_powerup_cost(floor: u32) -> u64 {
    5*(floor/5) as u64 + 10
}

pub fn get_enemy_bullet_speed(floor: u32) -> f32 {
    (floor/3) as f32 + 3.
}