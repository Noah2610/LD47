use super::system_prelude::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct HandleEventsActionsSystem;

impl<'a> System<'a> for HandleEventsActionsSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Object>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut events_register_store,
            object_store,
            mut player_store,
            mut hidden_store,
            mut animations_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let mut trigger_foreign_actions = HashMap::new();

        for (entity, events_register, _) in
            (&entities, &mut events_register_store, !&unloaded_store).join()
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
                        player_store
                            .get_mut(entity)
                            .expect(
                                "SetControllable action only works with Player",
                            )
                            .in_control = controllable;
                    }

                    ActionType::Show => {
                        let _ = hidden_store.remove(entity);
                    }

                    ActionType::Hide => {
                        let _ = hidden_store.insert(entity, Hidden);
                    }

                    ActionType::PlayAnimation(anim) => {
                        animations_store
                            .get_mut(entity)
                            .expect(
                                "PlayAnimation action requires \
                                 AnimationsContainer component",
                            )
                            .play(anim)
                            .unwrap();
                    }

                    ActionType::PushAnimation(anim) => {
                        animations_store
                            .get_mut(entity)
                            .expect(
                                "PushAnimation action requires \
                                 AnimationsContainer component",
                            )
                            .push(anim)
                            .unwrap();
                    }

                    ActionType::PopAnimation => {
                        animations_store
                            .get_mut(entity)
                            .expect(
                                "PopAnimation action requires \
                                 AnimationsContainer component",
                            )
                            .pop()
                            .unwrap();
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
