<div align="center">
  <h1>WorldStat</h1>
  <p>All-in-one Rust library for interfacing with Minecraft world information</p>
</div>

<div align="center">
  <img src="https://img.shields.io/github/actions/workflow/status/SpikeHD/worldstat/build.yml" />
  <img src="https://img.shields.io/github/repo-size/SpikeHD/worldstat" />
  <img src="https://img.shields.io/github/commit-activity/m/SpikeHD/worldstat" />
</div>

<div align="center">
  <img src="https://img.shields.io/github/release-date/SpikeHD/worldstat" />
  <img src="https://img.shields.io/github/stars/SpikeHD/worldstat" />
</div>

*Looking for the CLI tool? Check the [GitHub!](https://github.com/SpikeHD/worldstat)*

`worldstat` provides a simple interface for getting information about a Minecraft world.
It supports both singleplayer and multiplayer worlds, so you can view and modify data for
other players.

Documentation is available at [docs.rs](https://docs.rs/worldstat).

# Examples

Get a players UUID and skin URLs from their username:

```rust
use worldstat::player::Player;

let player = Player::new()
  .with_name("SpikeHD");
let uuid = player.uuid()?;
let skin_urls = player.skin_urls()?;

println!("Player UUID: {}", uuid);
println!("Player skin URLs: {:?}", skin_urls);
```

See how many oak planks a user has broken:

```rust
use worldstat::{context::Context, player::Player};

let ctx = Context::new()
  .with_is_singleplayer(false)
  .with_path("./myworld");
let stats = Player::new()
  .with_name("SpikeHD")
  .with_ctx(ctx)
  .statistics()?;

let crafted = stats.crafted("minecraft:oak_planks")?;
let count = crafted.as_i64()?;

println!("SpikeHD has crafted {} oak planks", count);
```