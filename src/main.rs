// src/main.rs
//! Binary entry point. THIN: only CLI parsing, telemetry init, then delegate to lib.

use color_eyre::Result;
use p2p_messenger::{App, cli, telemetry};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Initialize telemetry FIRST — before anything else
    // This must happen before any tracing macros are used
    let _telemetry_guard = telemetry::init();

    // 2. Now it's safe to use tracing
    tracing::info!("P2P Messenger starting up");

    // 3. Parse CLI arguments (uses clap)
    let args = cli::Args::parse();

    // 4. Load configuration
    let config = cli::load_config(&args)?;

    // 5. Run the application (defined in lib)
    App::new(config).run().await
}
