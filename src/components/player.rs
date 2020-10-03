use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub interacting_with: Option<InteractableType>,
}

impl Player {
    pub fn is_in_control(&self) -> bool {
        self.interacting_with.is_none()
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            interacting_with: None,
        }
    }
}
