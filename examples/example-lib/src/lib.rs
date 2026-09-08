use lib_conf::LibConfig;
use secrecy::SecretString;
use std::time::Duration;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    // example notes:
    // - implicit default (e.g `#[config(default)]`, no `= ..`) is prohibited
    //   as the expression is directly in docs (to denote library default).
    /// Enables logging of the current sdk version during startup
    #[config(copy, default = false)]
    print_version: bool,

    // example notes:
    // - can not be configured with the Builder.
    // - only supports configuration via `MySdkOverrideConfig`
    /// API token
    #[config(builder_skip)]
    api_token: Option<SecretString>,

    /// Path to use for file logging.
    ///
    /// If unset, no logs are written to files.
    log_file_path: Option<String>,

    /// Refresh interval for content
    #[config(
        copy, default = Duration::from_secs(30),
        override_from = u64, override_via = SecondsAdapter,
    )]
    refresh_interval: Duration,

    // example notes:
    // - can only be set using the builder
    // - runtime-configuration is disabled for this field
    /// Name of the connecting client
    #[config(override_skip)]
    client_name: Option<String>,
}

/// Non-public struct used as an intermediate type to handle type conversion
/// between override -> config for mapped type
#[derive(Debug, Clone, Copy)]
pub(crate) struct SecondsAdapter(pub u64);

impl From<u64> for SecondsAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<SecondsAdapter> for Duration {
    fn from(adapter: SecondsAdapter) -> Self {
        Self::from_secs(adapter.0)
    }
}
