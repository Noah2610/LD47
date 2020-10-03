use super::data::*;
use crate::components::prelude::*;
use crate::resource;
use crate::resources::*;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use std::path::PathBuf;

pub(super) fn load_player(
    world: &mut World,
    object: ObjectData,
) -> amethyst::Result<()> {
    let player_settings = (*world.read_resource::<PlayerSettings>()).clone();

    let transform = {
        let mut transform = Transform::default();
        transform.set_translation_x(object.pos.x);
        transform.set_translation_y(object.pos.y);
        if let Some(z) = object.props.get("z").and_then(|val| val.as_f64()) {
            transform.set_translation_z(z as f32);
        }
        transform
    };

    let sprite_render = {
        let sprite_sheet = world
            .write_resource::<SpriteSheetHandles<PathBuf>>()
            .get_or_load(
                resource(format!(
                    "spritesheets/{}",
                    &player_settings.spritesheet_filename
                )),
                world,
            );
        SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        }
    };

    let size = Size::new(object.size.w, object.size.h);

    let mut entity_builder = world
        .create_entity()
        .with(transform)
        .with(size.clone())
        .with(sprite_render)
        .with(Transparent)
        .with(Object::from(object.object_type))
        .with(ScaleOnce::default())
        .with(Player::default())
        .with(Velocity::default());

    entity_builder = add_components_to_entity(
        entity_builder,
        player_settings.components.components,
        Some(size),
    );

    if let Some(events_register) = player_settings.events_register {
        entity_builder = entity_builder.with(events_register);
    }

    entity_builder.build();

    Ok(())
}
