// resources/settings/player.ron

use super::EntityComponents;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlayerSettings {
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: String,
    pub components:           EntityComponents,
}
