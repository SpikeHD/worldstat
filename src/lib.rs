//! A library for interfacing with Minecraft world information
//! 
//! This library provides a simple interface for getting information about a Minecraft world.
//! It supports both singleplayer and multiplayer worlds, so you can view and modify data for
//! other players.
//! 
//! # Usage
//! 
//! While some parts do not require providing a [Context](context::Context), most do, as they pertain to physical world data.
//! [Context](context::Context) is used to tell other parts of the library where to look for data. You can also specify
//! whether you want to read multiplayer or singleplayer data.
//! 
//! Many structs may *seem* to be missing fields, but they are actually just fields that are not explicitly deserialized. If you know your data
//! should have a certain field, you can still access it by using `<Struct>.get("your_field")`. This is useful for accessing custom data that is not
//! covered, or may even be custom (e.g. through mods).
//! 
//! # Examples
//! 
//! Get a players UUID and skin URLs from their username:
//! 
//! ```
//! use worldstat::player::Player;
//! 
//! let player = Player::new()
//!   .with_name("SpikeHD");
//! let uuid = player.uuid()?;
//! let skin_urls = player.skin_urls()?;
//! 
//! println!("Player UUID: {}", uuid);
//! println!("Player skin URLs: {:?}", skin_urls);
//! ```
//!
//! See how many oak planks a user has broken:
//! 
//! ```
//! use worldstat::{context::Context, player::Player};
//! 
//! let ctx = Context::new()
//!   .with_is_singleplayer(false)
//!   .with_path("./myworld");
//! let stats = Player::new()
//!   .with_name("SpikeHD")
//!   .with_ctx(ctx)
//!   .statistics()?;
//! 
//! let crafted = stats.crafted("minecraft:oak_planks")?;
//! let count = crafted.as_i64()?;
//! 
//! println!("SpikeHD has crafted {} oak planks", count);
//! ```
mod util;

pub mod context;
pub mod level;
pub mod player;
pub mod world;
