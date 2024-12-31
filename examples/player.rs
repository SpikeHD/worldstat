use worldstat::player::Player;

fn main() {
  let mut player = Player::new().with_name("spikehd4k");

  println!("UUID: {}", player.uuid().unwrap());
}
