// src/telemetry.rs
//! Observability setup: tracing subscriber, metrics, profiling.
//!
//! Call `init()` once at application startup. Safe to call in tests via `try_init()`.

use std::io;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    EnvFilter,
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

/// Guard returned from init to keep the non-blocking writer alive.
/// Drop this only when the application exits.
pub struct TelemetryGuard {
    _non_blocking_guard: Option<WorkerGuard>,
}

/// Initialize global tracing subscriber.
///
/// # Panics
/// Panics if called twice in the same process without `try_init()` semantics.
/// In tests, use `try_init()` instead.
///
/// # Example
/// ```no_run
/// use p2p_messenger::telemetry;
/// let _guard = telemetry::init();
/// ```
pub fn init() -> TelemetryGuard {
    // 1. Environment filter from RUST_LOG
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::new("info")
            .add_directive("p2p_messenger=debug".parse().unwrap())
            .add_directive("iroh=warn".parse().unwrap())
    });

    // 2. Determine output mode: pretty (dev) vs JSON (prod)
    let is_production = std::env::var("ENVIRONMENT")
        .map(|v| v == "production")
        .unwrap_or(false);

    let (non_blocking_guard, fmt_layer) = if is_production {
        // Production: JSON to stdout + optional file appender
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
            .with_writer(file_appender);

        (Some(guard), layer.boxed())
    } else {
        // Development: pretty colored output to stderr
        let layer = fmt::layer()
            .pretty()
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::ACTIVE)
            .with_writer(io::stderr);

        (None, layer.boxed())
    };

    // 3. Build and install global subscriber
    tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

    tracing::info!("Telemetry initialized");

    TelemetryGuard {
        _non_blocking_guard: non_blocking_guard,
    }
}

/// Try-init for tests — does not panic on double-init.
///
/// # Example
/// ```rust
/// # use p2p_messenger::telemetry;
/// # fn test_something() {
/// let _ = telemetry::try_init();
/// # }
/// ```
pub fn try_init() -> Option<TelemetryGuard> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let fmt_layer = fmt::layer().with_test_writer().with_target(false);

    // Use try_init() which returns Result instead of panicking
    match tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()
    {
        Ok(()) => {
            tracing::debug!("Test telemetry initialized");
            Some(TelemetryGuard {
                _non_blocking_guard: None,
            })
        }
        Err(_) => {
            // Subscriber already set, likely by another test
            None
        }
    }
}
