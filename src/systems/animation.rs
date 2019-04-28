use amethyst::ecs::prelude::*;
use amethyst::renderer::SpriteRender;
use crate::game::{Animation, LoadedSpriteSheet};
use std::time::Duration;

pub struct AnimateSprites;

impl<'a> System<'a> for AnimateSprites {
    type SystemData = (
        WriteStorage<'a, Animation>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut anim_store, mut sprite_store) = data;

        for (anim, sprite) in (&mut anim_store, &mut sprite_store).join() {
            if anim.cooldown == Duration::from_secs(0) {
                sprite.sprite_number = *anim.sprites.next().unwrap();
                anim.cooldown = anim.time_per_frame;
            }
        }
    }
}