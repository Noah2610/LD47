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
    LessThan(IfValue, IfValue),
    GreaterThan(IfValue, IfValue),
    Not(Box<IfCondition>),
    And(Vec<IfCondition>),
    Or(Vec<IfCondition>),
}

impl IfCondition {
    pub fn passes(&self, entity: Entity, stores: &IfStorages) -> bool {
        match self {
            Self::Equal(a, b) => {
                a.value(entity, stores) == b.value(entity, stores)
            }
            Self::LessThan(a, b) => {
                a.value(entity, stores) < b.value(entity, stores)
            }
            Self::GreaterThan(a, b) => {
                a.value(entity, stores) > b.value(entity, stores)
            }
            Self::Not(cond) => !cond.passes(entity, stores),
            Self::And(conds) => {
                conds.iter().all(|cond| cond.passes(entity, stores))
            }
            Self::Or(conds) => {
                conds.iter().any(|cond| cond.passes(entity, stores))
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
    TextLinesIdx(String),
}

impl IfValue {
    fn value(&self, entity: Entity, stores: &IfStorages) -> VariableValue {
        use self::VariableValue as Value;

        match self {
            Self::Val(val) => val.clone(),
            Self::Var(var_name) => stores
                .variables_register
                .get(entity)
                .expect("IfValue::Var requires VariablesRegister component")
                .variables
                .get(var_name)
                .cloned()
                .unwrap_or(Value::Null),
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
                    Value::Null
                }
            }
            Self::CurrentLoop => {
                Value::Num(stores.scene_manager.current_loop as i32)
            }
            Self::CurrentScene => {
                Value::Num(stores.scene_manager.current_scene_idx as i32)
            }
            Self::TextLinesIdx(name) => stores
                .text_lines
                .get(entity)
                .expect("IfValue::TextLineIdx requires TextLines component")
                .line_idx(name)
                .map(|idx| Value::Num(idx as i32))
                .unwrap_or_else(|| {
                    eprintln!(
                        "[WARNING IfValue::TextLineIdx]\n    Given text lines \
                         name doesn't exist: {}",
                        name
                    );
                    Value::Null
                }),
        }
    }
}

#[derive(SystemData)]
pub struct IfStorages<'a> {
    entities:           Entities<'a>,
    scene_manager:      Read<'a, SceneManager>,
    variables_register: ReadStorage<'a, VariablesRegister>,
    objects:            ReadStorage<'a, Object>,
    text_lines:         ReadStorage<'a, TextLines>,
}
