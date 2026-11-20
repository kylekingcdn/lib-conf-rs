## Getter return types

Return types of getter fn's for the user-provided `Config` struct and the `Override` struct are
decided as follows:

- Are both `#[config(override_from)]` and `#[config(override_from_copy)]` present?
  - Return type **for `Override` struct** matches type provided by `#[config(override_from)]`

- Does the field have the [`#[config(copy)]`](#copy) attribute present?
  - Return type matches field type

- Is the field's type `Option<String>`?
  - Return type is **`Option<&str>`**
  - Handled by calling `as_deref()` on the field

- Is the field's type `Option<_>`?
  - Return type is **`Option<&_>`**
  - Handled by calling `as_ref()` on the field

- Is the field's type `String`?
  - Return type is **`&str`**
  - Handled by calling `as_str()` on the field

- Otherwise:
  - Return type is a borrow, e.g. `&T` for a field of type `T`
