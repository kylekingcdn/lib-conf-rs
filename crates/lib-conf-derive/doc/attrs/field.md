## Field attributes <a name="field_attributes"></a>

### `copy` <a name="copy"></a>

Flag indicating that the field's type implements `Copy`.

This affects the return types of getter methods associated with this field. Getter return types
will be an owned type rather than a borrow/ref.

In order to avoid wonky return types, this should *always* be used if the type does in fact
support `Copy`.

#### Restrictions <a name="copy_restrictions"></a>

- The type must implement `Copy`
- This behavior does not apply to the `Override`'s getter if
  [`override_from`](#override_from) or [`override_via`](#override_via) are present
  - See [`override_from_copy`](#override_from_copy)
- See [Getter return types](#getter-return-types) for more information

---

#### Example <a name="copy_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(copy)]
    pub(crate) worker_count: Option<u8>,

    pub(crate) client_name: Option<String>,
}
```

### `default` <a name="default"></a>

Sets the default value for the given field, which makes initialization optional for consumers.

#### Restrictions <a name="default_restrictions"></a>

- | -
**Incompatible types** | `Option<_>`
**Supported values** | Literals (`true`, `5`), expressions (`String::from("local")`), fn paths (`MySdkConfig::default_for_this_field`), fn/method calls (`"example.com".to_string()`)

- Unlike `serde`'s `default` attribute, implicit (no-value, e.g. `#[config(default)]`) declarations are **not supported**
  - This is because the provided expression is used to provide the '**Library default**' in the generated docs

##### `Option<_>` incompatibility

The `default`attribute **cannot be used on `Option<_>` types**.

`Option<_>` fields have an implicit default value of `None`. Therefore, if this were supported, it would only be used as `default = Some(_)`.

This behavior would result in the inability to assign `None` at either compile-time or run-time.

If you find yourself reaching for this, consider changing the type from `Option<MyType>` to `MyType` and using the `default` attr.

#### Example <a name="default_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(default = false)]
    pub(crate) debug_mode: bool,

    #[config(default = true)]
    pub(crate) ssl_cert_verification: bool,

    #[config(default = "https://api.example.com".to_string())]
    pub(crate) endpoint: String,
}
```

---

### `skip_all` <a name="skip_all"></a>

Flag indicating that this field should be skipped in the `Override` struct, and any relevant
methods for the deriving, `Builder`, and `Override` structs.

This is identical to specifying the `config_skip_getter`, `builder_skip`, and `override_skip`
attributes.

#### Restrictions <a name="skip_all_restrictions"></a>

- | -
**Incompatible attrs** | `config_skip_getter`, `builder_skip`, `override_skip`, `override_required`,
  `override_from`, `override_from_copy`, `override_via`, `override_attr`
**Implied attrs** | `config_skip_getter`, `builder_skip`, `override_skip`

- This should probably only be used with fields that are either `Option<_>` or have
  the `default` attribute present
  - Otherwise, consumers are forced to initialize this as a parameter in the `builder()` fn.
- If your config struct is generic, any skipped fields will be substituted
  with `PhantomData<{type}>` fields in order to preserve generics in the `Builder`
  and `Override` structs
  - Therefore, this should not be used with generic fields where the
    concrete types which are not public-facing

#### Example <a name="skip_all_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Copy, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(skip_all)]
    pub(crate) some_internal_only_state: Option<bool>,
}
```

---

### `config_skip_getter` <a name="config_skip_getter"></a>

Flag used to skip the deriving struct's getter method for this field.

#### Restrictions <a name="config_skip_getter_restrictions"></a>

- | -
**Incompatible attrs** | `skip_all`

#### Example <a name="config_skip_getter_example"></a>

```rust
# use lib_conf_derive::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(config_skip_getter)]
    pub(crate) api_token: Option<SecretString>,
}
```

---

<!--
- | -
**Required attrs** |
**Incompatible attrs** |
**Implied attrs** |
**Incompatible types** |
**Supported values** |
-->

### `builder_skip` <a name="builder_skip"></a>

Flag used to skip the `Builder` struct setter method for this field.

A potential use-case is allowing an optional field to only be configured at runtime.

#### Example <a name="builder_skip_example"></a>

```rust
# use lib_conf_derive::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(builder_skip)]
    pub(crate) api_token: Option<SecretString>,
}
```

#### Restrictions <a name="builder_skip_restrictions"></a>

- Incompatible with: `skip_all`, `override_required` (implied by)

---

### `override_skip` <a name="override_skip"></a>

Flag used to prevent inclusion of this field in the generated `Override` struct.

Can be used to prevent runtime adjustment of a setting.

#### Example <a name="override_skip_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(override_skip)]
    pub(crate) client_version: Option<String>,
}
```

#### Restrictions <a name="override_skip_restrictions"></a>

- Incompatible with: `skip_all`, `override_required`, `override_from`, `override_via`,
  `override_attr`
- If your config struct is generic, any skipped fields will be substituted
  with `PhantomData<{type}>` fields in order to preserve generics in the `Builder`
  and `Override` structs
  - Therefore, this should probably not be used with generic fields where the
    concrete types are not public-facing

---

### `override_required` <a name="override_required"></a>

By default, fields in the `Override` struct will always be wrapped in an `Option`, even if the
source type is fully required (non-`Option`, no `default` attr).

This is the default behaviour because the `Override` struct is an additional layer used to allow
overriding `Builder`-configured (or library-provided default) settings. It is by design that
integrating the `Override` struct is entirely optional. It's available to dependant crates that wish
to support runtime configuration of your library, and seamlessly absent for those that don't.

This attribute reverts this behaviour, enforcing presence of this field at runtime.

> Note: the following explanation uses a fake deriving struct name of `MySdkConfig` to improve legibility.

An undesirable consequence - the `MySdkOverrideConfig` struct loses its `Default` impl as it now has a
required field.

- To account for this, the `MySdkConfig::builder()` and `MySdkConfigBuilder::new()` functions now require
  a `override_conf: MySdkOverrideConfig` parameter.
- Further, as it doesn't make sense to provide a `MySdkConfigBuilder::with_override()` method after having just
  initialized the builder with one, the `MySdkConfigBuilder::with_override` method is completely removed.

The *only* scenario where you should consider using this, is one where **all** of the following conditions
are met:

- The field stores a sensitive value and you wish to deter hard-coded initialization of the field.
- You recognize the introduction or removal of this attribute is a **guaranteed breaking change**.
- Your library dependants are already required to initialize some or all configuration at runtime.
  - This will typically only be true for internal/personal-use crates or for very large frameworks
    such as `Loco`.

#### Example <a name="override_required_example"></a>

```rust
# use lib_conf_derive::LibConfig;
use secrecy::SecretString;

#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(override_required)]
    pub(crate) api_token: SecretString,
}
```

#### Restrictions <a name="override_required_restrictions"></a>

- Implies: `builder_skip`
- Incompatible attributes: `default`, `skip_all`, `builder_skip`, `override_skip`
- Incompatible types: `Option<_>`
- Runtime configuration is now mandatory and is no longer opt-in
- The `Builder` struct loses its `with_override()` method
- The `Builder` struct must be initialized with an `Override` config
- No setter is available for this field in the `Builder` struct

---

### `override_from` <a name="override_from"></a>

Allows for specifying a custom type to be used for the field in the `Override` struct.

The mapped type must implement `Into` against the field's type used in the deriving struct.

This is most useful for types that lack or have undesired/incompatible `Deserialize` impl's.

#### Example <a name="override_from_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(
        copy, default = ModuleSupport::Automatic,
        override_from = bool, override_from_copy,
    )]
    pub(crate) proxy_module_enabled: ModuleSupport,
}

#[derive(Debug, Clone, Copy)]
pub enum ModuleSupport {
    Automatic,
    Enabled,
    Disabled,
}
impl From<bool> for ModuleSupport {
    fn from(enable: bool) -> Self {
        if enable {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}
```

#### Restrictions <a name="override_from_restrictions"></a>

- `Option<_>`-wrapping of the type in the `Override` struct is handled automatically (only
  applicable when `override_required` is not present)
- Incompatible with: `skip_all`, `override_skip`
- As the mapped type is directly used in the `Override` struct, the type should be public

---

### `override_from_copy` <a name="override_from_copy"></a>

Marks the `override_from` type as implementing `Copy`.

This is used to support correct return types in `Override` struct getters.

#### Example <a name="override_from_copy_example"></a>

See the [`override_via` example](#override_via_example) for usage.

#### Restrictions <a name="override_from_copy_restrictions"></a>

- Must be used with: [`override_from`](#override_from)

---

### `override_via` <a name="override_via"></a>

To be used alongside `override_from`, this attribute allows for using some intermediate type to
handle the conversion between the `override_from` type and the `Config` field type.

This type must implement `From<OverrideFrom>` (where `OverrideFrom` is the type provided
by `override_from`) as well as `Into<T>` (where `T` is the field type defined in the
deriving `Config` struct.

This is most useful for types that lack or have undesired/incompatible `Deserialize` impl's.

#### Example <a name="override_via_example"></a>

In this example, the `Override` struct stores the interval as a `u64`, allowing for runtime
configuration of the interval in seconds.

A custom adapter struct is used to handle the conversion, as `Duration` doesn't
implement `From<u64>`.

This adapter is only used internally during the `Override` merge, and therefore does not need to be
public.

```rust
# use lib_conf_derive::LibConfig;
use std::time::Duration;

#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(
        copy, default = Duration::from_secs(30),
        override_from = u64,
        override_from_copy,
        override_via = SecondsAdapter,
    )]
    pub(crate) refresh_interval: Duration,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SecondsAdapter(pub u64);

impl From<u64> for SecondsAdapter {
    fn from(val: u64) -> Self {
        Self(val)
    }
}
impl From<SecondsAdapter> for Duration {
    fn from(adapter: SecondsAdapter) -> Self {
        Self::from_secs(adapter.0)
    }
}
```

#### Restrictions <a name="override_via_restrictions"></a>

- Must be used with: [`override_from`](#override_from)
- Incompatible with: `skip_all`, `override_skip`

---

### `override_attr` <a name="override_attr_field"></a>

Passes through the contained attributes to the corresponding field in the generated `Override`
struct.

Can contain multiple attributes and can be specified multiple times.

#### Example <a name="override_attr_field_example"></a>

```rust
# #[cfg(feature = "serde")]
# {
# use lib_conf_derive::LibConfig;
use url::Url;

#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
pub struct MySdkConfig {
    #[config(override_attr(serde(alias = "db_url")))]
    pub(crate) database_url: Option<Url>,
}
# }
```

#### Restrictions <a name="override_attr_field_restrictions"></a>

- Incompatible with: `skip_all`, `override_skip`
