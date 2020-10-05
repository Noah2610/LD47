pub mod prelude {
    pub use super::action_type::ActionType;
    pub use super::event_type::EventType;
    pub use super::variable_value::VariableValue;
    pub use super::EventsRegister;
}

mod action_type;
mod event_type;
mod variable_value;

use super::component_prelude::*;
use climer::Timer;
use std::collections::HashMap;

type EventsActionsMap = HashMap<EventType, Vec<ActionType>>;

#[derive(Component, Deserialize, Clone)]
#[serde(from = "EventsActionsMap", deny_unknown_fields)]
pub struct EventsRegister {
    pub events:            EventsActionsMap,
    #[serde(skip)]
    pub timers:            HashMap<String, Timer>,
    #[serde(skip)]
    pub variables:         HashMap<String, VariableValue>,
    #[serde(skip)]
    pub triggered_actions: Vec<ActionType>,
}

impl EventsRegister {
    pub fn trigger_event(&mut self, event: &EventType) {
        if let Some(mut actions) = self.events.get(event).cloned() {
            self.triggered_actions.append(&mut actions);
        }
    }
}

impl From<EventsActionsMap> for EventsRegister {
    fn from(events: EventsActionsMap) -> Self {
        Self {
            events,
            timers: HashMap::new(),
            variables: HashMap::new(),
            triggered_actions: Vec::new(),
        }
    }
}

impl ActionQueue for EventsRegister {
    type Action = ActionType;
    fn mut_actions(&mut self) -> &mut Vec<Self::Action> {
        &mut self.triggered_actions
    }
}
