mod control_player;
mod handle_events_actions;
mod trigger_collision_events;
mod trigger_interaction_events;

pub mod prelude {
    pub use super::control_player::ControlPlayerSystem;
    pub use super::handle_events_actions::HandleEventsActionsSystem;
    pub use super::trigger_collision_events::TriggerCollisionEventsSystem;
    pub use super::trigger_interaction_events::TriggerInteractionEventsSystem;

    pub use deathframe::systems::prelude::*;
}

mod system_prelude {
    pub use crate::components::prelude::*;
    pub use crate::input::prelude::*;
    pub use crate::resources::*;
    pub use crate::settings::prelude::*;
    pub use deathframe::core::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;
}
