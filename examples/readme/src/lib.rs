//! This is an example library for [`lib_conf`] and does not provide actual functionality.
/// This is an example library for [`lib_conf`] and does not provide actual functionality.
pub mod crab_log {
    use lib_conf::LibConfig;

    /// Settings for `crab-log`
    #[derive(Debug, Clone, LibConfig)]
    pub struct CrabLogConfig {
        /// Enables the Logger library
        #[config(copy, default = true)]
        pub(crate) enabled: bool,

        /// Enables verbose logging
        #[config(copy, default = false)]
        pub(crate) verbose: bool,

        /// Enables file logging at the given path
        ///
        /// If no path is provided, file logging is disabled.
        pub(crate) log_file_path: Option<String>,
    }
}
