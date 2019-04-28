use crate::game::{GameMapTile,GAME_MAP_X,GAME_MAP_Y};
use itertools::iproduct;

type game_map_base = [[u32; GAME_MAP_Y]; GAME_MAP_X];
pub type game_map_tiles = [[GameMapTile; GAME_MAP_Y]; GAME_MAP_X];

pub const NUM_LEVELS: usize = 3;
pub const LEVELS: &[game_map_base; NUM_LEVELS] = &[LEVEL1,LEVEL2,LEVEL3];

pub const LEVEL1: game_map_base = [
    [ 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,4,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,3,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,],
    [ 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
];

pub fn turn_into_game_map(base: game_map_base) -> game_map_tiles {
    let mut end = [[GameMapTile::Nothing; GAME_MAP_Y]; GAME_MAP_X];
    for x in 0..GAME_MAP_X {
        for y in 0..GAME_MAP_Y {
            end[x][y] = match base[x][y] {
                0 => GameMapTile::Nothing,
                1 => GameMapTile::Wall,
                2 => GameMapTile::Floor,
                3 => GameMapTile::PlayerStart,
                4 => GameMapTile::Teleport,
                _ => GameMapTile::Nothing,
            }
        }
    }
    end
}

pub const LEVEL2: game_map_base = [
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,3,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,1,1,2,2,2,2,2,1,1,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,0,1,2,2,2,2,2,1,0,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,1,1,2,2,2,2,2,1,1,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,4,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,1,1,2,2,2,2,2,1,1,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,0,1,2,2,2,2,2,1,0,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,1,1,1,2,2,2,2,2,1,1,1,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,1,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],    
];



pub const LEVEL3: game_map_base = [
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,1,1,1,1,1,1,1,1,1,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,1,1,1,1,1,1,1,1,1,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,0,0,0,0,0,0,0,0,0,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,1,1,1,1,1,1,1,1,1,1,1,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,0,0,0,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,1,1,1,1,1,1,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,4,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,1,2,2,2,2,2,2,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,3,2,2,1,0,0,0,0,0,0,1,1,1,1,1,1,1,1,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,2,2,2,2,2,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
    [ 0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,],
];

