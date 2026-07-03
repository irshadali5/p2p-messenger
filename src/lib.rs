//! P2P Messenger library crate.

pub mod config;
pub mod crypto;
pub mod error;
pub mod messaging;
pub mod network;
pub mod storage;
pub mod telemetry;
pub mod ui;

// REMOVED: pub use cli::Args;  // This was broken - cli module doesn't exist yet
// REMOVED: pub mod cli;        // Add this back when you actually create cli.rs
