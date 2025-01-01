use std::time::Duration;

use worldstat::{context::Context, player::Player};

fn main() {
  let path = std::env::args().nth(1).unwrap();
  let user = std::env::args().nth(2).unwrap();
  let ctx = Context::new().with_path(path);
  let player = Player::new()
    .with_name(user)
    .with_ctx(ctx)
    .statistics()
    .unwrap();

  let crafted = player.crafted("minecraft:oak_planks").unwrap();
  let count = crafted.as_i64().unwrap_or(0);
  let playtime = player.playtime().unwrap();

  println!(
    "User has crafted {} oak planks, and has played for {}",
    count,
    duration_to_unit(playtime)
  );
}

fn duration_to_unit(duration: Duration) -> String {
  if duration.as_secs() < 60 {
    format!("{} seconds", duration.as_secs())
  } else if duration.as_secs() < 3600 {
    format!("{:.2} minutes", duration.as_secs() as f64 / 60.)
  } else {
    format!("{:.2} hours", duration.as_secs() as f64 / 3600.)
  }
}
