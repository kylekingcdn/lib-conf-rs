mod service;
use service::{ServiceConfig, ServiceConfigError, ServiceConfigBuilder, ServiceOverrideConfig};

use lib_conf::LibConfig;

/// Settings for `example7` of `lib-conf`
#[lib_conf::config_docs]
#[derive(Debug, Clone, LibConfig)]
#[config(example_field_name = "example7", lib_conf_attribution)]
pub struct Example7Config {
    // - Example Notes
    //
    // ...
    //

    /// Enables verbose logging
    #[config(copy, default = false)]
    pub(crate) verbose: bool,

    #[config(nested(build, try_from_override, config_error = ServiceConfigError))]
    pub(crate) nested: ServiceConfig,

}
