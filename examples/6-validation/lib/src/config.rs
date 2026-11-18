use crate::{RuntimeBackend, RuntimeThreading};

use lib_conf::LibConfig;

#[derive(Debug, thiserror::Error)]
pub enum ValidationError {
    #[error("Backend `{backend:#?} is not compatible with threading mode `{threading:#?}`")]
    IncompatibleRuntimeThreading { backend: RuntimeBackend, threading: RuntimeThreading },
}

/// Settings for `example6` of `lib-conf`
#[derive(Debug, Clone, LibConfig)]
#[config(
    example_field_name = "example6",
    validate = Example6Config::validate,
    validate_error = ValidationError,
)] //                ^^^^^^^^^^^^^^^ inform `LibConfig` of our error type used in the validate fn
pub struct Example6Config {
    // - Example Notes
    //
    // We add a `validate` attr to the main config struct.
    //
    // This lets the derive macro:
    // - know that configuration is fallible
    // - the validation fn to execute as the last step in the builder's `build()` method
    //
    // As such, 2 significant changes to the builder's build method occur
    //
    // 1. The return type changes from
    //
    //    > `Example6Config`
    //    to
    //    > `Result<Example6Config, ConfigError>`
    //
    // 2. Following rust best practices, the fn name changes from
    //
    //    > `build()`
    //    to
    //    > `try_build()`
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
    pub(crate) fn validate(&self) -> Result<(), ValidationError> {
        if self.backend.supports_threading_strategy(self.threading) {
            Ok(())
        } else {
            Err(ValidationError::IncompatibleRuntimeThreading {
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
