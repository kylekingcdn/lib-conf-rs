use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct DefContainsRequiredAndOverrideSkipConfig {
    pub(crate) foo: WithRequiredAndOverrideSkipConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl DefContainsRequiredAndOverrideSkipConfig {
    #[doc(hidden)]
    #[allow(dead_code)]
    fn __assertions() {
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredAndOverrideSkipConfig : ::std::clone::Clone, ::std::fmt::Debug
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredAndOverrideSkipConfig : ::std::marker::Copy
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredAndOverrideSkipConfig : ::lib_conf::traits::merge::MergeOverride
            < Override = WithRequiredAndOverrideSkipOverrideConfig >
        );
        ::lib_conf::__private::static_assertions::assert_impl_all!(
            WithRequiredAndOverrideSkipConfigBuilder :
            ::lib_conf::traits::build::BuildConfig < Config =
            WithRequiredAndOverrideSkipConfig >
        );
    }
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub(crate) fn new() -> Self {
        Self {
            foo: WithRequiredAndOverrideSkipConfig::builder(false).build(),
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> DefContainsRequiredAndOverrideSkipConfigBuilder {
        <DefContainsRequiredAndOverrideSkipConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredAndOverrideSkipConfig::builder(false).build()`**
    #[must_use]
    pub fn foo(&self) -> WithRequiredAndOverrideSkipConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: DefContainsRequiredAndOverrideSkipOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsRequiredAndOverrideSkipConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for DefContainsRequiredAndOverrideSkipConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride
for DefContainsRequiredAndOverrideSkipConfig {
    type Override = DefContainsRequiredAndOverrideSkipOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if let ::std::option::Option::Some(val) = val {
                <WithRequiredAndOverrideSkipConfig as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
                    &mut self.foo,
                    val,
                );
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<DefContainsRequiredAndOverrideSkipOverrideConfig>
for DefContainsRequiredAndOverrideSkipConfig {
    fn from(override_conf: DefContainsRequiredAndOverrideSkipOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<DefContainsRequiredAndOverrideSkipOverrideConfig>
for DefContainsRequiredAndOverrideSkipConfig {
    type Output = Self;
    fn add(mut self, rhs: DefContainsRequiredAndOverrideSkipOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<DefContainsRequiredAndOverrideSkipOverrideConfig>
for DefContainsRequiredAndOverrideSkipConfig {
    fn add_assign(&mut self, rhs: DefContainsRequiredAndOverrideSkipOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`DefContainsRequiredAndOverrideSkipConfig`](struct@DefContainsRequiredAndOverrideSkipConfig) at compile-time.
///
/// Can be constructed using either [`DefContainsRequiredAndOverrideSkipConfig::builder()`](method@DefContainsRequiredAndOverrideSkipConfig::builder) or [`DefContainsRequiredAndOverrideSkipConfigBuilder::new()`](method@DefContainsRequiredAndOverrideSkipConfigBuilder::new).
///
/// For run-time configuration of `DefContainsRequiredAndOverrideSkipConfig`, see [`DefContainsRequiredAndOverrideSkipOverrideConfig`](struct@DefContainsRequiredAndOverrideSkipOverrideConfig)
#[derive(Debug, Clone, Copy, Default)]
pub struct DefContainsRequiredAndOverrideSkipConfigBuilder {
    pub(crate) inner: DefContainsRequiredAndOverrideSkipConfig,
    pub(crate) override_conf: ::std::option::Option<
        DefContainsRequiredAndOverrideSkipOverrideConfig,
    >,
}
impl DefContainsRequiredAndOverrideSkipConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <DefContainsRequiredAndOverrideSkipConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredAndOverrideSkipConfig::builder(false).build()`**
    #[must_use]
    pub(crate) fn foo(mut self, val: WithRequiredAndOverrideSkipConfig) -> Self {
        self.inner.foo = val;
        self
    }
    /// Allows for inline configuration of nested [`WithRequiredAndOverrideSkipConfig`](struct@WithRequiredAndOverrideSkipConfig)
    ///
    /// Closure takes no parameters, and returns a compile-time configured [`WithRequiredAndOverrideSkipConfigBuilder`](struct@WithRequiredAndOverrideSkipConfigBuilder).
    ///
    /// The [`WithRequiredAndOverrideSkipConfigBuilder`](struct@WithRequiredAndOverrideSkipConfigBuilder) returned by the closure will automatically be built (and associated [`WithRequiredAndOverrideSkipOverrideConfig`](struct@WithRequiredAndOverrideSkipOverrideConfig) merged).
    #[must_use]
    pub fn foo_builder<F>(mut self, f: F) -> Self
    where
        F: Fn() -> WithRequiredAndOverrideSkipConfigBuilder,
    {
        let builder = f();
        self.inner.foo = <WithRequiredAndOverrideSkipConfigBuilder as ::lib_conf::traits::build::BuildConfig>::build(
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
        override_conf: DefContainsRequiredAndOverrideSkipOverrideConfig,
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
    /// Builds the [`DefContainsRequiredAndOverrideSkipConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> DefContainsRequiredAndOverrideSkipConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig
for DefContainsRequiredAndOverrideSkipConfigBuilder {
    type Config = DefContainsRequiredAndOverrideSkipConfig;
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
/// Provides support for configuring [`DefContainsRequiredAndOverrideSkipConfig`](struct@DefContainsRequiredAndOverrideSkipConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `DefContainsRequiredAndOverrideSkipOverrideConfig` is provided for just this case.
///
/// ---
///
/// `DefContainsRequiredAndOverrideSkipOverrideConfig` provides run-time support for:
/// - *Setting* [`DefContainsRequiredAndOverrideSkipConfig`](struct@DefContainsRequiredAndOverrideSkipConfig) fields
/// - *Overriding* compile-time assignments (made using [`DefContainsRequiredAndOverrideSkipConfigBuilder`](struct@DefContainsRequiredAndOverrideSkipConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     def_contains_required_and_override_skip: DefContainsRequiredAndOverrideSkipOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `WithRequiredAndOverrideSkipOverrideConfig` |
#[derive(Debug, Clone, Copy, ::serde::Deserialize)]
pub struct DefContainsRequiredAndOverrideSkipOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredAndOverrideSkipConfig::builder(false).build()`**
    pub foo: ::std::option::Option<WithRequiredAndOverrideSkipOverrideConfig>,
}
impl DefContainsRequiredAndOverrideSkipOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithRequiredAndOverrideSkipConfig::builder(false).build()`**
    #[must_use]
    pub fn foo(
        &self,
    ) -> ::std::option::Option<WithRequiredAndOverrideSkipOverrideConfig> {
        self.foo
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for DefContainsRequiredAndOverrideSkipOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
