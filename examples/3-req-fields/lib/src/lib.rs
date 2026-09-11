#![allow(unused, clippy::pedantic)]

use lib_conf::LibConfig;

/// Settings for `example3` of `lib-conf`
#[derive(Debug, Clone, LibConfig)]
pub struct Example3Config {
    // - Example notes
    // By default, override support is still enabled for required fields
    //
    // Use the `override_skip` attr to block a field from usage in the Override struct.
    //
    // In this example, `enable_telemetry` can be modified at runtime.
    // However, client_name can not be updated at runtime.

    /// Name of the client app using the library
    ///
    /// This field is mandatory and cannot be changed at runtime
    #[config(override_skip)]
    pub(crate) client_name: String,

    /// Enables usage telemetry
    ///
    /// This field is mandatory
    pub(crate) enable_telemetry: bool,

    /// Enables verbose logging
    #[config(copy, default = false)]
    pub(crate) verbose: bool,
}
