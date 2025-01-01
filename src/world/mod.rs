use std::{error::Error, io::Read};

use flate2::read::GzDecoder;

use crate::{context::Context, level::{Level, WorldData}};

pub struct World {
  ctx: Context,

  pub world_data: WorldData,
}

impl World {
  pub fn new(ctx: Context) -> Result<Self, Box<dyn Error>> {
    let level_dat = std::fs::File::open(ctx.path().join("level.dat"))?;
    let decoder = GzDecoder::new(level_dat);
    let level: Level = fastnbt::from_reader(decoder)?;

    Ok(World { ctx, world_data: level.data })
  }

  pub fn save(&self) -> Result<(), Box<dyn Error>> {
    let path = self.ctx.path().join("level.dat");
    let level = Level { data: self.world_data.clone() };
    let contents = fastnbt::to_bytes(&level)?;

    std::fs::write(path, contents)?;

    Ok(())
  }
}
