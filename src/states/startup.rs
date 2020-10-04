use super::state_prelude::*;
use std::path::PathBuf;

#[derive(Default)]
pub struct Startup;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Startup {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        insert_resources(&mut data.world);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);
        Trans::Switch(Box::new(MainMenu::default()))
    }
}

fn insert_resources(world: &mut World) {
    let sprite_sheet_handles = SpriteSheetHandles::<PathBuf>::default();
    world.insert(sprite_sheet_handles);

    world.insert(ScreenShakeRes::default());
    world.insert(FadeRes::default());

    let scenes_settings = (*world.read_resource::<ScenesSettings>()).clone();
    world.insert(SceneManager::from(scenes_settings));

    let audio_settings = (*world.read_resource::<AudioSettings>()).clone();

    let mut songs = Songs::<SongKey>::default();
    for (song_key, song) in audio_settings.bgm.songs {
        songs
            .load_audio(
                song_key,
                resource(format!("audio/bgm/{}", song.file)),
                song.should_loop,
                world,
            )
            .unwrap();
    }
    world.insert(songs);

    let mut sounds = Sounds::<SoundKey>::default();
    for (sound_key, sound) in audio_settings.sfx.sounds {
        sounds
            .load_audio(
                sound_key,
                resource(format!("audio/sfx/{}", sound.file)),
                world,
            )
            .unwrap();
    }
    world.insert(sounds);

    world
        .insert(crate::components::prelude::SoundPlayer::<SoundKey>::default());
}
