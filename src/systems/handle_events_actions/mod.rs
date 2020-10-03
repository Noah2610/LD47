use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleEventsActionsSystem;

impl<'a> System<'a> for HandleEventsActionsSystem {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Object>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            object_store,
            mut player_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let mut trigger_foreign_actions = HashMap::new();

        for (events_register, mut player_opt, _) in (
            &mut events_register_store,
            (&mut player_store).maybe(),
            !&unloaded_store,
        )
            .join()
        {
            for action in events_register.drain_actions() {
                match action {
                    ActionType::Echo(msg) => println!("> {}", msg),
                    ActionType::ForeignObjectAction(
                        object_type,
                        mut foreign_actions,
                    ) => {
                        trigger_foreign_actions
                            .entry(object_type)
                            .or_insert_with(Vec::new)
                            .append(&mut foreign_actions);
                    }
                    ActionType::SetControllable(controllable) => {
                        player_opt
                            .as_mut()
                            .expect(
                                "SetControllable action only works with Player",
                            )
                            .in_control = controllable;
                    }
                }
            }
        }

        for (object_type, actions) in trigger_foreign_actions {
            for (object, events_register) in
                (&object_store, &mut events_register_store).join()
            {
                if &object.object_type == &object_type {
                    for action in actions.clone() {
                        events_register.add_action(action);
                    }
                }
            }
        }
    }
}
