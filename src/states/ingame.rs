use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::Size;
use crate::level_loader::{data, load_level, load_objects::load_object};
use crate::resource;
use deathframe::amethyst::assets::ProgressCounter;
use std::collections::HashMap;

#[derive(Default)]
pub struct Ingame {
    loaded_first_level: bool,
    level_size:         Option<Size>,
    ui_data:            UiData,
    ui_progress:        Option<ProgressCounter>,
}

impl Ingame {
    fn load_ui(&mut self, data: &mut StateData<GameData>) {
        self.ui_progress = Some(
            self.create_ui(data, resource("ui/ingame.ron").to_str().unwrap()),
        );
    }

    fn unload_ui(&mut self, data: &mut StateData<GameData>) {
        self.delete_ui(data);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.insert(TextOutput::default());

        self.load_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.unload_ui(&mut data);
    }

    fn update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        if self.loaded_first_level {
            // LOAD NEXT LEVEL
            let next_level_opt = {
                let mut scene_manager =
                    data.world.write_resource::<SceneManager>();
                if scene_manager.should_load_next_scene {
                    let scene = scene_manager.next_scene();
                    Some(scene.level_filename.to_string())
                } else {
                    None
                }
            };
            if let Some(next_level) = next_level_opt {
                self.unload_ui(&mut data);
                data.world.delete_all();
                data.world.insert(TextOutput::default());
                self.load_ui(&mut data);
                match load_level(
                    data.world,
                    resource(format!("levels/{}", &next_level)),
                ) {
                    Ok(level_size) => self.level_size = Some(level_size),
                    Err(e) => {
                        panic!("Error loading level {}: {}", next_level, e)
                    }
                }
            }

            // SPAWN OBJECTS
            let to_spawn = {
                let mut object_spawner =
                    data.world.write_resource::<ObjectSpawner>();
                object_spawner.to_spawn.drain(..).collect::<Vec<_>>()
            };
            for object in to_spawn {
                let mut props = HashMap::new();
                props.insert(
                    "z".to_string(),
                    serde_json::Value::from(object.position.2),
                );
                let object_data = data::ObjectData {
                    object_type: object.object.clone(),
                    pos: data::PosData {
                        x: object.position.0,
                        y: object.position.1,
                    },
                    size: data::SizeData {
                        w: object.size.0,
                        h: object.size.1,
                    },
                    props,
                };
                if let Err(e) = load_object(
                    data.world,
                    object_data,
                    self.level_size
                        .as_ref()
                        .map(|s| s.clone())
                        .expect("Should have level_size at this point"),
                ) {
                    panic!("Error loading object {:?}: {}", object.object, e);
                }
            }
        } else {
            // LOAD LEVEL AFTER UI LOADED
            if let Some(ui_progress) = self.ui_progress.as_ref() {
                if ui_progress.is_complete() {
                    self.loaded_first_level = true;

                    let level_filename = {
                        let scene_manager =
                            data.world.read_resource::<SceneManager>();
                        let scene = scene_manager.current_scene();
                        scene.level_filename.to_string()
                    };
                    match load_level(
                        data.world,
                        resource(format!("levels/{}", &level_filename)),
                    ) {
                        Ok(level_size) => self.level_size = Some(level_size),
                        Err(e) => {
                            panic!(
                                "Error loading level {}: {}",
                                level_filename, e
                            );
                        }
                    }

                    data.world
                        .write_resource::<Songs<SongKey>>()
                        .play(&SongKey::MainBgm);
                }
            }
        }

        Trans::None
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for Ingame {
    fn event_triggered(
        &mut self,
        _data: &mut StateData<GameData<'a, 'b>>,
        event_name: String,
        event: UiEvent,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        if let UiEventType::ClickStop = event.event_type {
            match event_name.as_str() {
                _ => None,
            }
        } else {
            None
        }
    }

    fn ui_data(&self) -> &UiData {
        &self.ui_data
    }

    fn ui_data_mut(&mut self) -> &mut UiData {
        &mut self.ui_data
    }
}
