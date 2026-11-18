use super::*;

///
/// -------------------------------------
/// LibConf input (config attr cleaned):
/// -------------------------------------
///
#[derive(Debug, Clone)]
pub struct WithDefaultConfig {
    pub(crate) foo: bool,
}
///
/// ----------------
/// LibConf output:
/// ----------------
///
impl WithDefaultConfig {
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
    pub(crate) fn new() -> Self {
        Self { foo: false }
    }
    /// Creates a new config builder
    #[must_use]
    pub fn builder() -> WithDefaultConfigBuilder {
        <WithDefaultConfigBuilder>::new()
    }
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    #[must_use]
    pub fn foo(&self) -> bool {
        self.foo
    }
    /// Applies all values from override (which are `Some(_)`) to self
    pub fn merge_with_override(&mut self, override_conf: WithDefaultOverrideConfig) {
        <Self as ::lib_conf::traits::merge::MergeOverride>::merge_with_override(
            self,
            override_conf,
        )
    }
}
#[automatically_derived]
impl ::std::default::Default for WithDefaultConfig {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::std::marker::Copy for WithDefaultConfig {}
#[automatically_derived]
impl ::lib_conf::traits::merge::MergeOverride for WithDefaultConfig {
    type Override = WithDefaultOverrideConfig;
    fn merge_with_override(&mut self, override_conf: Self::Override) {
        {
            let val = override_conf.foo;
            if override_conf.foo_unset {
                self.foo = false;
            } else if let ::std::option::Option::Some(val) = val {
                self.foo = val;
            }
        }
    }
}
#[automatically_derived]
impl ::std::convert::From<WithDefaultOverrideConfig> for WithDefaultConfig {
    fn from(override_conf: WithDefaultOverrideConfig) -> Self {
        let mut out = Self::default();
        out.merge_with_override(override_conf);
        out
    }
}
#[automatically_derived]
impl ::std::ops::Add<WithDefaultOverrideConfig> for WithDefaultConfig {
    type Output = Self;
    fn add(mut self, rhs: WithDefaultOverrideConfig) -> Self {
        self += rhs;
        self
    }
}
#[automatically_derived]
impl ::std::ops::AddAssign<WithDefaultOverrideConfig> for WithDefaultConfig {
    fn add_assign(&mut self, rhs: WithDefaultOverrideConfig) {
        self.merge_with_override(rhs);
    }
}
/// Provides a builder used to configure [`WithDefaultConfig`](struct@WithDefaultConfig) at compile-time.
///
/// Can be constructed using either [`WithDefaultConfig::builder()`](method@WithDefaultConfig::builder) or [`WithDefaultConfigBuilder::new()`](method@WithDefaultConfigBuilder::new).
///
/// For run-time configuration of `WithDefaultConfig`, see [`WithDefaultOverrideConfig`](struct@WithDefaultOverrideConfig)
#[derive(Debug, Clone)]
#[derive(Copy)]
pub struct WithDefaultConfigBuilder {
    pub(crate) inner: WithDefaultConfig,
    pub(crate) override_conf: ::std::option::Option<WithDefaultOverrideConfig>,
}
impl WithDefaultConfigBuilder {
    /// Constructs a new builder instance
    #[must_use]
    #[allow(
        clippy::redundant_field_names,
        reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
    )]
    pub fn new() -> Self {
        Self {
            override_conf: ::std::option::Option::None,
            inner: <WithDefaultConfig>::new(),
        }
    }
    /// # Revert at run-time
    ///
    /// Has run-time support for reverting back to the provided default (**`false`**)
    ///
    /// See [`foo_unset()`](method@WithDefaultOverrideConfig::foo_unset) for more information
    ///
    /// ---
    ///
    /// - Library default: **`false`**
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
    pub fn with_override(mut self, override_conf: WithDefaultOverrideConfig) -> Self {
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
    /// Builds the [`WithDefaultConfig`]
    ///
    /// Values present in an override config (if supplied) will replace
    /// corresponding assignments made using the builder.
    #[must_use]
    pub fn build(mut self) -> WithDefaultConfig {
        <Self as ::lib_conf::traits::build::BuildConfig>::build(self)
    }
}
#[automatically_derived]
impl ::std::default::Default for WithDefaultConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}
#[automatically_derived]
impl ::lib_conf::traits::build::BuildConfig for WithDefaultConfigBuilder {
    type Config = WithDefaultConfig;
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
/// Provides support for configuring [`WithDefaultConfig`](struct@WithDefaultConfig) at run-time.
///
/// A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time.
///
/// `WithDefaultOverrideConfig` is provided for just this case.
///
/// ---
///
/// `WithDefaultOverrideConfig` provides run-time support for:
/// - *Setting* [`WithDefaultConfig`](struct@WithDefaultConfig) fields
/// - *Overriding* compile-time assignments (made using [`WithDefaultConfigBuilder`](struct@WithDefaultConfigBuilder))
/// - *Reverting* compile-time (or run-time) assignments back to provided defaults
/// # Example
///
/// ```rust,ignore
/// #[derive(Debug, Clone, Deserialize)]
/// pub struct Conf {
///     api: ApiConf,
///     // ..
///     #[serde(default)]
///     with_default: WithDefaultOverrideConfig,
/// }
/// ```
///
/// # Fields
///
/// | Name | Type |
/// | - | - |
/// | `foo` | `bool` |
///
/// ## Revert to default
///
/// For each of [`WithDefaultConfig`](struct@WithDefaultConfig)'s optional fields, there is a corresponding **`bool`** with a name in the form of `*_unset`.
///
/// If set to `true`, the associated field will have its value reverted to the provided default, e.g.
/// - if `foo_unset` deserializes to `true`:
///   - `foo` will be reverted back to the provided default: `false`
///
/// ### Unset fields
///
/// The following fields are supported:
///
/// | Name |
/// | - |
/// | `foo_unset` |
///
/// ### Unset aliases
///
/// Defaults can also be restored using the following aliases
///
/// | Alias |
/// | - |
/// | `reset` |
/// | `revert` |
/// | `clear` |
/// | `default` |
///
/// For example, `foo` could be reset using either `foo_unset`, or any of the following:
/// - `foo_reset`
/// - `foo_revert`
/// - `foo_clear`
/// - `foo_default`
#[derive(Debug, Clone)]
#[derive(Copy)]
#[derive(::serde::Deserialize)]
pub struct WithDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    pub foo: ::std::option::Option<bool>,
    #[serde(
        default,
        alias = "foo_reset",
        alias = "foo_revert",
        alias = "foo_clear",
        alias = "foo_default"
    )]
    /// flag allowing for reverting a builder-configured
    /// setting at runtime
    pub foo_unset: bool,
}
impl WithDefaultOverrideConfig {
    ///
    /// ---
    ///
    /// - Library default: **`false`**
    #[must_use]
    pub fn foo(&self) -> ::std::option::Option<bool> {
        self.foo
    }
    /// Whether the unset field for [`foo`](Self::foo) has explicitly been set to `true`.
    ///
    /// # Aliases
    ///
    /// Various aliases are supported in addition to reverting via `foo_unset`.
    ///
    /// - **Note:** unset aliases are only supported when the **`serde` feature is enabled**.
    ///
    /// <details>
    /// <summary><b>Expand to view supported unset field aliases</b></summary
    ///
    /// > - `foo_reset`
    /// > - `foo_revert`
    /// > - `foo_clear`
    /// > - `foo_default`
    ///
    /// </details>
    #[must_use]
    pub fn foo_unset(&self) -> bool {
        self.foo_unset
    }
}
#[automatically_derived]
#[allow(
    clippy::redundant_field_names,
    reason = "https://github.com/rust-lang/rust-clippy/issues/17525"
)]
impl ::std::default::Default for WithDefaultOverrideConfig {
    fn default() -> Self {
        Self {
            foo: ::std::option::Option::None,
            foo_unset: false,
        }
    }
}
