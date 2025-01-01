use std::error::Error;

use advancements::Advancements;
use player_data::PlayerData;
use statistics::Statistics;

use crate::context::Context;

pub mod advancements;
mod api;
pub mod player_data;
pub mod statistics;

/// Struct for representing and interfacing with a player.
pub struct Player {
  ctx: Option<Context>,

  name: String,
  uuid: String,

  skin_urls: Option<SkinUrls>,

  player_data: Option<PlayerData>,
}

impl Default for Player {
  fn default() -> Self {
    Self::new()
  }
}

impl Player {
  pub fn new() -> Self {
    Player {
      ctx: None,

      name: String::new(),
      uuid: String::new(),

      skin_urls: None,

      player_data: None,
    }
  }

  /// Specify the [Context] to use.
  pub fn with_ctx(mut self, ctx: Context) -> Self {
    self.ctx = Some(ctx);
    self
  }

  /// Specify the player's name. It will be used to fetch the UUID.
  pub fn with_name(mut self, name: impl Into<String>) -> Self {
    self.name = name.into();
    self
  }

  /// Specify the player's UUID. It will be used to fetch player name and data.
  pub fn with_uuid(mut self, uuid: impl Into<String>) -> Self {
    self.uuid = uuid.into();
    self
  }

  /// Get the player's data.
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

  /// Get the player's skin URLs.
  pub fn skin_urls(&mut self) -> Result<Option<SkinUrls>, Box<dyn Error>> {
    if self.skin_urls.is_none() {
      let uuid = self.uuid()?;
      self.skin_urls = Some(api::get_skin_data(&uuid)?);
    }

    Ok(self.skin_urls.clone())
  }

  /// Get the player's UUID.
  pub fn uuid(&mut self) -> Result<String, Box<dyn Error>> {
    if self.uuid.is_empty() && !self.name.is_empty() {
      let uuid = api::get_uuid(self.name.as_str())?;
      self.uuid = uuid;
    }

    Ok(self.uuid.clone())
  }

  /// Get the player's name.
  pub fn name(&mut self) -> Result<String, Box<dyn Error>> {
    if self.name.is_empty() {
      let name = api::get_name(self.uuid.as_str())?;
      self.name = name.clone();
    }

    Ok(self.name.clone())
  }

  /// get the player's advancements.
  pub fn advancements(&mut self) -> Result<Advancements, Box<dyn Error>> {
    let uuid = self.uuid()?;
    let ctx = self.ctx.as_ref();

    if ctx.is_none() {
      Err("Context required for getting advancements".to_string().into())
    } else {
      let ctx = ctx.unwrap();
      let advancements = Advancements::new(ctx, uuid)?;
      Ok(advancements)
    }
  }

  /// Get the player's statistics.
  pub fn statistics(&mut self) -> Result<Statistics, Box<dyn Error>> {
    let uuid = self.uuid()?;
    let ctx = self.ctx.as_ref();

    if ctx.is_none() {
      Err("Context required for getting statistics".to_string().into())
    } else {
      let ctx = ctx.unwrap();
      let statistics = Statistics::get(ctx, uuid)?;
      Ok(statistics)
    }
  }
}

#[derive(Clone, Debug)]
/// Struct representing URLs for a player's skin.
pub struct SkinUrls {
  pub skin: String,
  pub cape: String,
}