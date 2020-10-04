use super::system_prelude::*;
use climer::Timer;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Default)]
pub struct HandleEventsActionsSystem;

impl<'a> System<'a> for HandleEventsActionsSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, TextOutput>,
        Write<'a, ScreenShakeRes>,
        Write<'a, SceneManager>,
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Object>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        WriteStorage<'a, Transform>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut text_output,
            mut screen_shake,
            mut scene_manager,
            mut events_register_store,
            object_store,
            mut player_store,
            mut hidden_store,
            mut animations_store,
            mut transform_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let mut trigger_foreign_actions = HashMap::new();

        for (entity, events_register, _) in
            (&entities, &mut events_register_store, !&unloaded_store).join()
        {
            for action in events_register.triggered_actions.drain(..) {
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

                    ActionType::FaceTowardsObject(other_object_type) => {
                        if let Some(other_x) = (&object_store, &transform_store)
                            .join()
                            .find_map(|(object, transform)| {
                                if &object.object_type == &other_object_type {
                                    Some(transform.translation().x)
                                } else {
                                    None
                                }
                            })
                        {
                            if let Some(transform) =
                                transform_store.get_mut(entity)
                            {
                                let pos = transform.translation();
                                let diff = other_x - pos.x;
                                let scale = transform.scale_mut();
                                scale.x = scale.x.abs() * diff.signum();
                            }
                        }
                    }

                    ActionType::StartTimer(timer_name, millis) => {
                        let mut timer = Timer::new(
                            Some(Duration::from_millis(millis).into()),
                            None,
                        );
                        timer.start().unwrap();
                        events_register.timers.insert(timer_name, timer);
                    }

                    ActionType::SetOutput(lines) => {
                        text_output.set(lines);
                    }

                    ActionType::AddOutput(lines) => {
                        text_output.add(lines);
                    }

                    ActionType::ClearOutput => {
                        text_output.clear();
                    }

                    ActionType::ScreenShake(shake) => {
                        screen_shake.shake = Some(shake);
                    }

                    ActionType::NextScene => {
                        scene_manager.should_load_next_scene = true;
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
