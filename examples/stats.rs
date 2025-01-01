use worldstat::{context::Context, player::Player};

fn main() {
  let path = std::env::args().nth(1).unwrap();
  let user = std::env::args().nth(2).unwrap();
  let ctx = Context::new()
    .with_path(path);
  let player = Player::new()
    .with_name(user)
    .with_ctx(ctx)
    .statistics()
    .unwrap();

  let crafted = player.crafted("minecraft:oak_planks").unwrap();
  let count = crafted.as_i64().unwrap_or(0);

  println!("SpikeHD has crafted {} oak planks", count);
}