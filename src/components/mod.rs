mod events_register;
mod if_actions;
mod interactable;
mod movement;
mod object;
mod player;
mod text_lines;
mod variables_register;

pub mod prelude {
    pub use super::events_register::prelude::*;
    pub use super::if_actions::prelude::*;
    pub use super::interactable::Interactable;
    pub use super::movement::Movement;
    pub use super::object::Object;
    pub use super::player::Player;
    pub use super::text_lines::{TextLines, TextLinesBehavior, TextLinesGroup};
    pub use super::variables_register::prelude::*;

    pub use deathframe::components::prelude::*;
}

mod component_prelude {
    pub use super::prelude::*;
    pub use deathframe::components::component_prelude::*;
}
