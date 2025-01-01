//! A library for interfacing with Minecraft world information
//! 
//! This library provides a simple interface for getting information about a Minecraft world.
//! It supports both singleplayer and multiplayer worlds, so you can view and modify data for
//! other players.
//! 
//! # Examples
//! 
//! Get a players UUID from their username:
//! 
//! ```
//! use worldstat::player::Player;
//! 
//! let player = Player::new()
//!   .with_name("SpikeHD")
//!   .uuid()
//!   .unwrap();
//! 
//! println!("Player UUID: {}", player);
//! ```
//!
//! See how many oak planks a user has broken:
//! 
//! ```
//! use worldstat::player::Player;
//! 
//! let ctx = Context::new()
//!   .with_path("./myworld");
//! let player = Player::new()
//!   .with_name("SpikeHD")
//!   .with_ctx(ctx)
//!   .statistics()
//!   .unwrap();
//! 
//! let broken = player.broken("minecraft:planks").unwrap();
//! let count = broken.as_i64().unwrap();
//! 
//! println!("SpikeHD has broken {} oak planks", count);
//! ```
pub mod context;
pub mod level;
pub mod player;
pub mod world;
