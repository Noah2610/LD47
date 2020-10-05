pub mod data;
pub mod load_objects;

mod load_camera;
mod load_player;
mod load_tiles;

use crate::components::prelude::*;
use amethyst::ecs::World;
use data::*;
use deathframe::amethyst;
use std::fs::File;
use std::path::PathBuf;

pub fn load_level(
    world: &mut World,
    filepath: PathBuf,
) -> amethyst::Result<Size> {
    let level_file = File::open(filepath)?;
    let level_data = serde_json::de::from_reader::<_, LevelData>(level_file)?;
    let level_size =
        Size::new(level_data.level.size.w, level_data.level.size.h);
    let tile_size =
        Size::new(level_data.level.tile_size.w, level_data.level.tile_size.h);

    load_tiles::load_tiles(world, level_data.tiles, tile_size)?;
    load_objects::load_objects(world, level_data.objects, level_size.clone())?;

    Ok(level_size)
}
