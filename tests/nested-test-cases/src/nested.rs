use lib_conf::LibConfig;
use crate::shared::{validate_conf, ValidationError};

#[derive(LibConfig, Debug, Clone)]
pub struct WithDefaultConfig {
    #[config(copy, default = false)]
    pub(crate) foo: bool,
}

#[derive(LibConfig, Debug, Clone)]
pub struct WithRequiredConfig {
    #[config(copy)]
    pub(crate) foo: bool,
}

#[derive(LibConfig, Debug, Clone)]
pub struct WithOverrideRequiredConfig {
    #[config(copy, override_required)]
    pub(crate) foo: bool,
}

#[derive(LibConfig, Debug, Clone)]
pub struct WithRequiredAndOverrideSkipConfig {
    #[config(copy, override_skip)]
    pub(crate) foo: bool,
}

#[derive(LibConfig, Debug, Clone)]
#[config(validate = validate_conf, validate_error = ValidationError)]
pub struct WithValidationConfig {
    #[config(copy, default = true)]
    pub(crate) foo: bool,
}
