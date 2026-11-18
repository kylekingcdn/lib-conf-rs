use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct OptContainsValidationConfig {
    pub(crate) foo: Option<WithValidationConfig>,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl OptContainsValidationConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithValidationConfig > : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithValidationConfig > : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithValidationConfigBuilder : ::std::default::Default
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithValidationConfig : ::lib_conf::traits::merge::TryMergeOverride < Override
            = WithValidationOverrideConfig, Error = WithValidationConfigError >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithValidationConfigBuilder : ::lib_conf::traits::build::TryBuildConfig <
            Config = WithValidationConfig, Error = WithValidationConfigError >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithValidationConfig : ::std::convert::TryFrom <
            WithValidationOverrideConfig, Error = WithValidationConfigError >
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new() -> Self {
        Self { foo: None }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> OptContainsValidationConfigBuilder {
        <OptContainsValidationConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> Option<WithValidationConfig> {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn try_merge_with_override(
        &mut self,
        override_conf: OptContainsValidationOverrideConfig,
    ) -> ::std::result::Result<(), OptContainsValidationConfigError> {
        <Self as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsValidationConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for OptContainsValidationConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::TryMergeOverride for OptContainsValidationConfig {
    type Override = OptContainsValidationOverrideConfig;
    type Error = OptContainsValidationConfigError;
    fn try_merge_with_override(
        &mut self,
        override_conf: Self::Override,
    ) -> ::std::result::Result<(), Self::Error> {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                if let ::std::option::Option::Some(current) = &mut self.foo {
                    <WithValidationConfig as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
                            current,
                            val,
                        )
                        .map_err(|error| OptContainsValidationConfigError::FooField {
                            error,
                        })?;
                } else {
                    let mut inited = <WithValidationConfig>::try_from(val)
                        .map_err(|error| OptContainsValidationConfigError::FooField {
                            error,
                        })?;
                    self.foo = ::std::option::Option::Some(inited);
                }
            }
        }
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<OptContainsValidationOverrideConfig>
for OptContainsValidationConfig {
    type Error = OptContainsValidationConfigError;
    fn try_from(
        override_conf: OptContainsValidationOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    ::std::option::Option::Some(
                        <WithValidationConfig>::try_from(val)
                            .map_err(|error| OptContainsValidationConfigError::FooField {
                                error,
                            })?,
                    )
                } else {
                    ::std::option::Option::None
                }
            },
        };
        ::std::result::Result::Ok(out)
    }
}
/// Possible errors encountered during builf of [`OptContainsValidationConfig`](struct@OptContainsValidationConfig)
#[derive(::std::fmt::Debug)]
pub enum OptContainsValidationConfigError {
    /// Failed to merge nested [`WithValidationConfig`](struct@WithValidationConfig)
    FooField {
        /// Corresponds to the error type returned by [`WithValidationConfig::try_merge()`](method@WithValidationConfig::try_merge)
        error: WithValidationConfigError,
    },
}
impl OptContainsValidationConfigError {
    /// List of field names leading to the source of a nested error.
    ///
    /// Does not include the offending field(s) themselves, just the sequence of fields leading to their containing `Struct`
    pub fn source_chain(&self) -> ::std::vec::Vec<::std::string::String> {
        <Self as ::lib_conf::traits::error::ConfigError>::source_chain(self)
    }
    /// Same as [`source_chain()`](Self::source_chain), but returns the chain as a single dot-delimited string
    pub fn source_path(&self) -> ::std::string::String {
        <Self as ::lib_conf::traits::error::ConfigError>::source_path(self)
    }
    /// Returns the source error message
    ///
    /// Used to generate the full error including the nested path
    pub fn origin_message(&self) -> ::std::string::String {
        <Self as ::lib_conf::traits::error::ConfigError>::origin_message(self)
    }
}
#[automatically_derived]
impl ::std::fmt::Display for OptContainsValidationConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::FooField { error } => {
                let path = error.source_path();
                let origin_message = error.origin_message();
                format!("Error in nested config at `{path}`: {origin_message}")
            }
        };
        write!(f, "{text}")?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::error::Error for OptContainsValidationConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for OptContainsValidationConfigError {
    fn build_field_chain(
        &self,
        chain: &mut ::std::vec::Vec<::std::string::String>,
        self_field_name: ::std::option::Option<&str>,
    ) {
        if let ::std::option::Option::Some(name) = self_field_name {
            chain.push(name.to_string());
        }
        match self {
            Self::FooField { error } => {
                error.build_field_chain(chain, ::std::option::Option::Some("foo"))
            }
            _ => {}
        };
    }
    fn origin_message(&self) -> ::std::string::String {
        match self {
            Self::FooField { error } => error.origin_message(),
            _ => self.to_string(),
        }
    }
}
/// Provides a builder used to configure [`OptContainsValidationConfig`](struct@OptContainsValidationConfig) at compile-time.
///
/// Can be constructed using either [`OptContainsValidationConfig::builder()`](method@OptContainsValidationConfig::builder) or [`OptContainsValidationConfigBuilder::new()`](method@OptContainsValidationConfigBuilder::new).
///
/// For run-time configuration of `OptContainsValidationConfig`, see [`OptContainsValidationOverrideConfig`](struct@OptContainsValidationOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct OptContainsValidationConfigBuilder {
    pub(crate) inner: OptContainsValidationConfig,
    pub(crate) override_conf: ::std::option::Option<OptContainsValidationOverrideConfig>,
}
impl OptContainsValidationConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <OptContainsValidationConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub(crate) fn foo(mut self, val: Option<WithValidationConfig>) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithValidationConfig`](struct@WithValidationConfig)
    ///
    /// Closure has a single [`WithValidationConfigBuilder`](struct@WithValidationConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithValidationConfigBuilder`](struct@WithValidationConfigBuilder) returned by the closure will automatically be built (and associated [`WithValidationOverrideConfig`](struct@WithValidationOverrideConfig) merged).
    #[must_use]
    pub fn try_foo_builder<F>(
        mut self,
        f: F,
    ) -> ::std::result::Result<Self, WithValidationConfigError>
    where
        F: Fn() -> WithValidationConfigBuilder,
    {
        let builder = f();
        self.inner.foo = ::std::option::Option::Some(
            <WithValidationConfigBuilder as ::lib_conf::traits::build::TryBuildConfig>::try_build(
                builder,
            )?,
        );
        ::std::result::Result::Ok(self)
    }
    /// Allows for inline configuration of nested [`WithValidationConfig`](struct@WithValidationConfig)
    ///
    /// Closure has a single [`WithValidationConfigBuilder`](struct@WithValidationConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithValidationConfigBuilder`](struct@WithValidationConfigBuilder) returned by the closure will automatically be built (and associated [`WithValidationOverrideConfig`](struct@WithValidationOverrideConfig) merged).
    #[must_use]
    pub fn try_foo_builder_default<F>(
        mut self,
        f: F,
    ) -> ::std::result::Result<Self, WithValidationConfigError>
    where
        F: Fn(WithValidationConfigBuilder) -> WithValidationConfigBuilder,
    {
        let builder = f(<WithValidationConfigBuilder>::default());
        self.inner.foo = ::std::option::Option::Some(
            <WithValidationConfigBuilder as ::lib_conf::traits::build::TryBuildConfig>::try_build(
                builder,
            )?,
        );
        ::std::result::Result::Ok(self)
    }
    /// If supplied, will overwrite any values present in the provided
    /// override config.
    ///
    /// This happens as the last step within the
    ///[`build()`](self.build) method.
    ///
    /// Any previous calls to [`with_override()`](Self::with_override)
    /// in the builder chain will have no effect on the resulting
    /// config.
    ///
    /// The order in which this is chained with the builder's setter
    /// methods does not matter.
    #[must_use]
    pub fn with_override(
        mut self,
        override_conf: OptContainsValidationOverrideConfig,
    ) -> Self {
        self.override_conf = ::std::option::Option::Some(override_conf);
        self
    }
    /// Clears the override config previously set via
    /// [`with_override()`](Self::with_override).
    #[must_use]
    pub fn clear_override(mut self) -> Self {
        self.override_conf = ::std::option::Option::None;
        self
    }
    /// Builds the [`OptContainsValidationConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn try_build(
        mut self,
    ) -> ::std::result::Result<
        OptContainsValidationConfig,
        OptContainsValidationConfigError,
    > {
        <Self as ::lib_conf::traits::build::TryBuildConfig>::try_build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsValidationConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::TryBuildConfig for OptContainsValidationConfigBuilder {
    type Config = OptContainsValidationConfig;
    type Error = OptContainsValidationConfigError;
    fn try_build(mut self) -> ::std::result::Result<Self::Config, Self::Error> {
        if let ::std::option::Option::Some(override_conf) = self.override_conf {
            <Self::Config as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
                &mut self.inner,
                override_conf,
            )?;
        }
        ::std::result::Result::Ok(self.inner)
    }
}
/// Provides support for configuring [`OptContainsValidationConfig`](struct@OptContainsValidationConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `OptContainsValidationOverrideConfig` is provided for just this case.
///
/// ---
///
/// `OptContainsValidationOverrideConfig` provides run-time support for:
/// - *Setting* [`OptContainsValidationConfig`](struct@OptContainsValidationConfig) fields
/// - *Overriding* compile-time assignments (made using [`OptContainsValidationConfigBuilder`](struct@OptContainsValidationConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     opt_contains_validation: OptContainsValidationOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithValidationOverrideConfig` |
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct OptContainsValidationOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub foo: ::std::option::Option<WithValidationOverrideConfig>,
}
impl OptContainsValidationOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithValidationOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for OptContainsValidationOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
