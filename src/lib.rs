//! # SAMP Query
//!
//! Implementation of the SA-MP Query Mechanism.
//!
//! This library provides a complete implementation of the SA-MP Query protocol,
//! allowing you to query SA-MP servers for information such as player lists,
//! server rules, and detailed player information.
//!
//! ## Features
//!
//! - Full implementation of all query types (information, rules, clients, detailed player data)
//! - Robust packet handling with proper timeout and error management
//! - Memory-safe and efficient data structures
//! - Complete protocol compliance with the SA-MP Query Mechanism
//!
//! ## Example
//!
//! ```rust,no_run
//! use samp_query::{Client, QueryType};
//! use std::net::SocketAddr;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
//!     let addr: SocketAddr = "127.0.0.1:7777".parse()?;
//!     let client = Client::connect(addr).await?;
//!
//!     // Query server information
//!     let info = client.query_info().await?;
//!     println!("Server: {}", info.hostname);
//!     println!("Players: {}/{}", info.players, info.max_players);
//!     println!("Gamemode: {}", info.gamemode);
//!
//!     Ok(())
//! }
//! ```

pub use client::{Client, ClientConfig};
pub use error::{Error, Result};
pub use protocol::QueryType;
pub use types::*;

pub mod client;
pub mod error;
#[cfg(any(test, feature = "benchmarks"))]
pub mod packet;
#[cfg(not(any(test, feature = "benchmarks")))]
mod packet;
pub mod protocol;
pub mod types;

pub mod utils;
