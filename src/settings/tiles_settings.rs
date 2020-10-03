// resources/settings/tiles.ron

use super::EntityComponents;
use std::collections::HashMap;

pub type TileType = String;

#[derive(Deserialize, Clone)]
#[serde(from = "HashMap<TileType, TileSettings>", deny_unknown_fields)]
pub struct TilesSettings {
    pub tiles: HashMap<TileType, TileSettings>,
}

impl From<HashMap<TileType, TileSettings>> for TilesSettings {
    fn from(tiles: HashMap<TileType, TileSettings>) -> Self {
        Self { tiles }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TileSettings {
    pub components: EntityComponents,
}
