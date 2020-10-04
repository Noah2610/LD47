// resources/settings/audio.ron

use crate::resources::{SongKey, SoundKey};
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioSettings {
    pub bgm: AudioBgmSettings,
    pub sfx: AudioSfxSettings,
}

#[derive(Deserialize, Clone)]
#[serde(from = "HashMap<SongKey, AudioSongSettings>", deny_unknown_fields)]
pub struct AudioBgmSettings {
    pub songs: HashMap<SongKey, AudioSongSettings>,
}

impl From<HashMap<SongKey, AudioSongSettings>> for AudioBgmSettings {
    fn from(songs: HashMap<SongKey, AudioSongSettings>) -> Self {
        Self { songs }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioSongSettings {
    pub file:        String,
    pub should_loop: bool,
}

#[derive(Deserialize, Clone)]
#[serde(from = "HashMap<SoundKey, AudioSoundSettings>", deny_unknown_fields)]
pub struct AudioSfxSettings {
    pub sounds: HashMap<SoundKey, AudioSoundSettings>,
}

impl From<HashMap<SoundKey, AudioSoundSettings>> for AudioSfxSettings {
    fn from(sounds: HashMap<SoundKey, AudioSoundSettings>) -> Self {
        Self { sounds }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct AudioSoundSettings {
    pub file: String,
}
