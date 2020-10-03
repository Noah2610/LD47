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
    Movement(Movement),
}

pub fn add_components_to_entity(
    entity_builder: EntityBuilder,
    components: Vec<EntityComponent>,
) -> EntityBuilder {
    use self::EntityComponent as Comp;

    components
        .into_iter()
        .fold(entity_builder, |builder, component| match component {
            Comp::Velocity(velocity) => builder.with(velocity),
            Comp::Size(size) => builder.with(size),
            Comp::Gravity(gravity) => builder.with(gravity),
            Comp::Animation(animation) => builder.with(animation),
            Comp::Animations(animations) => builder.with(animations),
            Comp::BaseFriction(base_friction) => builder.with(base_friction),
            Comp::Hitbox(hitbox) => builder.with(hitbox),
            Comp::Loadable(loadable) => builder.with(loadable),
            Comp::Loader(loader) => builder.with(loader),
            Comp::Collider(collider) => builder.with(collider),
            Comp::Collidable(collidable) => builder.with(collidable),
            Comp::Solid(solid) => builder.with(solid),
            Comp::SolidPusher(solid_pusher) => builder.with(solid_pusher),
            Comp::SolidPushable(solid_pushable) => builder.with(solid_pushable),
            Comp::Movement(movement) => builder.with(movement),
        })
}
