use super::component_prelude::*;
use crate::settings::objects_settings::ObjectType;

#[derive(Component)]
pub struct Object {
    pub object_type: ObjectType,
}

impl From<ObjectType> for Object {
    fn from(object_type: ObjectType) -> Self {
        Self { object_type }
    }
}
