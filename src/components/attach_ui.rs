use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(deny_unknown_fields)]
pub struct AttachUi {
    #[serde(alias = "target")]
    pub target_id: String,
    pub offset:    (f32, f32),
}
