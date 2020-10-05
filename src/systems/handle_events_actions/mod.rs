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
        Write<'a, FadeRes>,
        Write<'a, SceneManager>,
        Write<'a, SoundPlayer<SoundKey>>,
        Write<'a, Songs<SongKey>>,
        WriteStorage<'a, EventsRegister>,
        ReadStorage<'a, Object>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, AnimationsContainer<AnimationKey>>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, TextLines>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, VariablesRegister>,
        WriteStorage<'a, IfActions>,
        ReadStorage<'a, Unloaded>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut text_output,
            mut screen_shake,
            mut fade_res,
            mut scene_manager,
            mut sound_player,
            mut songs,
            mut events_register_store,
            object_store,
            mut player_store,
            mut hidden_store,
            mut animations_store,
            mut transform_store,
            mut text_lines_store,
            mut velocity_store,
            mut variables_register_store,
            mut if_actions_store,
            unloaded_store,
        ): Self::SystemData,
    ) {
        let mut trigger_foreign_actions = HashMap::new();

        for (entity, events_register, _) in
            (&entities, &mut events_register_store, !&unloaded_store).join()
        {
            let mut trigger_actions = Vec::new();

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

                    ActionType::StopTimer(timer_name) => {
                        let _ = events_register.timers.remove(&timer_name);
                    }

                    ActionType::SetOutput {
                        text,
                        target_id,
                        does_scroll,
                    } => {
                        text_output.set(target_id, text, does_scroll);
                    }

                    ActionType::AddOutput {
                        text,
                        target_id,
                        does_scroll,
                    } => {
                        text_output.add(target_id, text, does_scroll);
                    }

                    ActionType::ClearOutput {
                        target_id: output_name,
                    } => {
                        text_output.clear(output_name.as_str());
                    }

                    ActionType::ScreenShake(shake) => {
                        screen_shake.shake = Some(shake);
                    }

                    ActionType::NextScene => {
                        scene_manager.should_load_next_scene = true;
                    }

                    ActionType::Fade(fade) => {
                        fade_res.fade = Some(fade);
                    }

                    ActionType::PlaySound(sound_key) => {
                        sound_player.add_action(SoundAction::Play(sound_key));
                    }

                    ActionType::PlaySong(song_key) => {
                        songs.play(&song_key);
                    }

                    ActionType::OutputNextLine {
                        id,
                        target_id,
                        does_scroll,
                    } => {
                        let line_opt = text_lines_store
                            .get_mut(entity)
                            .expect(
                                "PrintNextLine action requires TextLinesGroup \
                                 component",
                            )
                            .next_line(id.as_str())
                            .map(ToString::to_string);
                        if let Some(line) = line_opt {
                            text_output.set(target_id, line, does_scroll);
                        } else {
                            eprintln!(
                                "[WARNING]\n    PrintNextLine action got id \
                                 that doesn't exist: {}",
                                id
                            );
                        }
                    }

                    ActionType::SetVelocity { x, y } => {
                        let velocity = velocity_store.get_mut(entity).expect(
                            "SetVelocity action requires Velocity component",
                        );
                        if let Some(x) = x {
                            velocity.x = x;
                        }
                        if let Some(y) = y {
                            velocity.y = y;
                        }
                    }

                    ActionType::AddVelocity { x, y } => {
                        let velocity = velocity_store.get_mut(entity).expect(
                            "AddVelocity action requires Velocity component",
                        );
                        if let Some(x) = x {
                            velocity.x += x;
                        }
                        if let Some(y) = y {
                            velocity.y += y;
                        }
                    }

                    ActionType::SetVariable(name, value) => {
                        variables_register_store
                            .get_mut(entity)
                            .expect(
                                "SetVariable action requires \
                                 VariablesRegister component",
                            )
                            .variables
                            .insert(name, value);
                    }

                    ActionType::If(if_action) => {
                        if_actions_store
                            .get_mut(entity)
                            .expect("If action requires IfActions component")
                            .add_action(if_action);
                    }
                }
            }

            for trigger_action in trigger_actions {
                events_register.add_action(trigger_action);
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
