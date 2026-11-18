use crate::{RuntimeBackend, RuntimeThreading};

use lib_conf::LibConfig;

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Backend `{backend:#?} is not compatible with threading mode `{threading:#?}`")]
    IncompatibleRuntimeThreading { backend: RuntimeBackend, threading: RuntimeThreading },
}

/// Settings for `example6` of `lib-conf`
#[derive(Debug, Copy, Clone, LibConfig)]
#[config(
    example_field_name = "example6",
    validate = Example6Config::validate,
    validate_error = ConfigError,
)] //                ^^^^^^^^^^^ inform `LibConfig` of our error type used in the validate fn
pub struct Example6Config {
    // - Example Notes
    //
    // We add a `validate` attr to the main config struct.
    // This lets the derive macro:
    // - know that configuration is fallible
    // - the validation fn to execute as the last step in the builder's `build()` method
    //
    // As such, the builder's build() method return type changes:
    // - from `Example6Config`
    // - to `Result<Example6Config, ConfigError>`
    //
    // Therefore, any change in validation inclusion or in the error type will be
    // a BREAKING CHANGE.

    /// Enables verbose logging
    #[config(copy, default = false)]
    pub(crate) verbose: bool,

    /// Backend runtime selection
    #[config(copy, default = RuntimeBackend::Futures)]
    pub(crate) backend: RuntimeBackend,

    /// Backend runtime threading selection
    #[config(copy, default = RuntimeThreading::Current)]
    pub(crate) threading: RuntimeThreading,
}
impl Example6Config {
    // will be called from builder
    pub(crate) fn validate(&self) -> Result<(), ConfigError> {
        if self.backend.supports_threading_strategy(self.threading) {
            Ok(())
        } else {
            Err(ConfigError::IncompatibleRuntimeThreading {
                backend: self.backend,
                threading: self.threading,
            })
        }
    }
}

// Validate fn doesn't need to be a member of the config struct.
// This would be valid as well:
//
//fn validate(conf: &Example6Config) -> bool {
//    ..
//}
