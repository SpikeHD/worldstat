use std::collections::HashMap;

use fastnbt::{IntArray, Value};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

use crate::player::player_data::PlayerData;

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WorldData {
  pub data_version: i32,
  pub difficulty: i32,
  #[serde_as(as = "BoolFromInt")]
  pub difficulty_locked: bool,
  pub game_type: i32,
  pub last_played: i64,
  pub level_name: String,
  pub player: Option<PlayerData>,
  pub server_brands: Option<Vec<String>>,
  pub spawn_angle: f32,
  pub spawn_x: i32,
  pub spawn_y: i32,
  pub spawn_z: i32,
  pub time: i64,
  pub version: Version,
  pub world_gen_settings: WorldGenSettings,

  #[serde(flatten)]
  pub other: HashMap<String, Value>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorldGenSettings {
  #[serde_as(as = "BoolFromInt")]
  pub bonus_chest: bool,
  pub dimensions: Option<Value>,
  #[serde_as(as = "BoolFromInt")]
  pub generate_features: bool,
  pub seed: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
  pub id: i32,
  pub name: String,
  pub series: String,
  pub snapshot: i32,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Level {
  pub data: WorldData,
}
