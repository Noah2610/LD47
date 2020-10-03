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
    pub tiles: HashMap<String, ObjectSettings>,
}

impl From<HashMap<String, ObjectSettings>> for ObjectsSettings {
    fn from(tiles: HashMap<String, ObjectSettings>) -> Self {
        Self { tiles }
    }
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ObjectSettings {
    pub components: EntityComponents,
}
