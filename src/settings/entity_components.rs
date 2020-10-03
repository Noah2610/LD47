use crate::components::prelude::*;
use crate::resources::{AnimationKey, CollisionTag, SolidTag};
use amethyst::ecs::{Builder, EntityBuilder};
use deathframe::amethyst;

#[derive(Deserialize, Clone)]
#[serde(from = "Vec<EntityComponent>")]
#[serde(deny_unknown_fields)]
pub struct EntityComponents {
    pub components: Vec<EntityComponent>,
}

impl From<Vec<EntityComponent>> for EntityComponents {
    fn from(components: Vec<EntityComponent>) -> Self {
        Self { components }
    }
}

#[derive(Deserialize, Clone)]
pub enum EntityComponent {
    Velocity(Velocity),
    Size(Size),
    Gravity(Gravity),
    Animation(Animation),
    Animations(AnimationsContainer<AnimationKey>),
    BaseFriction(BaseFriction),
    Hitbox(Hitbox),
    Loadable(Loadable),
    Loader(Loader),
    Collider(Collider<CollisionTag>),
    Collidable(Collidable<CollisionTag>),
    Solid(Solid<SolidTag>),
    SolidPusher(SolidPusher),
    SolidPushable(SolidPushable),
}

pub fn add_components_to_entity(
    entity_builder: EntityBuilder,
    components: Vec<EntityComponent>,
) -> EntityBuilder {
    use self::EntityComponent as Comp;

    components
        .into_iter()
        .fold(
            entity_builder,
            |entity_builder, component| match component {
                Comp::Velocity(velocity) => entity_builder.with(velocity),
                Comp::Size(size) => entity_builder.with(size),
                Comp::Gravity(gravity) => entity_builder.with(gravity),
                Comp::Animation(animation) => entity_builder.with(animation),
                Comp::Animations(animations) => entity_builder.with(animations),
                Comp::BaseFriction(base_friction) => {
                    entity_builder.with(base_friction)
                }
                Comp::Hitbox(hitbox) => entity_builder.with(hitbox),
                Comp::Loadable(loadable) => entity_builder.with(loadable),
                Comp::Loader(loader) => entity_builder.with(loader),
                Comp::Collider(collider) => entity_builder.with(collider),
                Comp::Collidable(collidable) => entity_builder.with(collidable),
                Comp::Solid(solid) => entity_builder.with(solid),
                Comp::SolidPusher(solid_pusher) => {
                    entity_builder.with(solid_pusher)
                }
                Comp::SolidPushable(solid_pushable) => {
                    entity_builder.with(solid_pushable)
                }
            },
        )
}
