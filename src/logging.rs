//! Diagnostic logging: installs the `log` back end and writes the startup summary.
//!
//! By default the output is quiet: warnings and errors from every crate, plus Vesta's own
//! information messages (the startup summary). Set the `RUST_LOG` environment variable to see
//! more, for example `RUST_LOG=debug`; it uses the filter syntax of `env_logger`.

use std::env;
use std::path::Path;

use env_logger::{Builder, DEFAULT_FILTER_ENV};

/// The filter used when `RUST_LOG` is not set.
const DEFAULT_FILTER: &str = "warn,vesta=info";

/// Returns the logger configuration for `spec`, written in the `RUST_LOG` syntax, or for the
/// default filter if there is none.
#[must_use]
pub fn builder(spec: Option<&str>) -> Builder {
    let mut builder = Builder::new();
    builder.parse_filters(spec.unwrap_or(DEFAULT_FILTER));
    builder
}

/// Installs the logger, filtered by the `RUST_LOG` environment variable if it is set.
///
/// If a logger is already installed, it is kept and a warning is logged.
pub fn init() {
    let spec = env::var(DEFAULT_FILTER_ENV).ok();
    match builder(spec.as_deref()).try_init() {
        Ok(()) => log::debug!("log filter: {}", spec.as_deref().unwrap_or(DEFAULT_FILTER)),
        Err(_) => log::warn!("a logger is already installed; keeping it"),
    }
}

/// Logs, at `info` level, the app version, the operating system and the settings directory.
pub fn log_startup(settings_dir: &Path) {
    log::info!(
        "Vesta {} on {} {}; settings in {}",
        env!("CARGO_PKG_VERSION"),
        env::consts::OS,
        env::consts::ARCH,
        settings_dir.display(),
    );
}
