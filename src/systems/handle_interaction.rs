use super::system_prelude::*;
use crate::input::prelude::IngameAction;
use deathframe::physics::query::prelude::{FindQuery, Query};

#[derive(Default)]
pub struct HandleInteractionSystem;

impl<'a> System<'a> for HandleInteractionSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, InputManager<IngameBindings>>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Interactable>,
        ReadStorage<'a, Collider<CollisionTag>>,
    );

    fn run(
        &mut self,
        (
            entities,
            input_manager,
            mut player_store,
            interactable_store,
            collider_store,
        ): Self::SystemData,
    ) {
        for (player, player_collider) in
            (&mut player_store, &collider_store).join()
        {
            let query_exp = {
                use deathframe::physics::query::exp::prelude_variants::*;
                IsTag(CollisionTag::Interactable)
            };

            if let Some(interacting_with) = &player.interacting_with {
                unimplemented!();
            } else {
                if input_manager.is_down(IngameAction::Interact) {
                    if let Some(colliding_with) = player_collider
                        .query::<FindQuery<_>>()
                        .exp(&query_exp)
                        .run()
                    {
                        if let Some(interactable) =
                            (&entities, &interactable_store).join().find_map(
                                |(entity, interactable)| {
                                    if entity.id() == colliding_with.id {
                                        Some(interactable)
                                    } else {
                                        None
                                    }
                                },
                            )
                        {
                            player.interacting_with =
                                Some(interactable.interactable_type.clone());
                        }
                    }
                }
            }
        }
    }
}
