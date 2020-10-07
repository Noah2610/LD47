use super::menu_prelude::*;
use super::state_prelude::*;
use crate::components::prelude::Size;
use crate::level_loader::load_level;
use crate::resource;
use deathframe::amethyst::assets::ProgressCounter;

#[derive(Default)]
pub struct LoadIngame {
    level_size:  Option<Size>,
    ui_data:     UiData,
    ui_progress: Option<ProgressCounter>,
}

impl LoadIngame {
    fn start(&mut self, data: &mut StateData<GameData>) {
        self.unload_ui(data);
        data.world.delete_all();
        self.load_ui(data);
        data.world.insert(TextOutput::default());
    }

    fn load_ui(&mut self, data: &mut StateData<GameData>) {
        self.ui_progress = Some(
            self.create_ui(data, resource("ui/ingame.ron").to_str().unwrap()),
        );
    }

    fn unload_ui(&mut self, data: &mut StateData<GameData>) {
        self.delete_ui(data);
    }
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for LoadIngame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.start(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.unload_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update_core(data.world);

        if let Some(ui_progress) = self.ui_progress.as_ref() {
            if ui_progress.is_complete() {
                self.load_current_level(data.world);
                Trans::Push(Box::new(Ingame::default()))
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }
}

impl<'a, 'b> LoadIngame {
    fn load_current_level(&mut self, world: &mut World) {
        world.insert(TextOutput::default());
        let level_filename = {
            let scene_manager = world.read_resource::<SceneManager>();
            let scene = scene_manager.current_scene();
            scene.level_filename.to_string()
        };
        match load_level(world, resource(format!("levels/{}", &level_filename)))
        {
            Ok(level_size) => self.level_size = Some(level_size),
            Err(e) => {
                panic!("Error loading level {}: {}", level_filename, e);
            }
        }

        world
            .write_resource::<Songs<SongKey>>()
            .play(&SongKey::MainBgm);
    }
}

impl<'a, 'b> Menu<GameData<'a, 'b>, StateEvent> for LoadIngame {
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
