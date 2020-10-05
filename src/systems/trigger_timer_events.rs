use super::system_prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct TriggerTimerEventsSystem;

impl<'a> System<'a> for TriggerTimerEventsSystem {
    type SystemData = WriteStorage<'a, EventsRegister>;

    fn run(&mut self, mut events_register_store: Self::SystemData) {
        for events_register in (&mut events_register_store).join() {
            let mut finished_timers = HashSet::new();
            for (timer_name, timer) in &mut events_register.timers {
                timer.update().unwrap();
                if timer.state.is_finished() {
                    finished_timers.insert(timer_name.clone());
                }
            }
            for finished_timer_name in finished_timers {
                let _ = events_register.timers.remove(&finished_timer_name);
                events_register.trigger_event(&EventType::OnTimerFinish(
                    finished_timer_name,
                ));
            }
        }
    }
}
