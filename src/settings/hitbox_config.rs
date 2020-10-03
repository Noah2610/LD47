use crate::components::prelude::{Hitbox, Size};
use amethyst::ecs::{Builder, EntityBuilder};
use deathframe::amethyst;
use deathframe::core::geo::prelude::Rect;

#[derive(Deserialize, Clone)]
pub enum HitboxConfig {
    Size,
    Custom(Hitbox),
}

impl HitboxConfig {
    pub fn add_hitbox_to_entity<'a>(
        &self,
        entity_builder: EntityBuilder<'a>,
        size_opt: Option<&Size>,
    ) -> EntityBuilder<'a> {
        match self {
            HitboxConfig::Size => {
                if let Some(size) = size_opt {
                    entity_builder
                        .with(Hitbox::new().with_rect(Rect::from(size)))
                } else {
                    panic!("HitboxConfig::Size entity doesn't have a Size");
                }
            }
            HitboxConfig::Custom(hitbox) => entity_builder.with(hitbox.clone()),
        }
    }
}
