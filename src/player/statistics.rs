use std::{error::Error, time::Duration};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

#[derive(Serialize, Deserialize, Debug)]
pub struct Statistics {
  #[serde(skip)]
  pub uuid: String,

  #[serde(rename = "DataVersion")]
  pub data_version: u32,
  pub stats: Value,
}

impl Statistics {
  pub fn get(ctx: &Context, uuid: String) -> Result<Statistics, Box<dyn Error>> {
    let path = ctx.path().join("stats").join(format!("{}.json", uuid));

    if path.exists() {
      let contents = std::fs::read_to_string(path)?;
      let mut stats: Statistics = serde_json::from_str(&contents)?;

      stats.uuid = uuid.to_string();

      Ok(stats)
    } else {
      Err(format!("No stats found for uuid {}", uuid).into())
    }
  }

  pub fn save(&self, ctx: &Context, uuid: &str) -> Result<(), Box<dyn Error>> {
    let path = ctx.path().join("stats").join(format!("{}.json", uuid));
    let contents = serde_json::to_string(self)?;

    std::fs::write(path, contents)?;

    Ok(())
  }

  // pub fn playtime(&self) -> Result<Duration, Box<dyn Error>> {
  //   let playtime = self.custom("minecraft:play_time")?;
  //   Duration::from_secs(playtime.as_u64().unwrap() as u64)
  // }

  pub fn custom(&self, key: &str) -> Option<&Value> {
    self.stats.get(key)
  }

  pub fn custom_mut(&mut self, key: &str) -> Option<&mut Value> {
    self.stats.get_mut(key)
  }

  pub fn dropped(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:dropped").and_then(|d| d.get(key))
  }

  pub fn dropped_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:dropped")
      .and_then(|d| d.get_mut(key))
  }

  pub fn mined(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:mined").and_then(|d| d.get(key))
  }

  pub fn mined_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:mined")
      .and_then(|d| d.get_mut(key))
  }

  pub fn killed(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:killed").and_then(|d| d.get(key))
  }

  pub fn killed_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:killed")
      .and_then(|d| d.get_mut(key))
  }

  pub fn picked_up(&self, key: &str) -> Option<&Value> {
    self
      .stats
      .get("minecraft:picked_up")
      .and_then(|d| d.get(key))
  }

  pub fn picked_up_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:picked_up")
      .and_then(|d| d.get_mut(key))
  }

  pub fn used(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:used").and_then(|d| d.get(key))
  }

  pub fn used_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:used")
      .and_then(|d| d.get_mut(key))
  }

  pub fn crafted(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:crafted").and_then(|d| d.get(key))
  }

  pub fn crafted_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:crafted")
      .and_then(|d| d.get_mut(key))
  }

  pub fn broken(&self, key: &str) -> Option<&Value> {
    self.stats.get("minecraft:broken").and_then(|d| d.get(key))
  }

  pub fn broken_mut(&mut self, key: &str) -> Option<&mut Value> {
    self
      .stats
      .get_mut("minecraft:broken")
      .and_then(|d| d.get_mut(key))
  }
}
