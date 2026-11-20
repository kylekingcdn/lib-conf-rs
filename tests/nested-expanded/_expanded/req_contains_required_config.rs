use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct ReqContainsRequiredConfig {
    pub(crate) foo: WithRequiredConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl ReqContainsRequiredConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredConfig : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredConfig : ::std::marker::Copy
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
    pub(crate) fn new(foo: WithRequiredConfig) -> Self {
        Self { foo: foo }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(foo: WithRequiredConfig) -> ReqContainsRequiredConfigBuilder {
        <ReqContainsRequiredConfigBuilder>::new(foo)
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> WithRequiredConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: ReqContainsRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for ReqContainsRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for ReqContainsRequiredConfig {
    type Override = ReqContainsRequiredOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                <WithRequiredConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                    &mut self.foo,
                    val,
                );
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<ReqContainsRequiredOverrideConfig>
for ReqContainsRequiredConfig {
    type Error = ReqContainsRequiredConfigError;
    fn try_from(
        override_conf: ReqContainsRequiredOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    <WithRequiredConfig>::try_from(val)
                        .map_err(|error| ReqContainsRequiredConfigError::FooField {
                            error,
                        })?
                } else {
                    ::std::result::Result::Err(ReqContainsRequiredConfigError::MissingRequiredFields {
                        field_names: vec!["foo".to_string()],
                    })?
                }
            },
        };
        ::std::result::Result::Ok(out)
    }
}
#[automatically_derived]
impl ::std::ops::Add<ReqContainsRequiredOverrideConfig> for ReqContainsRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: ReqContainsRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<ReqContainsRequiredOverrideConfig>
for ReqContainsRequiredConfig {
    fn add_assign(&mut self, rhs: ReqContainsRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Possible errors encountered during build of [`ReqContainsRequiredConfig`](struct@ReqContainsRequiredConfig)
#[derive(::std::fmt::Debug)]
pub enum ReqContainsRequiredConfigError {
    /// Unable to initialize struct due to missing required fields
    MissingRequiredFields {
        /// Names of missing fields
        field_names: Vec<String>,
    },
    /// Failed to merge nested [`WithRequiredConfig`](struct@WithRequiredConfig)
    FooField {
        /// Corresponds to the error type returned by [`WithRequiredConfig::try_merge()`](method@WithRequiredConfig::try_merge)
        error: WithRequiredConfigError,
    },
}
impl ReqContainsRequiredConfigError {
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
impl ::std::fmt::Display for ReqContainsRequiredConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::MissingRequiredFields { field_names } => {
                format!(
                    "Missing required `ReqContainsRequiredConfig` fields:\n{}",
                    Self::__error_field_list(field_names)
                )
            }
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
impl ::std::error::Error for ReqContainsRequiredConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for ReqContainsRequiredConfigError {
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
/// Provides a builder used to configure [`ReqContainsRequiredConfig`](struct@ReqContainsRequiredConfig) at compile-time.
///
/// Can be constructed using either [`ReqContainsRequiredConfig::builder()`](method@ReqContainsRequiredConfig::builder) or [`ReqContainsRequiredConfigBuilder::new()`](method@ReqContainsRequiredConfigBuilder::new).
///
/// For run-time configuration of `ReqContainsRequiredConfig`, see [`ReqContainsRequiredOverrideConfig`](struct@ReqContainsRequiredOverrideConfig)
#[derive(Debug, Clone, Copy)]
pub struct ReqContainsRequiredConfigBuilder {
    pub(crate) inner: ReqContainsRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<ReqContainsRequiredOverrideConfig>,
}
impl ReqContainsRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(foo: WithRequiredConfig) -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <ReqContainsRequiredConfig>::new(foo),
        }
    }
    ///
    /// ---
    ///
    #[must_use]
    pub(crate) fn foo(mut self, val: WithRequiredConfig) -> Self {
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
        self.inner.foo = <WithRequiredConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
            builder,
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
        override_conf: ReqContainsRequiredOverrideConfig,
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
    /// Builds the [`ReqContainsRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> ReqContainsRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for ReqContainsRequiredConfigBuilder {
    type Config = ReqContainsRequiredConfig;
    fn build(mut self) -> Self::Config {
        if let ::std::option::Option::Some(override_conf) = self.override_conf {
            <Self::Config as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                &mut self.inner,
                override_conf,
            );
        }
        self.inner
    }
}
/// Provides support for configuring [`ReqContainsRequiredConfig`](struct@ReqContainsRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `ReqContainsRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `ReqContainsRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`ReqContainsRequiredConfig`](struct@ReqContainsRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`ReqContainsRequiredConfigBuilder`](struct@ReqContainsRequiredConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     req_contains_required: ReqContainsRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithRequiredOverrideConfig` |
#[derive(Debug, Clone, Copy, ::serde::Deserialize)]
pub struct ReqContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
    pub foo: ::std::option::Option<WithRequiredOverrideConfig>,
}
impl ReqContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
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
impl ::std::default::Default for ReqContainsRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
