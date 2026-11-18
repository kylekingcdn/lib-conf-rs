// !- Utils

#[doc(hidden)]
#[must_use]
pub fn error_field_list(names: &[String]) -> String {
    names.iter().map(|name| format!("- `{name}`")).collect::<Vec<_>>().join("\n")
}
