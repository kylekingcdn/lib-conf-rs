use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct DefContainsDefaultConfig {
    pub(crate) foo: WithDefaultConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl DefContainsDefaultConfig {
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
    pub(crate) fn new() -> Self {
        Self {
            foo: WithDefaultConfig::builder().foo(true).build(),
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> DefContainsDefaultConfigBuilder {
        <DefContainsDefaultConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithDefaultConfig::builder().foo(true).build()`**
    #[must_use]
    pub fn foo(&self) -> WithDefaultConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: DefContainsDefaultOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for DefContainsDefaultConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for DefContainsDefaultConfig {
    type Override = DefContainsDefaultOverrideConfig;
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
impl ::std::convert::From<DefContainsDefaultOverrideConfig>
for DefContainsDefaultConfig {
    fn from(override_conf: DefContainsDefaultOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<DefContainsDefaultOverrideConfig> for DefContainsDefaultConfig {
    type Output = Self;
    fn add(mut self, rhs: DefContainsDefaultOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<DefContainsDefaultOverrideConfig>
for DefContainsDefaultConfig {
    fn add_assign(&mut self, rhs: DefContainsDefaultOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`DefContainsDefaultConfig`](struct@DefContainsDefaultConfig) at compile-time.
///
/// Can be constructed using either [`DefContainsDefaultConfig::builder()`](method@DefContainsDefaultConfig::builder) or [`DefContainsDefaultConfigBuilder::new()`](method@DefContainsDefaultConfigBuilder::new).
///
/// For run-time configuration of `DefContainsDefaultConfig`, see [`DefContainsDefaultOverrideConfig`](struct@DefContainsDefaultOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct DefContainsDefaultConfigBuilder {
    pub(crate) inner: DefContainsDefaultConfig,
    pub(crate) override_conf: ::std::option::Option<DefContainsDefaultOverrideConfig>,
}
impl DefContainsDefaultConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <DefContainsDefaultConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithDefaultConfig::builder().foo(true).build()`**
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
        override_conf: DefContainsDefaultOverrideConfig,
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
    /// Builds the [`DefContainsDefaultConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> DefContainsDefaultConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsDefaultConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for DefContainsDefaultConfigBuilder {
    type Config = DefContainsDefaultConfig;
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
/// Provides support for configuring [`DefContainsDefaultConfig`](struct@DefContainsDefaultConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `DefContainsDefaultOverrideConfig` is provided for just this case.
///
/// ---
///
/// `DefContainsDefaultOverrideConfig` provides run-time support for:
/// - *Setting* [`DefContainsDefaultConfig`](struct@DefContainsDefaultConfig) fields
/// - *Overriding* compile-time assignments (made using [`DefContainsDefaultConfigBuilder`](struct@DefContainsDefaultConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     def_contains_default: DefContainsDefaultOverrideConfig,
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
pub struct DefContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithDefaultConfig::builder().foo(true).build()`**
    pub foo: ::std::option::Option<WithDefaultOverrideConfig>,
}
impl DefContainsDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithDefaultConfig::builder().foo(true).build()`**
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
impl ::std::default::Default for DefContainsDefaultOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
