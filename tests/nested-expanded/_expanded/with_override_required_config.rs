use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct WithOverrideRequiredConfig {
    pub(crate) foo: bool,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl WithOverrideRequiredConfig {
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
    pub(crate) fn new(override_conf: WithOverrideRequiredOverrideConfig) -> Self {
        Self {
            foo: override_conf.foo.clone(),
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder(
        override_conf: WithOverrideRequiredOverrideConfig,
    ) -> WithOverrideRequiredConfigBuilder {
        <WithOverrideRequiredConfigBuilder>::new(override_conf)
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
        override_conf: WithOverrideRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::marker::Copy for WithOverrideRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for WithOverrideRequiredConfig {
    type Override = WithOverrideRequiredOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            {
                self.foo = val;
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<WithOverrideRequiredOverrideConfig>
for WithOverrideRequiredConfig {
    fn from(override_conf: WithOverrideRequiredOverrideConfig) -> Self {
        Self {
            foo: {
                let val = override_conf.foo;
                { val }
            },
        }
    }
}
#[automatically_derived]
impl ::std::ops::Add<WithOverrideRequiredOverrideConfig> for WithOverrideRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: WithOverrideRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<WithOverrideRequiredOverrideConfig>
for WithOverrideRequiredConfig {
    fn add_assign(&mut self, rhs: WithOverrideRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`WithOverrideRequiredConfig`](struct@WithOverrideRequiredConfig) at compile-time.
///
/// Can be constructed using either [`WithOverrideRequiredConfig::builder()`](method@WithOverrideRequiredConfig::builder) or [`WithOverrideRequiredConfigBuilder::new()`](method@WithOverrideRequiredConfigBuilder::new).
///
/// For run-time configuration of `WithOverrideRequiredConfig`, see [`WithOverrideRequiredOverrideConfig`](struct@WithOverrideRequiredOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct WithOverrideRequiredConfigBuilder {
    pub(crate) inner: WithOverrideRequiredConfig,
    pub(crate) override_conf: WithOverrideRequiredOverrideConfig,
}
impl WithOverrideRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new(override_conf: WithOverrideRequiredOverrideConfig) -> Self {
        Self {
            override_conf: override_conf.clone(),
            inner: <WithOverrideRequiredConfig>::new(override_conf),
        }
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
        override_conf: WithOverrideRequiredOverrideConfig,
    ) -> Self {
        self.override_conf = override_conf;
        self
    }
    /// Builds the [`WithOverrideRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> WithOverrideRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for WithOverrideRequiredConfigBuilder {
    type Config = WithOverrideRequiredConfig;
    fn build(mut self) -> Self::Config {
        let override_conf = self.override_conf;
        <Self::Config as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            &mut self.inner,
            override_conf,
        );
        self.inner
    }
}
/// Provides support for configuring [`WithOverrideRequiredConfig`](struct@WithOverrideRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `WithOverrideRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `WithOverrideRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`WithOverrideRequiredConfig`](struct@WithOverrideRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`WithOverrideRequiredConfigBuilder`](struct@WithOverrideRequiredConfigBuilder))
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     with_override_required: WithOverrideRequiredOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type | Notes |
/// | - | - | - |
/// | `foo` | `bool` | <span align="center">❗️</span> |
///
/// **Notes:**
/// - ❗️: This field is mandatory.
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct WithOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    pub foo: bool,
}
impl WithOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    #[must_use]
    pub fn foo(&self) -> bool {
        self.foo
    }
}
