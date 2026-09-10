#![allow(unused, clippy::pedantic)]

use crate::internal::SecondsAdapter;

use lib_conf::LibConfig;
use std::time::Duration;

#[derive(Debug, Clone, LibConfig)]
pub struct Example1Config {
    // - Example Notes
    //
    // Specifying a literal value for the default here.
    // The library's default assignment will be added to all relevant method docs
    //
    // The copy attribute signals `lib-conf` to use the owned type for getter return types

    /// Enables verbose logging
    ///
    /// This control is provided in addition to the log level.
    /// When enabled, log messages will contain significantly more detail.
    #[config(copy, default = false)]
    pub(crate) verbose: bool,

    // - Example Notes
    //
    // Option is the only type that supports implicit defaults.
    // All **optional** fields that are not Option<_> must provide a default with an explicit value
    //
    // Generated getter's will automatically call as_ref()/as_str() where applicable.
    // The getter for this field will have a return type of `Option<&str>`

    /// Enables file logging at the given path
    ///
    /// If no path is provided, file logging is disabled.
    pub(crate) log_file_path: Option<String>,

    // - Example Notes
    //
    // We're specifying a custom type (u64) to deserialize runtime values with, as Duration lacks
    // a compatible deserialize impl.
    //
    // We provide an intermediate type that can handle From<u64> and Into<Duration>.
    // This type is only used by the Derive macro, and can remain private.
    //
    // We've also opted to use the more awkward `mins * 60` notation in the default value expr,
    // hopefully avoiding users from misinterpreting the unit of time we are using in the Override
    // struct.
    //
    // In reality, it would probably make more sense to use mins as the deserialize unit, but this
    // scenario provides conveys prioritizing clarity in default exprs for the sake of docs.

    /// Configures the log rotation interval.
    ///
    /// Has no effect if file logging isn't enabled
    ///
    /// **Note**: `ExampleOverride1Config` uses a `u64` to deserialize interval in seconds
    #[config(
        copy, default = Duration::from_secs(60 * 60),
        override_from = u64, override_via = SecondsAdapter,
    )]
    pub(crate) log_rotate_interval: Duration,
}

pub struct Example1Lib {
    _config: Example1Config,
}
impl Example1Lib {
    pub fn new(config: Example1Config) -> Self {
        Self {
            _config: config,
        }
    }
}

mod internal {
    use super::*;

    /// Handles mapping durations provided at runtime (chose to use sec units, but it's up to you)
    pub struct SecondsAdapter(u64);

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
}
