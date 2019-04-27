use amethyst::core::timing::Time;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use crate::game::TimeLeft;
use std::time::Duration;

pub struct DecrementTime;

impl<'a> System<'a> for DecrementTime {
    type SystemData = (
        Read<'a, Time>,
        WriteStorage<'a, TimeLeft>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (game_time, mut time_left) = data;

        warn!("{:?}", game_time.delta_time());
        for (time) in (&mut time_left).join() {
            time.subtract(game_time.delta_time());
            warn!("{:?}", time);
        }
    }
}