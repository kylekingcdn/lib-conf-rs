use super::attr::Shape;

use proc_macro2::Span;
use syn::{Error, Ident};
use std::rc::Rc;

#[derive(Debug, thiserror::Error)]
pub(crate) enum ParseError {
    #[error("Enums are not supported")]
    EnumUnsupported(Span),
    #[error("Unions are not supported")]
    UnionUnsupported(Span),
    #[error("Unnamed structs are not supported")]
    UnnamedStructUnsupported(Span),
    #[error("Union structs are not supported")]
    UnitStructUnsupported(Span),

    #[error("Expected struct name `{name}` to end with one of: {suffixes}")]
    UnknownStructSuffix { name: String, suffixes: String, span: Span },

    #[error("Unknown attribute: `{ident}`")]
    UnknownAttr { ident: Ident },
    #[error("Unexpected shape `{shape:#?}` for attribute `{ident}`.\n\nValid shapes are: [ {} ]", valid.join(","))]
    InvalidAttrShape { ident: Ident, shape: Shape, valid: Vec<String> },
    #[error("Invalid attribute expression type")]
    InvalidAttrExpr { span: Span },
    #[error("Expected string expr value, e.g. `#[config(_ = \"..\")]`")]
    ExpectedStrExpr { span: Span },

    // #[error("`{ident}` is missing required `#[config(..)]` attribute")]
    // MissingRequiredAttrHelper { ident: Ident },
    #[error("Missing required attribute: `{name}`")]
    MissingRequiredAttr { name: &'static str, span: Span },
    #[error("Attribute `{ident}` cannot be used with Option types")]
    AttrOptionUnsupported { ident: Ident },
    #[error("Attribute `{ident}` specified multiple times")]
    DuplicateAttr { ident: Ident },
    #[error("Attribute `{ident}` cannot be used with `{other}`")]
    IncompatibleAttrs { ident: Ident, other: Ident},
    #[error("Attribute `{ident}` is automatically implied by `{other}`")]
    ImplicitAttr { ident: Ident, other: Ident},
    #[error("Missing attribute `{missing}`, which is required while using `{present}`")]
    MissingAttrDep { present: Ident, missing: &'static str },
//     #[error("Attributes `{first}` and `{second}` cannot be used simultaneously")]
//     MultipleExclusiveDeps { first: Ident, second: Ident },
//     #[error(
//         "When the `{present}` attribute is used, exactly one of the following must also be provided: [\n{}\n]",
//         supported.into_iter().map(|v| format!("    `{v}`")).collect::<Vec<_>>().join("\n"),
//     )]
//     MissingExclusiveDep { present: Ident, supported: Vec<&'static str> },

    #[error("`nested(..)` requires either `nested(build)` or `nested(try_build)` to be present, corresponding to whichever is provided by `{builder_ident}`.
- If `{builder_ident}` has a `build()` method, use `nested(build)`
- Otherwise, if `{builder_ident}` has a `try_build()` method, use `nested(try_build)`")]
    NestedMissingBuildVariant { ident: Ident, builder_ident: Rc<Ident> },

    #[error("`nested(..)` requires one of the following to be present:
    - `nested(from_override)`
    - `nested(try_from_override)`
    - `nested(no_from_override)`

If `{config_ident}` has a `From<{override_ident}>` impl, use:
    nested(from_override)

Otherwise, if `{config_ident}` has a `TryFrom<{override_ident}>` impl, use:
    nested(try_from_override)

Otherwise, use:
    nested(no_from_override)")]
    NestedMissingFromOverrideVariant { ident: Ident, config_ident: Rc<Ident>, override_ident: Rc<Ident> },

    #[error("Missing `nested(config_error = ..)`, which is required since nested `{config_ident}` (or related struct) has at least 1 fallible fn (as indicated by presence of `nested({fallible_attr_ident})`.

To resolve:
  - Ensure the error type provided by `{fallible_item_path}` is in scope (imported)
    - The error type is *potentially* `{config_ident}Error`
  - Use the error's type as the value in `nested(config_error = ..)` .
    - E.g (assuming the above inferred type is correct):

      `nested(config_error = {config_ident}Error)`")]
    NestedFallibleMissingError {
        ident: Ident,
        config_ident: Rc<Ident>,
        fallible_attr_ident: Ident,
        fallible_item_path: String,
    },

    #[error("Conditions for nesting `{config_ident}` as  `Option<>` are not currently met.

Support for nesting `{config_ident}` as `Option<>` can be acheived by:
- Adjusting `{config_ident}` to meet requirements for a derived `From/TryFrom<{override_ident}>` impl
  - This is only feasible if `{config_ident}` is provided by this crate.
  - For more information, see the 'Background' docs section of `lib-conf-derive`

Alternatively, to make this field optional without the use of `Option`:
- Write a `fn` that returns a desired default for `{config_ident}`
- Assign the function as the default-provider via `#[config(default = fn_name)]`
 ")]
    NestedOptionNoFromOverride {
        field_type_span: Span,
        config_ident: Rc<Ident>,
        override_ident: Rc<Ident>,
    },
}
impl ParseError {
    pub(crate) fn span(self) -> Span {
        match self {
            Self::EnumUnsupported(span) |
            Self::UnionUnsupported(span) |
            Self::UnnamedStructUnsupported(span) |
            Self::UnitStructUnsupported(span) |
            Self::UnknownStructSuffix { span, .. } |
            Self::MissingRequiredAttr { span, .. } |
            Self::ExpectedStrExpr { span, .. } |
            Self::InvalidAttrExpr { span, .. } => span,

            Self::UnknownAttr { ident } |
            Self::IncompatibleAttrs { ident, .. } |
            Self::ImplicitAttr { ident, .. } |
            Self::DuplicateAttr { ident, .. } |
            Self::InvalidAttrShape { ident, .. } |
            // Self::MissingRequiredAttrHelper { ident, .. } |
            Self::AttrOptionUnsupported { ident }|
            Self::NestedMissingBuildVariant { ident, .. } |
            Self::NestedMissingFromOverrideVariant { ident, .. } |
            Self::NestedFallibleMissingError { ident, .. } => ident.span(),

            Self::NestedOptionNoFromOverride { field_type_span, .. } => field_type_span,
            //Self::MultipleExclusiveDeps { second, .. } => second.span(),

            //Self::MissingExclusiveDep { present, .. } |
            Self::MissingAttrDep { present, .. } => present.span(),
        }
    }
    pub(crate) fn error(self) -> Error {
        self.into()
    }
}
impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        let msg = error.to_string();
        Self::new(error.span(), msg)
    }
}
