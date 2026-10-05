//! ⚶ Vesta — a native desktop super-app in Rust, built with egui/eframe for Windows and Linux.
//!
//! All application logic lives in this library crate so that `cargo doc` and the test suite
//! cover it; the `vesta` binary only calls [`run`].
//!
//! # Architecture
//!
//! [`app`] owns the single state container. UI code reads [`state`] and pushes [`action`]s;
//! the app applies them, routes between [`pages`] via [`router`], and persists preferences
//! through [`storage`] and [`settings`].
//!
//! # Modules
//!
//! - [`app`]: the `eframe::App` implementation and frame lifecycle.
//! - [`state`]: domain state and the pure state-transition function.
//! - [`router`]: the page routing enum.
//! - [`action`]: actions, the action queue and the cross-thread sender.
//! - [`error`]: typed errors and the crate `Result` alias.
//! - [`storage`]: config paths, atomic writes and JSON stores.
//! - [`settings`]: the versioned user settings model.
//! - [`i18n`]: locales and FR/EN/JA dictionaries.
//! - [`fonts`]: font discovery and the egui font fallback chain.
//! - [`theme`]: semantic colour tokens and Light/Dark visuals.
//! - [`toast`]: the toast notification manager and overlay.
//! - [`commands`]: the command registry, fuzzy search, shortcuts and palette.
//! - [`shell`]: the activity bar and status bar.
//! - [`pages`]: one module per route.
//! - [`platform`]: Windows/Linux specifics.

pub mod action;
pub mod app;
pub mod commands;
pub mod error;
pub mod fonts;
pub mod i18n;
pub mod pages;
pub mod platform;
pub mod router;
pub mod settings;
pub mod shell;
pub mod state;
pub mod storage;
pub mod theme;
pub mod toast;

use std::process::ExitCode;

/// Starts Vesta and returns the process exit code.
///
/// A failure that prevents the app from starting is logged and gives a failure exit code
/// instead of a crash.
#[must_use]
pub fn run() -> ExitCode {
    error::exit_code(start())
}

/// Starts the app.
///
/// Placeholder: the eframe bootstrap (VESTA-53) will open the main window here.
#[expect(clippy::unnecessary_wraps, reason = "the real start can fail")]
fn start() -> error::Result<()> {
    Ok(())
}
