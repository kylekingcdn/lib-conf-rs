use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct WithRequiredAndOverrideSkipConfig {
    pub(crate) foo: bool,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl WithRequiredAndOverrideSkipConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            bool : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            bool : ::std::marker::Copy
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new(foo: bool) -> Self {
        Self { foo: foo }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(foo: bool) -> WithRequiredAndOverrideSkipConfigBuilder {
        <WithRequiredAndOverrideSkipConfigBuilder>::new(foo)
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> bool {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: WithRequiredAndOverrideSkipOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for WithRequiredAndOverrideSkipConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for WithRequiredAndOverrideSkipConfig {
    type Override = WithRequiredAndOverrideSkipOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {}
}
#[automatically_derived]
impl ::std::ops::Add<WithRequiredAndOverrideSkipOverrideConfig>
for WithRequiredAndOverrideSkipConfig {
    type Output = Self;
    fn add(mut self, rhs: WithRequiredAndOverrideSkipOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<WithRequiredAndOverrideSkipOverrideConfig>
for WithRequiredAndOverrideSkipConfig {
    fn add_assign(&mut self, rhs: WithRequiredAndOverrideSkipOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`WithRequiredAndOverrideSkipConfig`](struct@WithRequiredAndOverrideSkipConfig) at compile-time.
///
/// Can be constructed using either [`WithRequiredAndOverrideSkipConfig::builder()`](method@WithRequiredAndOverrideSkipConfig::builder) or [`WithRequiredAndOverrideSkipConfigBuilder::new()`](method@WithRequiredAndOverrideSkipConfigBuilder::new).
///
/// For run-time configuration of `WithRequiredAndOverrideSkipConfig`, see [`WithRequiredAndOverrideSkipOverrideConfig`](struct@WithRequiredAndOverrideSkipOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct WithRequiredAndOverrideSkipConfigBuilder {
    pub(crate) inner: WithRequiredAndOverrideSkipConfig,
    pub(crate) override_conf: ::std::option::Option<
        WithRequiredAndOverrideSkipOverrideConfig,
    >,
}
impl WithRequiredAndOverrideSkipConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(foo: bool) -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <WithRequiredAndOverrideSkipConfig>::new(foo),
        }
    }
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(mut self, val: bool) -> Self {
        self.inner.foo = val;
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
        override_conf: WithRequiredAndOverrideSkipOverrideConfig,
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
    /// Builds the [`WithRequiredAndOverrideSkipConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> WithRequiredAndOverrideSkipConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig
for WithRequiredAndOverrideSkipConfigBuilder {
    type Config = WithRequiredAndOverrideSkipConfig;
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
/// Provides support for configuring [`WithRequiredAndOverrideSkipConfig`](struct@WithRequiredAndOverrideSkipConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `WithRequiredAndOverrideSkipOverrideConfig` is provided for just this case.
///
/// ---
///
/// `WithRequiredAndOverrideSkipOverrideConfig` provides run-time support for:
/// - *Setting* [`WithRequiredAndOverrideSkipConfig`](struct@WithRequiredAndOverrideSkipConfig) fields
/// - *Overriding* compile-time assignments (made using [`WithRequiredAndOverrideSkipConfigBuilder`](struct@WithRequiredAndOverrideSkipConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     with_required_and_override_skip: WithRequiredAndOverrideSkipOverrideConfig,
/// }
/// ```
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct WithRequiredAndOverrideSkipOverrideConfig {}
impl WithRequiredAndOverrideSkipOverrideConfig {}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for WithRequiredAndOverrideSkipOverrideConfig {
    fn default() -> Self {
        Self {}
    }
}
