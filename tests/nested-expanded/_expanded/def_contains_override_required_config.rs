use super::*;
///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct DefContainsOverrideRequiredConfig {
    pub(crate) foo: WithOverrideRequiredConfig,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl DefContainsOverrideRequiredConfig {
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
    pub(crate) fn new() -> Self {
        Self {
            foo: WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig {
                    foo: true,
                })
                .build(),
        }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> DefContainsOverrideRequiredConfigBuilder {
        <DefContainsOverrideRequiredConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig {
    ///         foo: true,
    ///     })
    ///     .build()`**
    #[must_use]
    pub fn foo(&self) -> WithOverrideRequiredConfig {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(
        &mut self,
        override_conf: DefContainsOverrideRequiredOverrideConfig,
    ) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsOverrideRequiredConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for DefContainsOverrideRequiredConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for DefContainsOverrideRequiredConfig {
    type Override = DefContainsOverrideRequiredOverrideConfig;
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
impl ::std::convert::From<DefContainsOverrideRequiredOverrideConfig>
for DefContainsOverrideRequiredConfig {
    fn from(override_conf: DefContainsOverrideRequiredOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<DefContainsOverrideRequiredOverrideConfig>
for DefContainsOverrideRequiredConfig {
    type Output = Self;
    fn add(mut self, rhs: DefContainsOverrideRequiredOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<DefContainsOverrideRequiredOverrideConfig>
for DefContainsOverrideRequiredConfig {
    fn add_assign(&mut self, rhs: DefContainsOverrideRequiredOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`DefContainsOverrideRequiredConfig`](struct@DefContainsOverrideRequiredConfig) at compile-time.
///
/// Can be constructed using either [`DefContainsOverrideRequiredConfig::builder()`](method@DefContainsOverrideRequiredConfig::builder) or [`DefContainsOverrideRequiredConfigBuilder::new()`](method@DefContainsOverrideRequiredConfigBuilder::new).
///
/// For run-time configuration of `DefContainsOverrideRequiredConfig`, see [`DefContainsOverrideRequiredOverrideConfig`](struct@DefContainsOverrideRequiredOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct DefContainsOverrideRequiredConfigBuilder {
    pub(crate) inner: DefContainsOverrideRequiredConfig,
    pub(crate) override_conf: ::std::option::Option<
        DefContainsOverrideRequiredOverrideConfig,
    >,
}
impl DefContainsOverrideRequiredConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <DefContainsOverrideRequiredConfig>::new(),
        }
    }
    ///
    /// ---
    ///
    /// - Library default: **`WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig {
    ///         foo: true,
    ///     })
    ///     .build()`**
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
        override_conf: DefContainsOverrideRequiredOverrideConfig,
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
    /// Builds the [`DefContainsOverrideRequiredConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> DefContainsOverrideRequiredConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for DefContainsOverrideRequiredConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig
for DefContainsOverrideRequiredConfigBuilder {
    type Config = DefContainsOverrideRequiredConfig;
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
/// Provides support for configuring [`DefContainsOverrideRequiredConfig`](struct@DefContainsOverrideRequiredConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `DefContainsOverrideRequiredOverrideConfig` is provided for just this case.
///
/// ---
///
/// `DefContainsOverrideRequiredOverrideConfig` provides run-time support for:
/// - *Setting* [`DefContainsOverrideRequiredConfig`](struct@DefContainsOverrideRequiredConfig) fields
/// - *Overriding* compile-time assignments (made using [`DefContainsOverrideRequiredConfigBuilder`](struct@DefContainsOverrideRequiredConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     def_contains_override_required: DefContainsOverrideRequiredOverrideConfig,
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
pub struct DefContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig {
    ///         foo: true,
    ///     })
    ///     .build()`**
    pub foo: ::std::option::Option<WithOverrideRequiredOverrideConfig>,
}
impl DefContainsOverrideRequiredOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`WithOverrideRequiredConfig::builder(WithOverrideRequiredOverrideConfig {
    ///         foo: true,
    ///     })
    ///     .build()`**
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
impl ::std::default::Default for DefContainsOverrideRequiredOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
        }
    }
}
