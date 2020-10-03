pub mod prelude {
    pub use super::action_type::ActionType;
    pub use super::event_type::EventType;
    pub use super::EventsRegister;
}

mod action_type;
mod event_type;

use super::component_prelude::*;
use std::collections::HashMap;

#[derive(Component, Deserialize, Clone)]
pub struct EventsRegister {
    pub events:        HashMap<EventType, Vec<ActionType>>,
    #[serde(skip)]
    triggered_actions: Vec<ActionType>,
}

impl EventsRegister {
    pub fn trigger_event(&mut self, event: &EventType) {
        if let Some(mut actions) = self.events.get(event).cloned() {
            self.triggered_actions.append(&mut actions);
        }
    }
}

impl ActionQueue for EventsRegister {
    type Action = ActionType;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.triggered_actions
    }
}
