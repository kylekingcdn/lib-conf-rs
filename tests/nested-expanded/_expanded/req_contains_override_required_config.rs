use super::*;
///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct ReqContainsOverrideRequiredConfig {
    pub(crate) foo: WithOverrideRequiredConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl ReqContainsOverrideRequiredConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::lib_conf::traits::merge::MergeOverride <
            Override = WithOverrideRequiredOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfigBuilder : ::lib_conf::traits::build::BuildConfig <
            Config = WithOverrideRequiredConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithOverrideRequiredConfig : ::std::convert::From <
            WithOverrideRequiredOverrideConfig >
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new(foo: WithOverrideRequiredConfig) -> Self {
        Self { foo: foo }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(
        foo: WithOverrideRequiredConfig,
    ) -> ReqContainsOverrideRequiredConfigBuilder {
        <ReqContainsOverrideRequiredConfigBuilder>::new(foo)
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> WithOverrideRequiredConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: ReqContainsOverrideRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for ReqContainsOverrideRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for ReqContainsOverrideRequiredConfig {
    type Override = ReqContainsOverrideRequiredOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                <WithOverrideRequiredConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                    &mut self.foo,
                    val,
                );
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<ReqContainsOverrideRequiredOverrideConfig>
for ReqContainsOverrideRequiredConfig {
    type Error = ReqContainsOverrideRequiredConfigError;
    fn try_from(
        override_conf: ReqContainsOverrideRequiredOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    <WithOverrideRequiredConfig>::from(val)
                } else {
                    ::std::result::Result::Err(ReqContainsOverrideRequiredConfigError::MissingRequiredFields {
                        field_names: vec!["foo".to_string()],
                    })?
                }
            },
        };
        ::std::result::Result::Ok(out)
    }
}
#[automatically_derived]
impl ::std::ops::Add<ReqContainsOverrideRequiredOverrideConfig>
for ReqContainsOverrideRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: ReqContainsOverrideRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<ReqContainsOverrideRequiredOverrideConfig>
for ReqContainsOverrideRequiredConfig {
    fn add_assign(&mut self, rhs: ReqContainsOverrideRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Possible errors encountered during builf of [`ReqContainsOverrideRequiredConfig`](struct@ReqContainsOverrideRequiredConfig)
#[derive(::std::fmt::Debug)]
pub enum ReqContainsOverrideRequiredConfigError {
    /// Unable to initialize struct due to missing required fields
    MissingRequiredFields {
        /// Names of missing fields
        field_names: Vec<String>,
    },
}
impl ReqContainsOverrideRequiredConfigError {
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
impl ::std::fmt::Display for ReqContainsOverrideRequiredConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::MissingRequiredFields { field_names } => {
                format!(
                    "Missing required `ReqContainsOverrideRequiredConfig` fields:\n{}",
                    ::lib_conf::__private::error_field_list(field_names)
                )
            }
        };
        write!(f, "{text}")?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::error::Error for ReqContainsOverrideRequiredConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for ReqContainsOverrideRequiredConfigError {
    fn build_field_chain(
        &self,
        chain: &mut ::std::vec::Vec<::std::string::String>,
        self_field_name: ::std::option::Option<&str>,
    ) {
        if let ::std::option::Option::Some(name) = self_field_name {
            chain.push(name.to_string());
        }
        match self {
            _ => {}
        };
    }
    fn origin_message(&self) -> ::std::string::String {
        match self {
            _ => self.to_string(),
        }
    }
}
/// Provides a builder used to configure [`ReqContainsOverrideRequiredConfig`](struct@ReqContainsOverrideRequiredConfig) at compile-time.
///
/// Can be constructed using either [`ReqContainsOverrideRequiredConfig::builder()`](method@ReqContainsOverrideRequiredConfig::builder) or [`ReqContainsOverrideRequiredConfigBuilder::new()`](method@ReqContainsOverrideRequiredConfigBuilder::new).
///
/// For run-time configuration of `ReqContainsOverrideRequiredConfig`, see [`ReqContainsOverrideRequiredOverrideConfig`](struct@ReqContainsOverrideRequiredOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct ReqContainsOverrideRequiredConfigBuilder {
    pub(crate) inner: ReqContainsOverrideRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<
        ReqContainsOverrideRequiredOverrideConfig,
    >,
}
impl ReqContainsOverrideRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(foo: WithOverrideRequiredConfig) -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <ReqContainsOverrideRequiredConfig>::new(foo),
        }
    }
    ///
    /// ---
    ///
    #[must_use]
    pub(crate) fn foo(mut self, val: WithOverrideRequiredConfig) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithOverrideRequiredConfig`](struct@WithOverrideRequiredConfig)
    ///
    /// Closure takes no parameters, and returns a compile-time configured [`WithOverrideRequiredConfigBuilder`](struct@WithOverrideRequiredConfigBuilder).
    ///
    /// The [`WithOverrideRequiredConfigBuilder`](struct@WithOverrideRequiredConfigBuilder) returned by the closure will automatically be built (and associated [`WithOverrideRequiredOverrideConfig`](struct@WithOverrideRequiredOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithOverrideRequiredConfigBuilder,
    {
        let builder = f();
        self.inner.foo = <WithOverrideRequiredConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
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
        override_conf: ReqContainsOverrideRequiredOverrideConfig,
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
    /// Builds the [`ReqContainsOverrideRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> ReqContainsOverrideRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig
for ReqContainsOverrideRequiredConfigBuilder {
    type Config = ReqContainsOverrideRequiredConfig;
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
/// Provides support for configuring [`ReqContainsOverrideRequiredConfig`](struct@ReqContainsOverrideRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `ReqContainsOverrideRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `ReqContainsOverrideRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`ReqContainsOverrideRequiredConfig`](struct@ReqContainsOverrideRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`ReqContainsOverrideRequiredConfigBuilder`](struct@ReqContainsOverrideRequiredConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     req_contains_override_required: ReqContainsOverrideRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithOverrideRequiredOverrideConfig` |
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct ReqContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    pub foo: ::std::option::Option<WithOverrideRequiredOverrideConfig>,
}
impl ReqContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithOverrideRequiredOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for ReqContainsOverrideRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
