use super::system_prelude::*;
use deathframe::physics::query::prelude::{FindQuery, Query};

#[derive(Default)]
pub struct TriggerCollisionEventsSystem;

impl<'a> System<'a> for TriggerCollisionEventsSystem {
    type SystemData = (
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Collider<CollisionTag>>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            mut events_register_store,
            collider_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        for (events_register, collider, _) in (
            &mut events_register_store,
            &collider_store,
            !&unloaded_store,
        )
            .join()
        {
            let mut trigger_actions = Vec::new();

            for (event_type, action_types) in &events_register.events {
                if let EventType::OnCollision(query) = event_type {
                    if collider
                        .query::<FindQuery<_>>()
                        .exp(query)
                        .run()
                        .is_some()
                    {
                        trigger_actions.append(&mut action_types.clone());
                    }
                }
            }

            for action in trigger_actions {
                events_register.add_action(action);
            }
        }
    }
}
