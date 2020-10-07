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
use deathframe::components::prelude::Merge;
use prelude::*;
use std::fmt;
use std::fs::File;
use std::path::PathBuf;

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
            objects: load_settings_dir("objects")?,
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

// Functions below copied from deathfloor project.

fn load_settings_dir<T, S>(dirname: S) -> amethyst::Result<T>
where
    for<'de> T: serde::Deserialize<'de> + Merge + Default,
    S: std::fmt::Display,
{
    let path = resource(format!("settings/{}", dirname));
    let errmsg = format!("No settings files found in {:?}", &path);
    let all_settings = load_configs_recursively_from(path)?;
    let merged_settings = merge_settings(all_settings).unwrap_or_else(|| {
        eprintln!(
            "[WARNING]\n    {}\n    Using default (probably empty settings)",
            errmsg
        );
        T::default()
    });
    Ok(merged_settings)
}

fn load_configs_recursively_from<T>(path: PathBuf) -> amethyst::Result<Vec<T>>
where
    for<'de> T: serde::Deserialize<'de> + Merge,
{
    let mut settings = Vec::new();

    for entry in path.read_dir()? {
        let entry_path = entry?.path();
        if entry_path.is_file() {
            if let Some("ron") = entry_path.extension().and_then(|e| e.to_str())
            {
                let file = File::open(&entry_path)?;
                settings.push(ron::de::from_reader(file).map_err(|e| {
                    amethyst::Error::from_string(format!(
                        "Failed parsing RON settings file: {:?}\n{:#?}",
                        entry_path, e
                    ))
                })?);
            }
        } else if entry_path.is_dir() {
            settings.append(&mut load_configs_recursively_from(entry_path)?);
        }
    }

    Ok(settings)
}

/// Merge `Vec` of settings `T` together.
/// Returns `None` if given `Vec` is empty.
fn merge_settings<T>(all_settings: Vec<T>) -> Option<T>
where
    T: Merge,
{
    let mut merged_settings: Option<T> = None;
    for settings in all_settings {
        if let Some(merged) = merged_settings.as_mut() {
            merged.merge(settings);
        } else {
            merged_settings = Some(settings);
        }
    }
    merged_settings
}
