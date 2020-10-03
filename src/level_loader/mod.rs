mod data;

use data::*;

use crate::components::prelude::*;
use crate::resource;
use crate::resources::*;
use crate::settings::entity_components::add_components_to_entity;
use crate::settings::prelude::*;
use amethyst::ecs::{Builder, World, WorldExt};
use deathframe::amethyst;
use std::fs::File;
use std::path::PathBuf;

pub fn load_level(
    world: &mut World,
    filepath: PathBuf,
) -> amethyst::Result<()> {
    let level_file = File::open(filepath)?;
    let level_data = serde_json::de::from_reader::<_, LevelData>(level_file)?;
    let tile_size =
        Size::new(level_data.level.tile_size.w, level_data.level.tile_size.h);

    load_tiles(world, level_data.tiles, tile_size)?;

    Ok(())
}

fn load_tiles(
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

        entity_builder = add_components_to_entity(
            entity_builder,
            tile_settings.components.components,
        );

        entity_builder.build();
    }

    Ok(())
}
