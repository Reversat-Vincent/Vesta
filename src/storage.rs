//! Persistence: OS config directory resolution, atomic `.tmp` + rename writes and JSON stores.

use std::path::PathBuf;

use crate::error::{Result, VestaError};

/// Name of Vesta's folder inside the user's configuration directory.
const APP_DIR: &str = "Vesta";

/// Returns the directory where Vesta stores its settings: the `Vesta` folder inside the user's
/// configuration directory (the application-data folder on Windows, `$XDG_CONFIG_HOME` or
/// `~/.config` on Linux).
///
/// # Errors
///
/// Returns [`VestaError::ConfigDirUnavailable`] if the system provides no configuration
/// directory.
pub fn settings_dir() -> Result<PathBuf> {
    settings_dir_in(dirs::config_dir())
}

/// Returns Vesta's settings directory inside `config_dir`, the user's configuration directory.
///
/// # Errors
///
/// Returns [`VestaError::ConfigDirUnavailable`] if `config_dir` is `None`.
pub fn settings_dir_in(config_dir: Option<PathBuf>) -> Result<PathBuf> {
    config_dir
        .map(|dir| dir.join(APP_DIR))
        .ok_or(VestaError::ConfigDirUnavailable)
}
