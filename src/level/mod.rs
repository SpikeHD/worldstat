use fastnbt::Value;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

#[serde_as]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct WorldData {
  #[serde(rename = "allowCommands")]
  #[serde_as(as = "BoolFromInt")]
  allow_commands: bool,

  border_center_x: i32,
  border_center_z: i32,
  border_damage_per_block: f32,
  border_safe_zone: i32,
  border_size: f64,
  border_size_lerp_target: f64,
  border_size_lerp_time: i64,
  border_warning_blocks: i32,
  border_warning_time: i32,

  #[serde(rename = "clearWeatherTime")]
  clear_weather_time: i64,

  custom_boss_events: Option<Value>,
  data_packs: Option<Value>,
  data_version: i32,
  day_time: i64,
  difficulty: i32,
  difficulty_locked: i32,
  dragon_fight: Option<Value>,
  game_rules: Option<Value>,
  game_type: i32,

  #[serde(rename = "hardcore")]
  #[serde_as(as = "BoolFromInt")]
  hardcore: bool,

  #[serde(rename = "initialized")]
  #[serde_as(as = "BoolFromInt")]
  initialized: bool,

  last_played: i64,
  level_name: String,
  player: Option<Value>,

  #[serde(rename = "rainTime")]
  rain_time: i64,

  #[serde(rename = "raining")]
  #[serde_as(as = "BoolFromInt")]
  raining: bool,

  scheduled_events: Option<Value>,
  server_brands: Vec<String>,
  spawn_angle: f32,
  spawn_x: i32,
  spawn_y: i32,
  spawn_z: i32,
  time: i64,

  #[serde(rename = "thunderTime")]
  thunder_time: i64,

  #[serde(rename = "thundering")]
  #[serde_as(as = "BoolFromInt")]
  thundering: bool,

  version: Option<Value>,
  wandering_trader_id: [i32; 4],
  wandering_trader_spawn_chance: i32,
  wandering_trader_spawn_delay: i32,
  #[serde_as(as = "BoolFromInt")]
  was_modded: bool,
  world_gen_settings: WorldGenSettings,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct WorldGenSettings {
  #[serde_as(as = "BoolFromInt")]
  bonus_chest: bool,
  dimensions: Option<Value>,
  #[serde_as(as = "BoolFromInt")]
  generate_features: bool,
  seed: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Version {
  pub id: i32,
  pub name: String,
  pub series: String,
  pub snapshot: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Level {
  data: WorldData,
}
