use crate::settings::prelude::ObjectType;

#[derive(Default)]
pub struct ObjectSpawner {
    pub to_spawn: Vec<ObjectSpawnData>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ObjectSpawnData {
    pub object:   ObjectType,
    #[serde(rename = "offset")]
    pub position: (f32, f32, f32),
    pub size:     (f32, f32),
}
