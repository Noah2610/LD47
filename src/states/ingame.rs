use super::menu_prelude::*;
use super::state_prelude::*;
use crate::level_loader::load_level;
use crate::resource;

#[derive(Default)]
pub struct Ingame {
    ui_data: UiData,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        data.world.insert(TextOutput::default());
        self.create_ui(&mut data, resource("ui/ingame.ron").to_str().unwrap());
        if let Err(e) = load_level(data.world, resource("levels/dev.json")) {
            panic!("Error loading level: {}", e);
        }
    }

    fn on_stop(&mut self, mut data: StateData<GameData<'a, 'b>>) {
        self.delete_ui(&mut data);
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

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
