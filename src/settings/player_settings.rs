// resources/settings/player.ron

use super::EntityComponents;
use crate::components::prelude::EventsRegister;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct PlayerSettings {
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: String,
    pub components:           EntityComponents,
    #[serde(default, alias = "events")]
    pub events_register:      Option<EventsRegister>,
}
