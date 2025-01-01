use std::error::Error;

use player_data::PlayerData;
use statistics::Statistics;

use crate::context::Context;

mod api;
pub mod player_data;
pub mod statistics;

pub struct Player {
  ctx: Option<Context>,

  name: String,
  uuid: String,

  player_data: Option<PlayerData>,
}

impl Player {
  pub fn new() -> Self {
    Player {
      ctx: None,

      name: String::new(),
      uuid: String::new(),

      player_data: None,
    }
  }

  pub fn with_ctx(mut self, ctx: Context) -> Self {
    self.ctx = Some(ctx);
    self
  }

  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
    self.uuid = uuid.into();
    self
  }

  pub fn player_data(&mut self) -> Result<PlayerData, Box<dyn Error>> {
    let ctx = match self.ctx.clone() {
      Some(ctx) => ctx,
      None => return Err("Context required for getting player data".into()),
    };

    if self.player_data.is_none() {
      if ctx.is_singleplayer() {
        self.player_data = Some(PlayerData::new(ctx.clone(), None)?);
      } else {
        let uuid = self.uuid()?;
        self.player_data = Some(PlayerData::new(ctx.clone(), Some(uuid))?);
      }
    }

    Ok(self.player_data.clone().unwrap())
  }

  pub fn uuid(&mut self) -> Result<String, Box<dyn Error>> {
    if self.uuid.is_empty() && !self.name.is_empty() {
      let uuid = api::get_uuid(self.name.as_str())?;
      self.uuid = uuid;
    }

    Ok(self.uuid.clone())
  }

  pub fn name(&mut self) -> Result<String, Box<dyn Error>> {
    if self.name.is_empty() {
      let name = api::get_name(self.uuid.as_str())?;
      self.name = name.clone();
    }

    Ok(self.name.clone())
  }

  pub fn statistics(&mut self) -> Result<Statistics, Box<dyn Error>> {
    let uuid = self.uuid()?;
    let ctx = self.ctx.as_ref();

    if ctx.is_none() {
      Err(format!("Context required for getting statistics").into())
    } else {
      let ctx = ctx.unwrap();
      let statistics = Statistics::get(ctx, uuid)?;
      Ok(statistics)
    }
  }
}
