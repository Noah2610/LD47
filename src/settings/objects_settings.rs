// resources/settings/objects.ron

use super::EntityComponents;
use std::collections::HashMap;

#[derive(Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Player,
    Custom(String),
}

#[derive(Deserialize, Clone)]
#[serde(from = "HashMap<String, ObjectSettings>", deny_unknown_fields)]
pub struct ObjectsSettings {
    pub objects: HashMap<String, ObjectSettings>,
}

impl From<HashMap<String, ObjectSettings>> for ObjectsSettings {
    fn from(objects: HashMap<String, ObjectSettings>) -> Self {
        Self { objects }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ObjectSettings {
    #[serde(alias = "spritesheet")]
    pub spritesheet_filename: String,
    pub components:           EntityComponents,
}
