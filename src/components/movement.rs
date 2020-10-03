use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Movement {
    pub acceleration: f32,
    pub max_velocity: f32,
}
