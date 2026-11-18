use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct DefContainsRequiredConfig {
    pub(crate) foo: WithRequiredConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl DefContainsRequiredConfig {
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
    pub(crate) fn new() -> Self {
        Self {
            foo: WithRequiredConfig::builder(true).build(),
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> DefContainsRequiredConfigBuilder {
        <DefContainsRequiredConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredConfig::builder(true).build()`**
    #[must_use]
    pub fn foo(&self) -> WithRequiredConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: DefContainsRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsRequiredConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for DefContainsRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for DefContainsRequiredConfig {
    type Override = DefContainsRequiredOverrideConfig;
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
impl ::std::convert::From<DefContainsRequiredOverrideConfig>
for DefContainsRequiredConfig {
    fn from(override_conf: DefContainsRequiredOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<DefContainsRequiredOverrideConfig> for DefContainsRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: DefContainsRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<DefContainsRequiredOverrideConfig>
for DefContainsRequiredConfig {
    fn add_assign(&mut self, rhs: DefContainsRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`DefContainsRequiredConfig`](struct@DefContainsRequiredConfig) at compile-time.
///
/// Can be constructed using either [`DefContainsRequiredConfig::builder()`](method@DefContainsRequiredConfig::builder) or [`DefContainsRequiredConfigBuilder::new()`](method@DefContainsRequiredConfigBuilder::new).
///
/// For run-time configuration of `DefContainsRequiredConfig`, see [`DefContainsRequiredOverrideConfig`](struct@DefContainsRequiredOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct DefContainsRequiredConfigBuilder {
    pub(crate) inner: DefContainsRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<DefContainsRequiredOverrideConfig>,
}
impl DefContainsRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <DefContainsRequiredConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredConfig::builder(true).build()`**
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
        override_conf: DefContainsRequiredOverrideConfig,
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
    /// Builds the [`DefContainsRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> DefContainsRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsRequiredConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for DefContainsRequiredConfigBuilder {
    type Config = DefContainsRequiredConfig;
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
/// Provides support for configuring [`DefContainsRequiredConfig`](struct@DefContainsRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `DefContainsRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `DefContainsRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`DefContainsRequiredConfig`](struct@DefContainsRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`DefContainsRequiredConfigBuilder`](struct@DefContainsRequiredConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     def_contains_required: DefContainsRequiredOverrideConfig,
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
pub struct DefContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredConfig::builder(true).build()`**
    pub foo: ::std::option::Option<WithRequiredOverrideConfig>,
}
impl DefContainsRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredConfig::builder(true).build()`**
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
impl ::std::default::Default for DefContainsRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
