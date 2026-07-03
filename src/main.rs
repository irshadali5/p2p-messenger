//! Binary entry point.

use p2p_messenger::telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize telemetry FIRST
    let _telemetry_guard = telemetry::init();

    tracing::info!("P2P Messenger starting up");

    // TODO: Parse CLI args, load config, run app
    println!("P2P Messenger started. Press Ctrl+C to exit.");

    // Keep running
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down");

    Ok(())
}
