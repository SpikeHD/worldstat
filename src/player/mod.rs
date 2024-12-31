use std::error::Error;

use statistics::Statistics;

use crate::context::Context;

mod api;
pub mod statistics;

pub struct Player {
  ctx: Option<Context>,

  name: String,
  uuid: String,
}

impl Player {
  pub fn new() -> Self {
    Player {
      ctx: None,

      name: String::new(),
      uuid: String::new(),
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
