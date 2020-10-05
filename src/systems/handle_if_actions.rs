use super::system_prelude::*;

#[derive(Default)]
pub struct HandleIfActionsSystem;

impl<'a> System<'a> for HandleIfActionsSystem {
    type SystemData = (
        Entities<'a>,
        IfStorages<'a>,
        WriteStorage<'a, IfActions>,
        WriteStorage<'a, EventsRegister>,
    );

    fn run(
        &mut self,
        (
            entities,
            if_stores,
            mut if_actions_store,
            mut events_register_store,
        ): Self::SystemData,
    ) {
        for (entity, if_actions, events_register) in
            (&entities, &mut if_actions_store, &mut events_register_store)
                .join()
        {
            for mut action in if_actions.drain_actions() {
                if action.condition.passes(entity, &if_stores) {
                    events_register
                        .triggered_actions
                        .append(&mut action.success);
                } else {
                    if let Some(mut failure) = action.failure {
                        events_register.triggered_actions.append(&mut failure);
                    }
                }
            }
        }
    }
}
