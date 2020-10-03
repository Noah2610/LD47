use crate::settings::prelude::{ObjectType, TileType};
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
pub struct LevelData {
    pub level:   LevelInfo,
    pub tiles:   Vec<TileData>,
    pub objects: Vec<ObjectData>,
}

#[derive(Deserialize, Clone)]
pub struct LevelInfo {
    pub size:      SizeData,
    pub tile_size: SizeData,
}

#[derive(Deserialize, Clone)]
pub struct PosData {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Clone)]
pub struct SizeData {
    pub w: f32,
    pub h: f32,
}

pub type PropsData = HashMap<String, serde_json::Value>;

#[derive(Deserialize, Clone)]
pub struct TileData {
    pub id:        usize,
    #[serde(rename = "type")]
    pub tile_type: TileType,
    #[serde(rename = "ts")]
    pub tileset:   String,
    pub pos:       PosData,
    pub props:     PropsData,
}

#[derive(Deserialize, Clone)]
pub struct ObjectData {
    #[serde(rename = "type")]
    pub object_type: ObjectType,
    pub pos:         PosData,
    pub size:        SizeData,
    pub props:       PropsData,
}
