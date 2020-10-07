// resources/settings/objects.ron

use super::EntityComponents;
use crate::components::prelude::EventsRegister;
use deathframe::components::prelude::Merge;
use std::collections::HashMap;

#[derive(Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ObjectType {
    Player,
    Custom(String),
}

#[derive(Deserialize, Clone, Default)]
#[serde(from = "HashMap<String, ObjectSettings>", deny_unknown_fields)]
pub struct ObjectsSettings {
    pub objects: HashMap<String, ObjectSettings>,
}

impl From<HashMap<String, ObjectSettings>> for ObjectsSettings {
    fn from(objects: HashMap<String, ObjectSettings>) -> Self {
        Self { objects }
    }
}

impl Merge for ObjectsSettings {
    fn merge(&mut self, other: Self) {
        (&mut self.objects).extend(other.objects)
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ObjectSettings {
    #[serde(alias = "spritesheet", default)]
    pub spritesheet_filename: Option<String>,
    #[serde(default)]
    pub components:           Option<EntityComponents>,
    #[serde(default, alias = "events")]
    pub events_register:      Option<EventsRegister>,
}
