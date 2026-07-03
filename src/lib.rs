// src/lib.rs
//! P2P Messenger library crate.
//!
//! Can be used as a library or via the binary.

pub mod config;
pub mod crypto;
pub mod error;
pub mod messaging;
pub mod network;
pub mod storage;
pub mod telemetry;
pub mod ui;

pub use cli::Args;
pub use ui::App;

pub mod cli {
    pub use crate::config::Config;
    // ... CLI parsing logic
}
