use super::data::*;
use super::load_player;
use crate::components::prelude::*;
use crate::resource;
use crate::resources::*;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use std::path::PathBuf;

pub(super) fn load_objects(
    world: &mut World,
    objects: Vec<ObjectData>,
    level_size: Size,
) -> amethyst::Result<()> {
    let current_loop = world.read_resource::<SceneManager>().current_loop;
    let objects_settings =
        world.read_resource::<ObjectsSettings>().objects.clone();

    for object in objects {
        match &object.object_type {
            ObjectType::Player => {
                load_player::load_player(world, object, level_size.clone())?;
            }

            ObjectType::Custom(object_ident) => {
                let object_settings = objects_settings
                    .get(object_ident)
                    .expect(&format!(
                        "No settings for object: {}",
                        object_ident
                    ))
                    .clone();

                let transform = {
                    let mut transform = Transform::default();
                    transform.set_translation_x(object.pos.x);
                    transform.set_translation_y(object.pos.y);
                    if let Some(z) =
                        object.props.get("z").and_then(|val| val.as_f64())
                    {
                        transform.set_translation_z(z as f32);
                    }
                    transform
                };

                let sprite_render_opt = if let Some(spritesheet_filename) =
                    &object_settings.spritesheet_filename
                {
                    Some({
                        let sprite_sheet = world
                            .write_resource::<SpriteSheetHandles<PathBuf>>()
                            .get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    spritesheet_filename,
                                )),
                                world,
                            );
                        SpriteRender {
                            sprite_sheet,
                            sprite_number: 0,
                        }
                    })
                } else {
                    None
                };

                let size = Size::new(object.size.w, object.size.h);

                let mut entity_builder = world
                    .create_entity()
                    .with(transform)
                    .with(size.clone())
                    .with(Transparent)
                    .with(Object::from(object.object_type))
                    .with(ScaleOnce::default())
                    .with(Loadable::default());

                if let Some(sprite_render) = sprite_render_opt {
                    entity_builder = entity_builder.with(sprite_render);
                }

                if let Some(components) = object_settings.components {
                    entity_builder = add_components_to_entity(
                        entity_builder,
                        components.components,
                        Some(size),
                        current_loop,
                    );
                }

                if let Some(events_register) = object_settings.events_register {
                    entity_builder = entity_builder.with(events_register);
                }

                entity_builder.build();
            }
        }
    }

    Ok(())
}
