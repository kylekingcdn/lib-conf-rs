//! This is an example library for [`lib_conf`] and does not provide actual functionality.
#![allow(unused, clippy::pedantic)]

use lib_conf::LibConfig;

/// Settings for `example2` of `lib-conf`
#[derive(Debug, Clone, LibConfig)]
pub struct Example2Config {
    /// Enables verbose logging
    ///
    /// This control is provided in addition to the log level.
    /// When enabled, log messages will contain significantly more detail.
    #[config(copy, default = false)]
    pub(crate) verbose: bool,

    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    pub(crate) log_file_path: Option<String>,
}

/// Primary interface for example 2's library
pub struct Example2Lib {
    _config: Example2Config,
}
impl Example2Lib {
    pub fn new(config: Example2Config) -> Self {
        Self {
            _config: config,
        }
    }
}
