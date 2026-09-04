use super::attr::{FieldAttr, Shape};

use proc_macro2::Span;
use syn::{Error, Ident};

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
    #[error("Expected struct name '{name}' to end with one of: {suffixes}")]
    UnknownStructSuffix { name: String, suffixes: String, span: Span },
    #[error("Unknown attribute: '{ident}'")]
    UnknownAttr { ident: Ident },
    //#[error("Invalid attribute expression type: {expr:?}")]
    //InvalidAttrExpr { span: Span, expr: Expr },
    #[error("Invalid attribute expression type")]
    InvalidAttrExpr { span: Span },
    #[error("Attribute '{ident}' specified multiple times")]
    DuplicateAttr { ident: Ident },
    #[error("Attribute '{ident}' cannot be used with '{other}'")]
    IncompatibleAttrs { ident: Ident, other: Ident},
    #[error("Attribute '{ident}' is automatically implied by '{other}'")]
    ImplicitAttr { ident: Ident, other: Ident},
    #[error("Unexpected shape {shape:#?} for attribute '{ident}'")]
    InvalidAttrShape { ident: Ident, shape: Shape },
    #[error("Missing attribute '{missing}', which is required while using '{present}'")]
    MissingAttrDep { present: Ident, missing: FieldAttr },
    #[error("Attribute '{ident}' cannot be used with Option types")]
    AttrOptionUnsupported { ident: Ident },
}
impl ParseError {
    pub fn span(self) -> Span {
        match self {
            Self::EnumUnsupported(span) |
            Self::UnionUnsupported(span) |
            Self::UnnamedStructUnsupported(span) |
            Self::UnitStructUnsupported(span) |
            Self::UnknownStructSuffix { span, .. } |
            Self::InvalidAttrExpr { span, .. } => span,
            Self::UnknownAttr { ident } |
            Self::IncompatibleAttrs { ident, .. } |
            Self::ImplicitAttr { ident, .. } |
            Self::DuplicateAttr { ident, .. } |
            Self::InvalidAttrShape { ident, .. } |
            Self::AttrOptionUnsupported { ident } => ident.span(),
            Self::MissingAttrDep { present, .. } => present.span(),
        }
    }
    pub fn error(self) -> Error {
        self.into()
    }
}
impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        let msg = error.to_string();
        Self::new(error.span(), msg)
    }
}
