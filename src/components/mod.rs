mod interactable;
mod movement;
mod player;

pub mod prelude {
    pub use super::interactable::{Interactable, InteractableType};
    pub use super::movement::Movement;
    pub use super::player::Player;

    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}
