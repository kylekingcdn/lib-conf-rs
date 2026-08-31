#![allow(unused)]

use lib_conf::LibConfig;
use secrecy::SecretString;
use std::time::Duration;

#[derive(Debug, Clone, LibConfig)]
pub struct MySdkConfig {
    /// Enables logging of the current sdk version during startup
    //
    // example notes:
    // - implicit default (e.g `#[config_builder(default)]`) is prohibited as the
    //   expression is directly used to show the default value.
    #[config(copy, default = false)]
    print_version: bool,

    /// API token (can only be set st runtime via override config)
    #[config(builder_skip)]
    api_token: Option<SecretString>,

    log_file_path: Option<String>,

    // /// Some text that uniquely identifies the running instance (can only be set at runtime, required)
    // NOTE: making an override-only option required effectively forces your users into using env parsing via crates
    // such as `dotenvy`, `config`.
    //
    // Unless your crate is intended for local/internal use or is a "supporting" crate for
    // your own binary, **this pattern is strongly discouraged**.
    // #[config_builder(skip)]
    // #[override_config(required)]
    // instance_id: String,

    /// Refresh interval for content
    #[config(copy, default = Duration::from_secs(30), override_from = u64, override_via = InternalSecNewtype)]
    refresh_interval: Duration,

    /// Name of the connecting client
    ///
    /// Testing multiline comment
    ///
    /// What will attrs look like
    // can only be set from init code
    // no default value, so this will be placed as a required param in the builder constructor
    #[config(override_skip)]
    client_name: Option<String>,
}

// one-off internal type used as an intermediate type for override -> config

struct InternalSecNewtype(u64);
impl From<u64> for InternalSecNewtype {
    fn from(secs: u64) -> Self {
        Self(secs)
    }
}
impl From<InternalSecNewtype> for Duration {
    fn from(secs: InternalSecNewtype) -> Self {
        Self::from_secs(secs.0)
    }
}
