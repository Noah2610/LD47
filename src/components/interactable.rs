use super::component_prelude::*;

#[derive(Component, Default, Deserialize, Clone)]
#[storage(NullStorage)]
pub struct Interactable;
