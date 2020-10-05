use super::system_prelude::*;
use deathframe::core::resources::input_manager::ActionState;

#[derive(Default)]
pub struct TriggerOnKeyEventsSystem;

impl<'a> System<'a> for TriggerOnKeyEventsSystem {
    type SystemData = (
        Read<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            input_manager,
            mut events_register_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for action_state in
            &[ActionState::Down, ActionState::Up, ActionState::Pressed]
        {
            input_manager.actions_for_each(action_state.clone(), |key| {
                let event = match action_state {
                    ActionState::Down => EventType::OnKeyDown(key.clone()),
                    ActionState::Up => EventType::OnKeyUp(key.clone()),
                    ActionState::Pressed => {
                        EventType::OnKeyPressed(key.clone())
                    }
                    _ => unreachable!(),
                };
                for (events_register, _) in
                    (&mut events_register_store, !&unloaded_store).join()
                {
                    events_register.trigger_event(&event);
                }
            });
        }
    }
}
