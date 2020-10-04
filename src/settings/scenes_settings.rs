// resources/settings/scenes.ron

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct ScenesSettings {
    pub scenes: Vec<SceneSettings>,
}

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SceneSettings {
    #[serde(alias = "level")]
    pub level_filename: String,
}
