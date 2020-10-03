use super::state_prelude::*;
use crate::level_loader::load_level;
use crate::resource;

#[derive(Default)]
pub struct Ingame;

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn on_start(&mut self, data: StateData<GameData<'a, 'b>>) {
        if let Err(e) = load_level(data.world, resource("levels/dev.json")) {
            panic!("Error loading level: {}", e);
        }
    }

    fn update(
        &mut self,
        data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        Trans::None
    }
}
