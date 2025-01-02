use worldstat::world::World;

fn main() {
  let path = std::env::args().nth(1).unwrap();

  if path.is_empty() {
    println!("Usage: world <path>");
    return;
  }

  let ctx = worldstat::context::Context::new().with_path(path);
  let world = World::new(ctx).unwrap();

  println!("Name: {:#?}", world.world_data.level_name);
  println!("Current time: {:#?}", world.world_data.time);
  println!(
    "Last played (unix time): {:#?}",
    world.world_data.last_played
  );
  println!("Game version: {:#?}", world.world_data.version.name);
  println!(
    "Spawn coordinates: {:#?}",
    (
      world.world_data.spawn_x,
      world.world_data.spawn_y,
      world.world_data.spawn_z
    )
  );
}
