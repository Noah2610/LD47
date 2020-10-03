use super::component_prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {
    pub in_control: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self { in_control: true }
    }
}

impl Player {
    pub fn is_in_control(&self) -> bool {
        self.in_control
    }
}
