use std::{collections::HashMap, error::Error};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{context::Context, util};

#[derive(Serialize, Deserialize, Clone, Debug)]
/// Struct representing player advancements. This covers recipe unlocks and regular achievements.
/// 
/// Refer to the [Advancements](https://minecraft.wiki/w/Advancements) page on the Minecraft Wiki for more information and fields.
pub struct Advancements {
  pub data_version: i32,

  #[serde(flatten)]
  other: HashMap<String, Value>,
}

impl Advancements {
  pub fn new(ctx: &Context, uuid: String) -> Result<Self, Box<dyn Error>> {
    let advancements = ctx.path().join("advancements");
    let path = util::player_file(&uuid, advancements);

    if let Some(path) = path {
      let contents = std::fs::read_to_string(path)?;
      let advancements: Advancements = serde_json::from_str(&contents)?;

      return Ok(advancements);
    }

    Err("No advancements found".into())
  }

  pub fn get(&self, key: &str) -> Option<&Value> {
    self.other.get(key)
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
    self.other.get_mut(key)
  }
}
