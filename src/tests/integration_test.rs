// tests/integration_test.rs
use p2p_messenger::telemetry;

#[tokio::test]
async fn test_message_delivery() {
    let _ = telemetry::try_init(); // Safe, no panic if already init

    // Test code with full tracing visibility...
}
