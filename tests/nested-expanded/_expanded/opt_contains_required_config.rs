use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct OptContainsRequiredConfig {
    pub(crate) foo: Option<WithRequiredConfig>,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl OptContainsRequiredConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithRequiredConfig > : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            Option < WithRequiredConfig > : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredConfig : ::lib_conf::traits::merge::MergeOverride < Override =
            WithRequiredOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredConfigBuilder : ::lib_conf::traits::build::BuildConfig < Config =
            WithRequiredConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredConfig : ::std::convert::TryFrom < WithRequiredOverrideConfig,
            Error = WithRequiredConfigError >
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
    pub fn builder() -> OptContainsRequiredConfigBuilder {
        <OptContainsRequiredConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> Option<WithRequiredConfig> {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn try_merge_with_override(
        &mut self,
        override_conf: OptContainsRequiredOverrideConfig,
    ) -> ::std::result::Result<(), OptContainsRequiredConfigError> {
        <Self as ::lib_conf::traits::merge::TryMergeOverride>::try_merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsRequiredConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for OptContainsRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::TryMergeOverride for OptContainsRequiredConfig {
    type Override = OptContainsRequiredOverrideConfig;
    type Error = OptContainsRequiredConfigError;
    fn try_merge_with_override(
        &mut self,
        override_conf: Self::Override,
    ) -> ::std::result::Result<(), Self::Error> {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                if let ::std::option::Option::Some(current) = &mut self.foo {
                    <WithRequiredConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                        current,
                        val,
                    );
                } else {
                    let mut inited = <WithRequiredConfig>::try_from(val)
                        .map_err(|error| OptContainsRequiredConfigError::FooField {
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
impl ::std::convert::TryFrom<OptContainsRequiredOverrideConfig>
for OptContainsRequiredConfig {
    type Error = OptContainsRequiredConfigError;
    fn try_from(
        override_conf: OptContainsRequiredOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    ::std::option::Option::Some(
                        <WithRequiredConfig>::try_from(val)
                            .map_err(|error| OptContainsRequiredConfigError::FooField {
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
/// Possible errors encountered during build of [`OptContainsRequiredConfig`](struct@OptContainsRequiredConfig)
#[derive(::std::fmt::Debug)]
pub enum OptContainsRequiredConfigError {
    /// Failed to merge nested [`WithRequiredConfig`](struct@WithRequiredConfig)
    FooField {
        /// Corresponds to the error type returned by [`WithRequiredConfig::try_merge()`](method@WithRequiredConfig::try_merge)
        error: WithRequiredConfigError,
    },
}
impl OptContainsRequiredConfigError {
    #[doc(hidden)]
    fn __error_field_list(names: &[::std::string::String]) -> ::std::string::String {
        names
            .iter()
            .map(|name| format!("- `{name}`"))
            .collect::<::std::vec::Vec<_>>()
            .join("\n")
    }
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
impl ::std::fmt::Display for OptContainsRequiredConfigError {
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
impl ::std::error::Error for OptContainsRequiredConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for OptContainsRequiredConfigError {
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
/// Provides a builder used to configure [`OptContainsRequiredConfig`](struct@OptContainsRequiredConfig) at compile-time.
///
/// Can be constructed using either [`OptContainsRequiredConfig::builder()`](method@OptContainsRequiredConfig::builder) or [`OptContainsRequiredConfigBuilder::new()`](method@OptContainsRequiredConfigBuilder::new).
///
/// For run-time configuration of `OptContainsRequiredConfig`, see [`OptContainsRequiredOverrideConfig`](struct@OptContainsRequiredOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct OptContainsRequiredConfigBuilder {
    pub(crate) inner: OptContainsRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<OptContainsRequiredOverrideConfig>,
}
impl OptContainsRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <OptContainsRequiredConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub(crate) fn foo(mut self, val: Option<WithRequiredConfig>) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithRequiredConfig`](struct@WithRequiredConfig)
    ///
    /// Closure takes no parameters, and returns a compile-time configured [`WithRequiredConfigBuilder`](struct@WithRequiredConfigBuilder).
    ///
    /// The [`WithRequiredConfigBuilder`](struct@WithRequiredConfigBuilder) returned by the closure will automatically be built (and associated [`WithRequiredOverrideConfig`](struct@WithRequiredOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithRequiredConfigBuilder,
    {
        let builder = f();
        self.inner.foo = ::std::option::Option::Some(
            <WithRequiredConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
                builder,
            ),
        );
        self
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
        override_conf: OptContainsRequiredOverrideConfig,
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
    /// Builds the [`OptContainsRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn try_build(
        mut self,
    ) -> ::std::result::Result<
        OptContainsRequiredConfig,
        OptContainsRequiredConfigError,
    > {
        <Self as ::lib_conf::traits::build::TryBuildConfig>::try_build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for OptContainsRequiredConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::TryBuildConfig for OptContainsRequiredConfigBuilder {
    type Config = OptContainsRequiredConfig;
    type Error = OptContainsRequiredConfigError;
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
/// Provides support for configuring [`OptContainsRequiredConfig`](struct@OptContainsRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `OptContainsRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `OptContainsRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`OptContainsRequiredConfig`](struct@OptContainsRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`OptContainsRequiredConfigBuilder`](struct@OptContainsRequiredConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     opt_contains_required: OptContainsRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithRequiredOverrideConfig` |
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct OptContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    pub foo: ::std::option::Option<WithRequiredOverrideConfig>,
}
impl OptContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`None`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithRequiredOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for OptContainsRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
