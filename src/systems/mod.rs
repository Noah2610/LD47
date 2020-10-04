mod control_player;
mod handle_events_actions;
mod handle_fade;
mod handle_screen_shake;
mod trigger_collision_events;
mod trigger_init_events;
mod trigger_interaction_events;
mod trigger_timer_events;
mod update_animations;
mod update_text_output;

pub mod prelude {
    pub use super::control_player::ControlPlayerSystem;
    pub use super::handle_events_actions::HandleEventsActionsSystem;
    pub use super::handle_fade::HandleFadeSystem;
    pub use super::handle_screen_shake::HandleScreenShakeSystem;
    pub use super::trigger_collision_events::TriggerCollisionEventsSystem;
    pub use super::trigger_init_events::TriggerInitEventsSystem;
    pub use super::trigger_interaction_events::TriggerInteractionEventsSystem;
    pub use super::trigger_timer_events::TriggerTimerEventsSystem;
    pub use super::update_animations::UpdateAnimationsSystem;
    pub use super::update_text_output::UpdateTextOutputSystem;

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
