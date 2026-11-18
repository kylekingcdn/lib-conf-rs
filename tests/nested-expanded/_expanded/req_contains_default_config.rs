use super::*;
///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct ReqContainsDefaultConfig {
    pub(crate) foo: WithDefaultConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl ReqContainsDefaultConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfigBuilder : ::std::default::Default
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::lib_conf::traits::merge::MergeOverride < Override =
            WithDefaultOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfigBuilder : ::lib_conf::traits::build::BuildConfig < Config =
            WithDefaultConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithDefaultConfig : ::std::convert::From < WithDefaultOverrideConfig >
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new(foo: WithDefaultConfig) -> Self {
        Self { foo: foo }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(foo: WithDefaultConfig) -> ReqContainsDefaultConfigBuilder {
        <ReqContainsDefaultConfigBuilder>::new(foo)
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> WithDefaultConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: ReqContainsDefaultOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for ReqContainsDefaultConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for ReqContainsDefaultConfig {
    type Override = ReqContainsDefaultOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                <WithDefaultConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                    &mut self.foo,
                    val,
                );
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::TryFrom<ReqContainsDefaultOverrideConfig>
for ReqContainsDefaultConfig {
    type Error = ReqContainsDefaultConfigError;
    fn try_from(
        override_conf: ReqContainsDefaultOverrideConfig,
    ) -> ::std::result::Result<Self, Self::Error> {
        let out = Self {
            foo: {
                let val = override_conf.foo;
                if let ::std::option::Option::Some(val) = val {
                    <WithDefaultConfig>::from(val)
                } else {
                    ::std::result::Result::Err(ReqContainsDefaultConfigError::MissingRequiredFields {
                        field_names: vec!["foo".to_string()],
                    })?
                }
            },
        };
        ::std::result::Result::Ok(out)
    }
}
#[automatically_derived]
impl ::std::ops::Add<ReqContainsDefaultOverrideConfig> for ReqContainsDefaultConfig {
    type Output = Self;
    fn add(mut self, rhs: ReqContainsDefaultOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<ReqContainsDefaultOverrideConfig>
for ReqContainsDefaultConfig {
    fn add_assign(&mut self, rhs: ReqContainsDefaultOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Possible errors encountered during builf of [`ReqContainsDefaultConfig`](struct@ReqContainsDefaultConfig)
#[derive(::std::fmt::Debug)]
pub enum ReqContainsDefaultConfigError {
    /// Unable to initialize struct due to missing required fields
    MissingRequiredFields {
        /// Names of missing fields
        field_names: Vec<String>,
    },
}
impl ReqContainsDefaultConfigError {
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
impl ::std::fmt::Display for ReqContainsDefaultConfigError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let text = match self {
            Self::MissingRequiredFields { field_names } => {
                format!(
                    "Missing required `ReqContainsDefaultConfig` fields:\n{}",
                    ::lib_conf::__private::error_field_list(field_names)
                )
            }
        };
        write!(f, "{text}")?;
        ::std::result::Result::Ok(())
    }
}
#[automatically_derived]
impl ::std::error::Error for ReqContainsDefaultConfigError {}
#[automatically_derived]
impl ::lib_conf::traits::error::ConfigError for ReqContainsDefaultConfigError {
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
/// Provides a builder used to configure [`ReqContainsDefaultConfig`](struct@ReqContainsDefaultConfig) at compile-time.
///
/// Can be constructed using either [`ReqContainsDefaultConfig::builder()`](method@ReqContainsDefaultConfig::builder) or [`ReqContainsDefaultConfigBuilder::new()`](method@ReqContainsDefaultConfigBuilder::new).
///
/// For run-time configuration of `ReqContainsDefaultConfig`, see [`ReqContainsDefaultOverrideConfig`](struct@ReqContainsDefaultOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct ReqContainsDefaultConfigBuilder {
    pub(crate) inner: ReqContainsDefaultConfig,
    pub(crate) override_conf: ::std::option::Option<ReqContainsDefaultOverrideConfig>,
}
impl ReqContainsDefaultConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(foo: WithDefaultConfig) -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <ReqContainsDefaultConfig>::new(foo),
        }
    }
    ///
    /// ---
    ///
    #[must_use]
    pub(crate) fn foo(mut self, val: WithDefaultConfig) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithDefaultConfig`](struct@WithDefaultConfig)
    ///
    /// Closure has a single [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) returned by the closure will automatically be built (and associated [`WithDefaultOverrideConfig`](struct@WithDefaultOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithDefaultConfigBuilder,
    {
        let builder = f();
        self.inner.foo = <WithDefaultConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
            builder,
        );
        self
    }
    /// Allows for inline configuration of nested [`WithDefaultConfig`](struct@WithDefaultConfig)
    ///
    /// Closure has a single [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) parameter and returns the builder after configured.
    ///
    /// The [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder) returned by the closure will automatically be built (and associated [`WithDefaultOverrideConfig`](struct@WithDefaultOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder_default<F>(mut self, f: F) -> Self
    where
        F: Fn(WithDefaultConfigBuilder) -> WithDefaultConfigBuilder,
    {
        let builder = f(<WithDefaultConfigBuilder>::default());
        self.inner.foo = <WithDefaultConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
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
        override_conf: ReqContainsDefaultOverrideConfig,
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
    /// Builds the [`ReqContainsDefaultConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> ReqContainsDefaultConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for ReqContainsDefaultConfigBuilder {
    type Config = ReqContainsDefaultConfig;
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
/// Provides support for configuring [`ReqContainsDefaultConfig`](struct@ReqContainsDefaultConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `ReqContainsDefaultOverrideConfig` is provided for just this case.
///
/// ---
///
/// `ReqContainsDefaultOverrideConfig` provides run-time support for:
/// - *Setting* [`ReqContainsDefaultConfig`](struct@ReqContainsDefaultConfig) fields
/// - *Overriding* compile-time assignments (made using [`ReqContainsDefaultConfigBuilder`](struct@ReqContainsDefaultConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     req_contains_default: ReqContainsDefaultOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithDefaultOverrideConfig` |
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct ReqContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    pub foo: ::std::option::Option<WithDefaultOverrideConfig>,
}
impl ReqContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<WithDefaultOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for ReqContainsDefaultOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
