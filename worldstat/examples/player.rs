use worldstat::{context::Context, player::Player};

fn main() {
  let name = std::env::args().nth(1).unwrap_or("".to_string());
  let path = std::env::args().nth(2).unwrap_or("".to_string());

  if name.is_empty() {
    println!("Usage: player <name> [<path>]");
    return;
  }

  let mut player = Player::new().with_name(name);

  println!("UUID: {}", player.uuid().unwrap());
  println!("Skin URLs: {:#?}", player.skin_urls().unwrap().unwrap());

  if !path.is_empty() {
    let ctx = Context::new().with_path(path).with_is_singleplayer(true);
    let mut player = Player::new().with_ctx(ctx);

    println!(
      "Player data: {}",
      serde_json::to_string(&player.player_data().unwrap()).unwrap()
    );
  } else {
    println!("To read full playerdata from a world, pass the world path as an argument");
  }
}
