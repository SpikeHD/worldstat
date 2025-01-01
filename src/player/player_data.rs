use std::{collections::HashMap, error::Error};

use fastnbt::{IntArray, Value};
use flate2::read::GzDecoder;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt};

use crate::{context::Context, world::World};

#[serde_as]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "PascalCase")]
/// Struct representation of raw player data.
pub struct PlayerData {
  pub data_version: i32,

  pub absorption_amount: f32,
  pub air: i32,
  pub dimension: String,
  pub health: f32,

  #[serde_as(as = "BoolFromInt")]
  pub invulnerable: bool,

  pub last_death_location: Option<LastDeathLocation>,

  #[serde_as(as = "BoolFromInt")]
  pub on_ground: bool,

  pub pos: Vec<f64>,
  pub rotation: Vec<f32>,
  pub score: i32,
  pub selected_item_slot: i32,
  pub sleep_timer: i32,

  pub spawn_angle: f32,
  pub spawn_x: i32,
  pub spawn_y: i32,
  pub spawn_z: i32,

  #[serde(rename = "UUID")]
  pub uuid: IntArray,
  pub xp_level: i32,
  pub xp_p: f32,
  pub xp_seed: i32,
  pub xp_total: i32,

  #[serde(rename = "foodLevel")]
  pub food_level: i32,

  #[serde(rename = "playerGameType")]
  pub player_game_type: i32,

  #[serde(rename = "seenCredits")]
  #[serde_as(as = "BoolFromInt")]
  pub seen_credits: bool,

  #[serde(flatten)]
  other: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LastDeathLocation {
  pub dimension: String,
  pub pos: IntArray,
}

impl PlayerData {
  pub fn new(ctx: Context, uuid: Option<String>) -> Result<Self, Box<dyn Error>> {
    // If no UUID is provided, assume singleplayer
    if uuid.is_none() {
      let world = World::new(ctx)?;
      world
        .world_data
        .player
        .clone()
        .ok_or("No player data found in level.dat".into())
    } else {
      let uuid = uuid.unwrap();
      let path = ctx.path().join("playerdata").join(format!("{}.dat", uuid));
      let contents = std::fs::File::open(path)?;
      let decoder = GzDecoder::new(contents);
      let player_data: PlayerData = fastnbt::from_reader(decoder)?;

      Ok(player_data)
    }
  }

  pub fn get(&self, key: &str) -> Option<&Value> {
    self.other.get(key)
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
    self.other.get_mut(key)
  }
}
