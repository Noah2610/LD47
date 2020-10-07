use super::state_prelude::*;
use crate::components::prelude::Size;
use crate::level_loader::{data, load_objects::load_object};
use std::collections::HashMap;

pub struct Ingame {
    level_size: Size,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Ingame {
    fn update(
        &mut self,
        mut data: StateData<GameData<'a, 'b>>,
    ) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(data.world, DispatcherId::Ingame).unwrap();

        if let Some(trans) = self.maybe_load_next_level(&mut data) {
            return trans;
        }
        self.spawn_objects(data.world);

        Trans::None
    }
}

impl<'a, 'b> Ingame {
    pub fn new(level_size: Size) -> Self {
        Self { level_size }
    }

    fn maybe_load_next_level(
        &mut self,
        data: &mut StateData<GameData<'a, 'b>>,
    ) -> Option<Trans<GameData<'a, 'b>, StateEvent>> {
        let mut scene_manager = data.world.write_resource::<SceneManager>();
        if scene_manager.should_load_next_scene {
            let _scene = scene_manager.next_scene();
            Some(Trans::Pop)
        } else {
            None
        }
    }

    fn spawn_objects(&mut self, world: &mut World) {
        let to_spawn = {
            let mut object_spawner = world.write_resource::<ObjectSpawner>();
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
            if let Err(e) =
                load_object(world, object_data, self.level_size.clone())
            {
                panic!("Error loading object {:?}: {}", object.object, e);
            }
        }
    }
}
