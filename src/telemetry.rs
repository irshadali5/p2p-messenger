//! Observability setup: tracing subscriber initialization.
//!
//! Call `init()` once at application startup.

use std::io;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
    Layer, // ← REQUIRED for .boxed()
};

/// Guard returned from init to keep the non-blocking writer alive.
pub struct TelemetryGuard {
    _non_blocking_guard: Option<WorkerGuard>,
}

/// Initialize global tracing subscriber.
pub fn init() -> TelemetryGuard {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info")
            .add_directive("p2p_messenger=debug".parse().unwrap())
            .add_directive("iroh=warn".parse().unwrap())
    });

    let is_production = std::env::var("ENVIRONMENT")
        .map(|v| v == "production")
        .unwrap_or(false);

    let (non_blocking_guard, fmt_layer): (
        Option<WorkerGuard>,
        Box<dyn tracing_subscriber::Layer<tracing_subscriber::Registry> + Send + Sync>,
    ) = if is_production {
        let (file_appender, guard) = tracing_appender::non_blocking(
            tracing_appender::rolling::daily("logs", "p2p-messenger.log"),
        );

        let layer = fmt::layer()
            .json()
            .with_current_span(true)
            .with_span_list(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_target(true)
            .with_span_events(FmtSpan::CLOSE)
            .flatten_event(true)
            .with_writer(file_appender)
            .boxed(); // ← Now works because Layer trait is in scope

        (Some(guard), layer)
    } else {
        let layer = fmt::layer()
            .pretty()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::ACTIVE)
            .with_writer(io::stderr)
            .boxed();

        (None, layer)
    };

    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        // .with(tracing_error::ErrorLayer::default()) // ← Uncomment if you add tracing-error crate
        .init();

    tracing::info!("Telemetry initialized");

    TelemetryGuard {
        _non_blocking_guard: non_blocking_guard,
    }
}

/// Try-init for tests — does not panic on double-init.
pub fn try_init() -> Option<TelemetryGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let fmt_layer = fmt::layer().with_test_writer().with_target(false).boxed();

    match tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
    {
        Ok(()) => Some(TelemetryGuard {
            _non_blocking_guard: None,
        }),
        Err(_) => None,
    }
}
