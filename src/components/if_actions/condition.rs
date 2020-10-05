pub mod prelude {
    pub use super::IfCondition;
    pub use super::IfStorages;
    pub use super::IfValue;
}

use crate::components::prelude::*;
use crate::resources::SceneManager;
use crate::settings::objects_settings::ObjectType;
use deathframe::amethyst::ecs::shred::ResourceId;
use deathframe::amethyst::ecs::{
    Entities,
    Entity,
    Join,
    Read,
    ReadStorage,
    SystemData,
    World,
};

#[derive(Deserialize, Clone)]
pub enum IfCondition {
    Equal(IfValue, IfValue),
}

impl IfCondition {
    pub fn passes(&self, entity: Entity, stores: &IfStorages) -> bool {
        match self {
            Self::Equal(a, b) => {
                a.value(entity, stores) == b.value(entity, stores)
            }
        }
    }
}

#[derive(Deserialize, Clone)]
pub enum IfValue {
    Val(VariableValue),
    Var(String),
    ForeignObjectValue(ObjectType, Box<IfValue>),
    CurrentLoop,
    CurrentScene,
}

impl IfValue {
    fn value(&self, entity: Entity, stores: &IfStorages) -> VariableValue {
        match self {
            Self::Val(val) => val.clone(),
            Self::Var(var_name) => stores
                .variables_register
                .get(entity)
                .expect("IfValue::Var requires VariablesRegister component")
                .variables
                .get(var_name)
                .cloned()
                .unwrap_or(VariableValue::Null),
            Self::ForeignObjectValue(foreign_object_type, foreign_value) => {
                let foreign_entity = (&stores.entities, &stores.objects)
                    .join()
                    .find_map(|(other_entity, other_object)| {
                        if &other_object.object_type == foreign_object_type {
                            Some(other_entity)
                        } else {
                            None
                        }
                    });

                if let Some(foreign_entity) = foreign_entity {
                    foreign_value.value(foreign_entity, stores)
                } else {
                    eprintln!(
                        "[WARNING IfValue::value]\n    Couldn't find foreign \
                         object entity: {:?}",
                        foreign_object_type
                    );
                    VariableValue::Null
                }
            }
            Self::CurrentLoop => {
                VariableValue::Num(stores.scene_manager.current_loop as i32)
            }
            Self::CurrentScene => VariableValue::Num(
                stores.scene_manager.current_scene_idx as i32,
            ),
        }
    }
}

#[derive(SystemData)]
pub struct IfStorages<'a> {
    entities:           Entities<'a>,
    scene_manager:      Read<'a, SceneManager>,
    variables_register: ReadStorage<'a, VariablesRegister>,
    objects:            ReadStorage<'a, Object>,
}
