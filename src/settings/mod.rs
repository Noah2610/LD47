pub mod prelude {
    pub use super::audio_settings::AudioSettings;
    pub use super::camera_settings::CameraSettings;
    pub use super::entity_components::*;
    pub use super::general_settings::GeneralSettings;
    pub use super::objects_settings::{ObjectType, ObjectsSettings};
    pub use super::player_settings::PlayerSettings;
    pub use super::scenes_settings::{SceneSettings, ScenesSettings};
    pub use super::tiles_settings::{TileType, TilesSettings};
    pub use super::Settings;
}

pub mod audio_settings;
pub mod camera_settings;
pub mod entity_components;
pub mod general_settings;
pub mod hitbox_config;
pub mod objects_settings;
pub mod player_settings;
pub mod scenes_settings;
pub mod tiles_settings;

use crate::resource;
use deathframe::amethyst;
use prelude::*;
use std::fmt;
use std::fs::File;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Settings {
    pub general: GeneralSettings,
    pub player:  PlayerSettings,
    pub camera:  CameraSettings,
    pub scenes:  ScenesSettings,
    pub tiles:   TilesSettings,
    pub objects: ObjectsSettings,
    pub audio:   AudioSettings,
}

impl Settings {
    pub fn load() -> amethyst::Result<Self> {
        Ok(Self {
            general: load_settings("general.ron")?,
            player:  load_settings("player.ron")?,
            camera:  load_settings("camera.ron")?,
            scenes:  load_settings("scenes.ron")?,
            tiles:   load_settings("tiles.ron")?,
            objects: load_settings("objects.ron")?,
            audio:   load_settings("audio.ron")?,
        })
    }
}

fn load_settings<S, P>(path: P) -> amethyst::Result<S>
where
    for<'de> S: serde::Deserialize<'de>,
    P: fmt::Display,
{
    let file = File::open(resource(format!("settings/{}", &path)))?;
    Ok(ron::de::from_reader(file).map_err(|e| {
        amethyst::Error::from_string(format!(
            "Failed parsing RON settings file: {}\n{:#?}",
            path, e
        ))
    })?)
}
