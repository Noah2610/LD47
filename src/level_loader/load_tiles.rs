use super::data::*;
use crate::components::prelude::*;
use crate::resource;
use crate::resources::*;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use std::path::PathBuf;

pub(super) fn load_tiles(
    world: &mut World,
    tiles: Vec<TileData>,
    tile_size: Size,
) -> amethyst::Result<()> {
    let tiles_settings = world.read_resource::<TilesSettings>().tiles.clone();

    for tile in tiles {
        let tile_settings = tiles_settings
            .get(&tile.tile_type)
            .expect(&format!("No settings for tile: {}", &tile.tile_type))
            .clone();

        let transform = {
            let mut transform = Transform::default();
            transform.set_translation_x(tile.pos.x);
            transform.set_translation_y(tile.pos.y);
            if let Some(z) = tile.props.get("z").and_then(|val| val.as_f64()) {
                transform.set_translation_z(z as f32);
            }
            transform
        };

        let sprite_render = {
            let sprite_sheet = world
                .write_resource::<SpriteSheetHandles<PathBuf>>()
                .get_or_load(
                    resource(format!(
                        "spritesheets/tilesets/{}",
                        &tile.tileset
                    )),
                    world,
                );
            SpriteRender {
                sprite_sheet,
                sprite_number: tile.id,
            }
        };

        let mut entity_builder = world
            .create_entity()
            .with(transform)
            .with(tile_size.clone())
            .with(sprite_render)
            .with(Transparent)
            .with(ScaleOnce::default())
            .with(Loadable::default());

        if let Some(components) = tile_settings.components {
            entity_builder =
                add_components_to_entity(entity_builder, components.components);
        }

        entity_builder.build();
    }

    Ok(())
}
