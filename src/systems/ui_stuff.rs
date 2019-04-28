use crate::game::{PlayerEntity, TimeLeft, UIEnts};
use amethyst::ecs::prelude::*;
use amethyst::ui::UiText;

pub struct UpdateTimer;

impl<'a> System<'a> for UpdateTimer {
    type SystemData = (
        ReadExpect<'a, UIEnts>,
        WriteStorage<'a, UiText>,
        ReadStorage<'a, TimeLeft>,
        ReadExpect<'a, PlayerEntity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (ui_ents, mut ui_text_store, time_left, player_ent) = data;

        let player_time = time_left.get(player_ent.0.unwrap()).unwrap();

        let ui_text = ui_text_store.get_mut(ui_ents.0).unwrap();

        ui_text.text = format!(
            "{}.{}",
            player_time.remainaing.as_secs(),
            player_time.remainaing.subsec_millis() / 100
        );

        if player_time.remainaing.as_secs() < 90 {
            ui_text.color = [1., 0.95, 0.211, 1.];
        }

        if player_time.remainaing.as_secs() < 45 {
            ui_text.color = [1., 0., 0., 1.];
        }
    }
}
