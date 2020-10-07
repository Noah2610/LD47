use super::system_prelude::*;
use crate::input::prelude::IngameAction;
use deathframe::physics::query::prelude::{FilterQuery, Query};

#[derive(Default)]
pub struct TriggerInteractionEventsSystem;

impl<'a> System<'a> for TriggerInteractionEventsSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Interactable>,
        ReadStorage<'a, Collider<CollisionTag>>,
        WriteStorage<'a, EventsRegister>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            mut player_store,
            interactable_store,
            collider_store,
            mut events_register_store,
        ): Self::SystemData,
    ) {
        for (player, player_collider) in
            (&mut player_store, &collider_store).join()
        {
            if player.in_control {
                let query_exp = {
                    use deathframe::physics::query::exp::prelude_variants::*;
                    // IsTag(CollisionTag::Interactable)
                    And(Vec::new())
                };

                if input_manager.is_down(IngameAction::Interact) {
                    let colliding_with = player_collider
                        .query::<FilterQuery<_>>()
                        .exp(&query_exp)
                        .run();
                    for (entity, _interactable, events_register) in (
                        &entities,
                        &interactable_store,
                        &mut events_register_store,
                    )
                        .join()
                    {
                        for colliding_with in &colliding_with {
                            if entity.id() == colliding_with.id {
                                events_register
                                    .trigger_event(&EventType::OnInteract);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
