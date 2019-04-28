use amethyst::{
    assets::{AssetStorage, Loader},
    audio::{output::Output, AudioSink, OggFormat, Source, SourceHandle},
    ecs::prelude::*,
    shred::{Read, ResourceId, Resources, SystemData},
};

pub const AUDIO_MUSIC: &[&str] =  &["sounds/ld44_musac.ogg"];

use std::{iter::Cycle, vec::IntoIter};

pub struct AudioSystemData<'s> {
    sources: Read<'s, AssetStorage<Source>>,
    sounds: ReadExpect<'s, Sounds>,
    outputs: Option<Read<'s, Output>>,
}

pub struct Sounds {
    //pub score_sfx: SourceHandle,
    pub player_shoot: SourceHandle,
    pub enemy_shoot: SourceHandle,
    pub enemy_die: SourceHandle,
    pub player_hit: SourceHandle,
    pub enemy_hit: SourceHandle,
}

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

impl Default for Music {
    fn default() -> Self {
        Music {
            music: vec![].into_iter().cycle(),
        }
    }
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), (), &world.read_resource())
}

/// Initialise audio in the world. This includes the background track and the
/// sound effects.
pub fn initialise_audio(world: &mut World) {
    //use crate::{AUDIO_BOUNCE, AUDIO_MUSIC, AUDIO_SCORE};

    let (sound_effects, music) = {
        let loader = world.read_resource::<Loader>();

        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

//        let music = ();
                let music = AUDIO_MUSIC
                    .iter()
                    .map(|file| load_audio_track(&loader, &world, file))
                    .collect::<Vec<_>>()
                    .into_iter()
                    .cycle();
                let music = Music { music };

        let sound = Sounds {
            player_shoot: load_audio_track(&loader, &world, "sounds/tic.ogg"),
            enemy_shoot:  load_audio_track(&loader, &world, "sounds/toc.ogg"),
            player_hit:  load_audio_track(&loader, &world, "sounds/player_hitt.ogg"),
            enemy_die:  load_audio_track(&loader, &world, "sounds/enemy_hit.ogg"),
            enemy_hit:  load_audio_track(&loader, &world, "sounds/enemy_r_hit.ogg"),
            //score_sfx: load_audio_track(&loader, &world, AUDIO_SCORE),
        };

        (sound, music)
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.add_resource(sound_effects);
    world.add_resource(music);
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_player_shoot(audio_system_data: &AudioSystemData) {
    let sounds = &audio_system_data.sounds;
    let storage = &audio_system_data.sources;
    let output = &audio_system_data.outputs;
    //                       sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.player_shoot) {
            output.play_once(sound, 0.2);
        }
    }
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_enemy_die(audio_system_data: &AudioSystemData) {
    let sounds = &audio_system_data.sounds;
    let storage = &audio_system_data.sources;
    let output = &audio_system_data.outputs;
    //                       sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.enemy_die) {
            output.play_once(sound, 0.2);
        }
    }
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_player_hit(audio_system_data: &AudioSystemData) {
    let sounds = &audio_system_data.sounds;
    let storage = &audio_system_data.sources;
    let output = &audio_system_data.outputs;
    //                       sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.player_hit) {
            output.play_once(sound, 0.5);
        }
    }
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_enemy_hit(audio_system_data: &AudioSystemData) {
    let sounds = &audio_system_data.sounds;
    let storage = &audio_system_data.sources;
    let output = &audio_system_data.outputs;
    //                       sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.enemy_hit) {
            output.play_once(sound, 0.5);
        }
    }
}

/// Plays the bounce sound when a ball hits a side or a paddle.
pub fn play_enemy_shoot(audio_system_data: &AudioSystemData) {
    let sounds = &audio_system_data.sounds;
    let storage = &audio_system_data.sources;
    let output = &audio_system_data.outputs;
    //                       sounds: &Sounds, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(&sounds.enemy_shoot) {
            output.play_once(sound, 0.2);
        }
    }
}

impl<'s> SystemData<'s> for AudioSystemData<'s> {
    fn setup(res: &mut Resources) {
        <Read<'s, AssetStorage<Source>> as SystemData>::setup(&mut *res);
        <ReadExpect<'s, Sounds> as SystemData>::setup(&mut *res);
        <Option<Read<'s, Output>> as SystemData>::setup(&mut *res);
    }

    fn fetch(res: &'s Resources) -> Self {
        AudioSystemData {
            sources: <Read<'s, AssetStorage<Source>> as SystemData>::fetch(res),
            sounds: <ReadExpect<'s, Sounds> as SystemData>::fetch(res),
            outputs: <Option<Read<'s, Output>> as SystemData>::fetch(res),
        }
    }

    fn reads() -> Vec<ResourceId> {
        let mut r = Vec::new();

        let mut reads = <Read<'s, AssetStorage<Source>> as SystemData>::reads();
        r.append(&mut reads);
        let mut reads = <ReadExpect<'s, Sounds> as SystemData>::reads();
        r.append(&mut reads);
        let mut reads = <Option<Read<'s, Output>> as SystemData>::reads();
        r.append(&mut reads);

        r
    }

    fn writes() -> Vec<ResourceId> {
        let mut r = Vec::new();

        let mut writes = <Read<'s, AssetStorage<Source>> as SystemData>::writes();
        r.append(&mut writes);
        let mut writes = <ReadExpect<'s, Sounds> as SystemData>::writes();
        r.append(&mut writes);
        let mut writes = <Option<Read<'s, Output>> as SystemData>::writes();
        r.append(&mut writes);

        r
    }
}
