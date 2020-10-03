use super::component_prelude::*;

#[derive(Component, Deserialize, Clone)]
#[storage(VecStorage)]
#[serde(from = "InteractableType", deny_unknown_fields)]
pub struct Interactable {
    #[serde(alias = "type")]
    pub interactable_type: InteractableType,
}

impl From<InteractableType> for Interactable {
    fn from(interactable_type: InteractableType) -> Self {
        Self { interactable_type }
    }
}

#[derive(Deserialize, Clone)]
pub enum InteractableType {
    Alarm,
}
