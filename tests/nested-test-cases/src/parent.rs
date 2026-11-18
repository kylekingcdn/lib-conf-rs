use crate::nested::*;
use lib_conf::LibConfig;

// !- Inner default

#[derive(LibConfig, Debug, Clone)]
pub struct DefContainsDefaultConfig {
    #[config(
        copy,
        default = WithDefaultConfig::builder().foo(true).build(),
        nested(builder_default, from_override, build),
    )]
    pub(crate) foo: WithDefaultConfig,
}

#[derive(LibConfig, Debug, Clone)]
pub struct ReqContainsDefaultConfig {
    #[config(
        copy,
        nested(builder_default, from_override, build),
    )]
    pub(crate) foo: WithDefaultConfig,
}

#[derive(LibConfig, Debug, Clone)]
pub struct OptContainsDefaultConfig {
    #[config(
        copy,
        nested(builder_default, from_override, build),
    )]
    pub(crate) foo: Option<WithDefaultConfig>,
}

// !- Inner validation

#[derive(LibConfig, Debug, Clone)]
pub struct DefContainsValidationConfig {
    #[config(
        copy,
        default = WithValidationConfig::default(),
        nested(builder_default, try_build, try_from_override, config_error = WithValidationConfigError),
    )]
    pub(crate) foo: WithValidationConfig,
}
#[derive(LibConfig, Debug, Clone)]
pub struct OptContainsValidationConfig {
    #[config(
        copy,
        nested(builder_default, try_build, try_from_override, config_error = WithValidationConfigError),
    )]
    pub(crate) foo: Option<WithValidationConfig>,
}
#[derive(LibConfig, Debug, Clone)]
pub struct ReqContainsValidationConfig {
    #[config(
        copy,
        nested(builder_default, try_build, try_from_override, config_error = WithValidationConfigError),
    )]
    pub(crate) foo: WithValidationConfig,
}

// !- Inner required

#[derive(LibConfig, Debug, Clone)]
pub struct DefContainsRequiredConfig {
    #[config(
        copy,
        default = WithRequiredConfig::builder(true).build(),
        nested(build, try_from_override, config_error = WithRequiredConfigError),
    )]
    pub(crate) foo: WithRequiredConfig,
}

#[derive(LibConfig, Debug, Clone)]
pub struct OptContainsRequiredConfig {
    #[config(
        copy,
        nested(build, try_from_override, config_error = WithRequiredConfigError),
    )]
    pub(crate) foo: Option<WithRequiredConfig>,
}

#[derive(LibConfig, Debug, Clone)]
pub struct ReqContainsRequiredConfig {
    #[config(
        copy,
        nested(build, try_from_override, config_error = WithRequiredConfigError),
    )]
    pub(crate) foo: WithRequiredConfig,
}

// !- Inner override required

#[derive(LibConfig, Debug, Clone)]
pub struct DefContainsOverrideRequiredConfig {
    #[config(
        copy,
        default = WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig { foo: true }).build(),
        nested(build, from_override)
    )]
    pub(crate) foo: WithOverrideRequiredConfig,
}

#[derive(LibConfig, Debug, Clone)]
pub struct OptContainsOverrideRequiredConfig {
    #[config(
        copy,
        nested(build, from_override)
    )]
    pub(crate) foo: Option<WithOverrideRequiredConfig>,
}

#[derive(LibConfig, Debug, Clone)]
pub struct ReqContainsOverrideRequiredConfig {
    #[config(
        copy,
        nested(build, from_override)
    )]
    pub(crate) foo: WithOverrideRequiredConfig,
}

// !- Inner required + override skip

#[derive(LibConfig, Debug, Clone)]
//#[config(vote_debug)]
pub struct DefContainsRequiredAndOverrideSkipConfig {
    #[config(
        copy,
        default = WithRequiredAndOverrideSkipConfig::builder(false).build(),
        nested(build, no_from_override),
    )]
    pub(crate) foo: WithRequiredAndOverrideSkipConfig,
}

// fails - as intended
// #[derive(LibConfig, Debug, Clone)]
// pub struct OptContainsRequiredAndOverrideSkipConfig {
//     #[config(
//         copy,
//         nested(build, no_from_override),
//     )]
//     pub(crate) foo: Option<WithRequiredAndOverrideSkipConfig>,
// }

#[derive(LibConfig, Debug, Clone)]
pub struct ReqContainsRequiredAndOverrideSkipConfig {
    #[config(
        copy,
        nested(build, no_from_override),
    )]
    pub(crate) foo: WithRequiredAndOverrideSkipConfig,
}
