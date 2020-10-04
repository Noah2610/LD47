use super::menu_prelude::*;
use super::state_prelude::*;
use crate::level_loader::load_level;
use crate::resource;

#[derive(Default)]
pub struct Ingame {
    ui_data: UiData,
}

impl Ingame {
    fn load_ui(&mut self, data: &mut StateData<GameData>) {
        self.create_ui(data, resource("ui/ingame.ron").to_str().unwrap());
    }

    fn unload_ui(&mut self, data: &mut StateData<GameData>) {
        self.delete_ui(data);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.insert(TextOutput::default());

        self.load_ui(&mut data);

        let level_filename = {
            let scene_manager = data.world.read_resource::<SceneManager>();
            let scene = scene_manager.current_scene();
            scene.level_filename.to_string()
        };
        if let Err(e) = load_level(
            data.world,
            resource(format!("levels/{}", &level_filename)),
        ) {
            panic!("Error loading level {}: {}", level_filename, e);
        }
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.unload_ui(&mut data);
    }

    fn update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        // TODO
        {
            use deathframe::amethyst::ecs::{Join, ReadStorage, WriteStorage};
            use deathframe::amethyst::ui::{UiImage, UiTransform};
            data.world.exec(
                |(transform_store, mut image_store): (
                    ReadStorage<UiTransform>,
                    WriteStorage<UiImage>,
                )| {
                    for (transform, image) in
                        (&transform_store, &mut image_store).join()
                    {
                        if &transform.id == "ingame_fade_overlay" {
                            if let UiImage::SolidColor(color) = image {
                                color[3] = (color[3] + 0.01) % 1.01
                            }
                        }
                    }
                },
            );
        }

        let next_level_opt = {
            let mut scene_manager = data.world.write_resource::<SceneManager>();
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
            if let Err(e) = load_level(
                data.world,
                resource(format!("levels/{}", &next_level)),
            ) {
                panic!("Error loading level {}: {}", next_level, e);
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
