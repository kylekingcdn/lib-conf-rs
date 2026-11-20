## Struct attributes <a name="struct_attributes"></a>

### `#[config_docs]` <a name="config_docs"></a>

- ***WIP***

#### Example <a name="config_docs_example"></a>

```rust,ignore
todo!();
```

---

### `example_field_name` <a name="example_field_name"></a>

Sets the field name to use for the Config struct in the generated example docs.

#### Example <a name="example_field_name_example"></a>

The following config/attributes will generate the following documentation

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(example_field_name = "my_sdk")]
pub struct MySdkConfig {
  // ...
}
```

**Generated example docs:**

```rust,ignore
#[derive(Debug, Clone, Deserialize)]
pub struct Conf {
    // ..
    #[serde(default)]
    my_sdk: MySdkOverrideConfig,
//  ^^^^^^
}
```

---

### `lib_conf_attribution` <a name="lib_conf_attribution"></a>

- ***WIP***

#### Example <a name="lib_conf_attribution_example"></a>

```rust,ignore
todo!();
```

---

### `validate` <a name="validate"></a>

- ***WIP***

#### Example <a name="validate_example"></a>

```rust,ignore
todo!();
```

---

### `validate_error` <a name="validate_error"></a>

- ***WIP***

#### Example <a name="validate_error_example"></a>

```rust,ignore
todo!();
```

---

### `no_autoderive` <a name="no_autoderive"></a>

- Disables automatic inclusion of common traits impls or derives for each generated struct type.

**Traits that are automatically derived, along with the associated struct and required conditions are as follows:**

- [**`Debug`**](https://doc.rust-lang.org/std/fmt/trait.Debug.html) will be auto-derived for:
  - **`Override`**
  - **`Builder`**

- [**`Clone`**](https://doc.rust-lang.org/std/clone/trait.Clone.html) will be auto-derived for:
  - **`Override`**
  - **`Builder`**

- [**`Copy`**](https://doc.rust-lang.org/std/market/trait.Copy.html) will be auto-derived for:
  - **`Config`**
    - if all fields have [`#[config(copy)]`](#copy)

  - **`Override`**
    - all unskipped fields ***with*** [`#[config(override_from)]`](#override_from) also have [`#[config(override_from_copy)]`](#override_from_copy)
    - all unskipped field ***without*** [`#[config(override_from)]`](#override_from) have [`#[config(copy)]`](#copy)

  - **`Builder`**
    - if `Config` and `Override` both auto-derive `Copy` (above 2 conditions are met)

- **Note**

  As they are required for base functionality, the following will always be derived when possible -
  even when **`no_autoderive`** is present.

  - [**`Default`**](https://doc.rust-lang.org/std/default/trait.Default.html) will always be derived for:
    - **`Config`**
      - if all fields have either an `Option<_>` type or have [`#[config(default)]`](#default)

    - **`Override`**
      - if no fields have a [`#[config(override_required)]`](#override_required)

    - **`Builder`**
      - if `Config` and `Override` both auto-derive `Default` (above 2 conditions are met)

  - [**`Deserialize`**](https://docs.rs/serde//serde/trait.Deserialize.html) will always be derived for:
    - **`Override`**
      - if the `serde` feature is enabled

#### Example <a name="autoderive_example"></a>

```rust,ignore
todo!();
```

---

### `derive` <a name="derive"></a>

Applies the provided derives to both the `Builder` and `Override` generated structs.

Can contain multiple derives and can be specified multiple times.

#### Restrictions <a name="derive_restrictions"></a>

- Cannot contain duplicates, including those which are derived automatically.
  - See the [Auto-derived traits](#auto_derived_traits) section for more details.
  - `Debug`
    - Derived on `Override` and `Builder`
  - `Clone`
    - Derived on `Override` and `Builder`
  - `Copy` if all unskipped fields use '`#[config(copy)]` (or `#[config(override_skip)]`, where applicable)
  - `Deserialize` on the `Override` struct (`serde` feature enabled)

#### Example <a name="derive_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, PartialEq, Eq, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(derive(PartialEq, Eq))]
pub struct MySdkConfig {
    #[config(copy, default = false)]
    pub(crate) verbose: bool,
}
```

---

### `builder_derive` <a name="builder_derive"></a>

Applies the provided derives to the generated `Builder` struct.

Can contain multiple derives and can be specified multiple times.

#### Restrictions <a name="builder_derive_restrictions"></a>

- Cannot contain duplicates, including the following which are derived automatically:
  - `Debug`
  - `Clone`
  - `Copy` if all fields use '`#[config(copy)]`

#### Example <a name="builder_derive_example"></a>

In reality, this example is very questionable.

I can't reason why someone would want to derive `Deserialize` on a Builder struct.
However, this does showcase how to handle `serde` attributes in `serde`-optional libraries.

We can't use the `#[config(derive(..))]` here, as `Deserialize` gets auto-derived for `Override`
structs if the `serde` feature is enabled.

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
#[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))] // derive Deserialize on this struct
#[cfg_attr(feature = "serde", config(builder_derive(serde::Deserialize)))] // derive on builder
pub struct MySdkConfig {
    // ...
}
```

For `serde`-mandatory, this would be done as follows:

```rust
# #[cfg(feature = "serde")]
# {
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig, serde::Deserialize)] // derive Deserialize on this struct
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(builder_derive(serde::Deserialize))] // derive Deserialize on the Builder struct
pub struct MySdkConfig {
    // ...
}
# }
```

---

### `override_derive` <a name="override_derive"></a>

Applies the provided derives to the generated `Override` struct.

Can contain multiple derives and can be specified multiple times.

#### Restrictions <a name="override_derive_restrictions"></a>

- Cannot contain duplicates, including the following which are derived automatically:
  - `Debug`
  - `Clone`
  - `Copy` if all unskipped fields use '`#[config(copy)]` (or `#[config(override_skip)]`, where applicable)
  - `Deserialize` (`serde` feature enabled)

#### Example <a name="override_derive_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(override_derive(serde::Serialize))]
pub struct MySdkConfig {
    // ...
}
```

---

### `attr` <a name="attr"></a>

Passes through the contained attributes to the `Builder` and `Override` generated structs.

Can contain multiple attributes and can be specified multiple times.

#### Example <a name="attr_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```

---

### `builder_attr` <a name="builder_attr"></a>

Passes through the contained attributes to the generated `Builder` struct.

Can contain multiple attributes and can be specified multiple times.

#### Example <a name="builder_attr_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(builder_attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```

---

### `override_attr` <a name="override_attr_struct"></a>

Passes through the contained attributes to the generated `Override` struct.

Can contain multiple attributes and can be specified multiple times.

#### Example <a name="override_attr_struct_example"></a>

```rust
# use lib_conf_derive::LibConfig;
#[derive(Debug, Clone, LibConfig)]
# #[config(traits_crate = lib_conf_traits, static_assertions_crate = static_assertions)]
#[config(override_attr(non_exhaustive))]
pub struct MySdkConfig {
    // ...
}
```
