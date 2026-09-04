use crate::{
    generate::util::render_expr,
    parse::error::ParseError,
};

use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;
use syn::{Expr, ExprCall, ExprLit, ExprPath, Ident, parse_quote, spanned::Spanned};

// !- Attribute keys

pub static HELPER_ATTR_CONFIG: &str = "config";
pub static DOC_ATTR: &str = "doc";

// !- Struct attr

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum StructAttr {
    Derive,
    BuilderDerive,
    OverrideDerive,
    
    Attr,
    BuilderAttr,
    OverrideAttr,
}
impl StructAttr {
    pub const DERIVE: &'static str = "derive";
    pub const BUILDER_DERIVE: &'static str = "builder_derive";
    pub const OVERRIDE_DERIVE: &'static str = "override_derive";
    pub const ATTR: &'static str = "attr";
    pub const BUILDER_ATTR: &'static str = "builder_attr";
    pub const OVERRIDE_ATTR: &'static str = "override_attr";

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Derive => Self::DERIVE,
            Self::BuilderDerive => Self::BUILDER_DERIVE,
            Self::OverrideDerive => Self::OVERRIDE_DERIVE,
            Self::Attr => Self::ATTR,
            Self::BuilderAttr => Self::BUILDER_ATTR,
            Self::OverrideAttr => Self::OVERRIDE_ATTR,
        }
    }
    pub(crate) fn supported_shapes(&self) -> &Vec<Shape> {
        match STRUCT_ATTR_SHAPES.get(self) {
            Some(x) => x,
            None => panic!("Missing supported shapes definition for struct attr: '{self}'"),
        }
    }
    pub(crate)fn supports_shape(self, shape: Shape) -> bool {
        self.supported_shapes().contains(&shape)
    }
    pub(crate) fn validate_shape(self, shape: Shape, ident: &Ident) -> Result<(), ParseError> {
        if self.supports_shape(shape) {
            Ok(())
        } else {
            Err(ParseError::InvalidAttrShape { ident: ident.clone(), shape })
        }
    }
}
// ! Field attr trait impls

impl fmt::Display for StructAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<StructAttr> for &'static str {
    fn from(struct_attr: StructAttr) -> Self {
        struct_attr.as_str()
    }
}
impl From<&StructAttr> for &'static str {
    fn from(struct_attr: &StructAttr) -> Self {
        struct_attr.as_str()
    }
}
impl From<StructAttr> for String {
    fn from(struct_attr: StructAttr) -> Self {
        struct_attr.to_string()
    }
}
impl From<&StructAttr> for String {
    fn from(struct_attr: &StructAttr) -> Self {
        struct_attr.to_string()
    }
}
impl TryFrom<&Ident> for StructAttr {
    type Error = ParseError;

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            Self::DERIVE => Ok(Self::Derive),
            Self::BUILDER_DERIVE => Ok(Self::BuilderDerive),
            Self::OVERRIDE_DERIVE => Ok(Self::OverrideDerive),
            Self::ATTR => Ok(Self::Attr),
            Self::BUILDER_ATTR => Ok(Self::BuilderAttr),
            Self::OVERRIDE_ATTR => Ok(Self::OverrideAttr),
            _unknown => Err(ParseError::UnknownAttr { ident: value.clone() })
        }
    }
}
impl TryFrom<Ident> for StructAttr {
    type Error = ParseError;

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

// ! Struct attr statics

pub static STRUCT_ATTR_SHAPES: LazyLock<HashMap<StructAttr, Vec<Shape>>> = LazyLock::new(|| [
    (StructAttr::Derive,            vec![Shape::List]),
    (StructAttr::BuilderDerive,     vec![Shape::List]),
    (StructAttr::OverrideDerive,    vec![Shape::List]),
    (StructAttr::Attr,              vec![Shape::List]),
    (StructAttr::BuilderAttr,       vec![Shape::List]),
    (StructAttr::OverrideAttr,      vec![Shape::List]),
].into());

// !- Field attr

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum FieldAttr {
    Copy,
    Default,
    SkipAll,

    ConfigSkipGetter,

    BuilderSkip,
    
    OverrideAttr,
    OverrideSkip,
    OverrideRequired,
    OverrideFrom,
    OverrideVia,
}
impl FieldAttr {
    pub const COPY: &'static str = "copy";
    pub const DEFAULT: &'static str = "default";
    pub const SKIP_ALL: &'static str = "skip_all";

    pub const CONFIG_SKIP_GETTER: &'static str = "config_skip_getter";

    pub const BUILDER_SKIP: &'static str = "builder_skip";

    pub const OVERRIDE_ATTR: &'static str = "override_attr";
    pub const OVERRIDE_SKIP: &'static str = "override_skip";
    pub const OVERRIDE_REQUIRED: &'static str = "override_required";
    pub const OVERRIDE_FROM: &'static str = "override_from";
    pub const OVERRIDE_VIA: &'static str = "override_via";

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Copy => Self::COPY,
            Self::Default => Self::DEFAULT,
            Self::SkipAll => Self::SKIP_ALL,
            Self::ConfigSkipGetter => Self::CONFIG_SKIP_GETTER,
            Self::BuilderSkip => Self::BUILDER_SKIP,
            Self::OverrideAttr => Self::OVERRIDE_ATTR,
            Self::OverrideSkip => Self::OVERRIDE_SKIP,
            Self::OverrideRequired => Self::OVERRIDE_REQUIRED,
            Self::OverrideFrom => Self::OVERRIDE_FROM,
            Self::OverrideVia => Self::OVERRIDE_VIA,
        }
    }

    pub fn dependencies(&self) -> &Vec<Self> {
        if let Some(deps) = FIELD_ATTR_DEPS.get(self) {
             deps
        } else {
            &FIELD_ATTR_EMPTY_VEC
        }
    }
    pub fn incompatible_attrs(&self) -> &Vec<Self> {
        if let Some(mutex) = FIELD_ATTR_MUT_EX.get(self) {
             mutex
        } else {
            &FIELD_ATTR_EMPTY_VEC
        }
    }
    pub fn implied_attrs(&self) -> &Vec<Self> {
        if let Some(mutex) = FIELD_ATTR_IMPLICIT.get(self) {
             mutex
        } else {
            &FIELD_ATTR_EMPTY_VEC
        }
    }
    pub(crate) fn supported_shapes(&self) -> &Vec<Shape> {
        match FIELD_ATTR_SHAPES.get(self) {
            Some(x) => x,
            None => panic!("Missing supported shapes definition for field attr: '{self}'"),
        }
    }
    pub(crate)fn supports_shape(self, shape: Shape) -> bool {
        self.supported_shapes().contains(&shape)
    }
    pub(crate) fn validate_shape(self, shape: Shape, ident: &Ident) -> Result<(), ParseError> {
        if self.supports_shape(shape) {
            Ok(())
        } else {
            Err(ParseError::InvalidAttrShape { ident: ident.clone(), shape })
        }
    }
    pub(crate) fn is_passthrough(&self) -> bool {
        FIELD_PASSTHROUGH_ATTRS.contains(self)
    }
}

// ! Field attr trait impls

impl fmt::Display for FieldAttr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
impl From<FieldAttr> for &'static str {
    fn from(field_attr: FieldAttr) -> Self {
        field_attr.as_str()
    }
}
impl From<&FieldAttr> for &'static str {
    fn from(field_attr: &FieldAttr) -> Self {
        field_attr.as_str()
    }
}
impl From<FieldAttr> for String {
    fn from(field_attr: FieldAttr) -> Self {
        field_attr.to_string()
    }
}
impl From<&FieldAttr> for String {
    fn from(field_attr: &FieldAttr) -> Self {
        field_attr.to_string()
    }
}
impl TryFrom<&Ident> for FieldAttr {
    type Error = ParseError;

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value.to_string().as_str() {
            Self::COPY => Ok(Self::Copy),
            Self::DEFAULT => Ok(Self::Default),
            Self::SKIP_ALL => Ok(Self::SkipAll),
            Self::CONFIG_SKIP_GETTER => Ok(Self::ConfigSkipGetter),
            Self::BUILDER_SKIP => Ok(Self::BuilderSkip),
            Self::OVERRIDE_ATTR => Ok(Self::OverrideAttr),
            Self::OVERRIDE_SKIP => Ok(Self::OverrideSkip),
            Self::OVERRIDE_REQUIRED => Ok(Self::OverrideRequired),
            Self::OVERRIDE_FROM => Ok(Self::OverrideFrom),
            Self::OVERRIDE_VIA => Ok(Self::OverrideVia),
            _unknown => Err(ParseError::UnknownAttr { ident: value.clone() })
        }
    }
}
impl TryFrom<Ident> for FieldAttr {
    type Error = ParseError;

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        (&value).try_into()
    }
}

// ! Field attr statics

/// Attributes that depend on other attrs being present
pub static FIELD_ATTR_DEPS: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
    (FieldAttr::OverrideVia, vec![FieldAttr::OverrideFrom]),
].into());

/// (Associated) attributes that can't be used simultaneously
/// (AKA mutually exclusive)
///
/// NOTE: only one side of the exclusivity is required
pub static FIELD_ATTR_MUT_EX: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
    (FieldAttr::OverrideRequired, vec![
        FieldAttr::Default,
        // FieldAttr::BuilderSkip, // covered by implicit
        // FieldAttr::OverrideSkip, // covered by inverse
    ]),
    (FieldAttr::OverrideSkip, vec![
        FieldAttr::OverrideRequired,
        FieldAttr::OverrideFrom,
        FieldAttr::OverrideVia,
    ]),
    // any attributes incompatible with any of the 3 _skip attrs
    (FieldAttr::SkipAll, vec![
        FieldAttr::OverrideRequired,
        FieldAttr::OverrideFrom,
        FieldAttr::OverrideVia,
    ]),
    (FieldAttr::OverrideAttr, vec![
        FieldAttr::SkipAll,
        FieldAttr::OverrideSkip,
    ]),
].into());

pub static FIELD_ATTR_IMPLICIT: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
    (FieldAttr::SkipAll, vec![
        FieldAttr::ConfigSkipGetter,
        FieldAttr::BuilderSkip,
        FieldAttr::OverrideSkip,
    ]),
    (FieldAttr::OverrideRequired, vec![
        FieldAttr::BuilderSkip,
    ]),
].into());

static FIELD_ATTR_EMPTY_VEC: LazyLock<Vec<FieldAttr>> = LazyLock::new(Vec::new);

pub static FIELD_ATTR_SHAPES: LazyLock<HashMap<FieldAttr, Vec<Shape>>> = LazyLock::new(|| [
    (FieldAttr::Copy,               vec![Shape::Flag]),
    (FieldAttr::Default,            vec![Shape::KeyValue]),
    (FieldAttr::SkipAll,            vec![Shape::Flag]),
    (FieldAttr::ConfigSkipGetter,   vec![Shape::Flag]),
    (FieldAttr::BuilderSkip,        vec![Shape::Flag]),
    (FieldAttr::OverrideAttr,       vec![Shape::List]),
    (FieldAttr::OverrideSkip,       vec![Shape::Flag]),
    (FieldAttr::OverrideRequired,   vec![Shape::Flag]),
    (FieldAttr::OverrideFrom,       vec![Shape::KeyValue]),
    (FieldAttr::OverrideVia,        vec![Shape::KeyValue]),
].into());

static FIELD_PASSTHROUGH_ATTRS: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
    FieldAttr::OverrideAttr,
].into());

// !- Attr shape

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Shape {
    Flag,
    KeyValue,
    List,
}

// !- Validation

pub fn validate_unique(attr: FieldAttr, seen: &HashMap<FieldAttr, Ident>) -> Result<(), ParseError> {
    if let Some(ident) = seen.get(&attr) {
        Err(ParseError::DuplicateAttr { ident: ident.clone() })
    } else {
        Ok(())
    }
}
pub fn validate_implicit(seen: &HashMap<FieldAttr, Ident>) -> Result<(), ParseError> {
    for (attr, ident) in seen {
        for implicit in attr.implied_attrs() {
            if let Some(implicit_ident) = seen.get(implicit) {
                return Err(ParseError::ImplicitAttr {
                    ident: implicit_ident.clone(), // error on implied
                    other: ident.clone(),
                });
            }
        }
    }
    Ok(())
}
pub fn validate_mutex(seen: &HashMap<FieldAttr, Ident>) -> Result<(), ParseError> {
    for (attr, ident) in seen {
        for incompat in attr.incompatible_attrs() {
            if let Some(incompat_ident) = seen.get(incompat) {
                return Err(ParseError::IncompatibleAttrs {
                    ident: incompat_ident.clone(), // error on implied
                    other: ident.clone(),
                });
            }
        }
    }
    Ok(())
}
pub fn validate_deps(seen: &HashMap<FieldAttr, Ident>) -> Result<(), ParseError> {
    for (attr, ident) in seen {
        let attr_deps = attr.dependencies();
        for dep in attr_deps {
            if !seen.contains_key(dep) {
                return Err(ParseError::MissingAttrDep { present: ident.clone(), missing: *dep });
            }
        }
    }

    Ok(())
}

// !- Attribute expression variants

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) enum AttrExpr {
    Lit(ExprLit),
    Path(ExprPath),
    Call(ExprCall),
}
impl AttrExpr {
    pub fn none_expr() -> Self {
        Self::Path(parse_quote!(None))
    }
    pub fn to_pretty_string(&self) -> String {
        let tokens = self.to_token_stream();
        if let Some(pretty) = render_expr(&tokens) {
            pretty
        } else {
            tokens.to_string()
        }
    }
}
impl ToTokens for AttrExpr {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Lit(lit) => lit.to_tokens(tokens),
            Self::Path(path) => path.to_tokens(tokens),
            Self::Call(call) => call.to_tokens(tokens),
        }
    }
}
impl TryFrom<Expr> for AttrExpr {
    type Error = ParseError;

    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        match expr {
            Expr::Lit(lit) => Ok(Self::Lit(lit)),
            Expr::Path(path) => Ok(Self::Path(path)),
            Expr::Call(call) => Ok(Self::Call(call)),
            ref other => Err(ParseError::InvalidAttrExpr {
                //span: other.span(), expr: expr.clone()
                span: other.span(),
            }),
        }
    }
}
impl From<AttrExpr> for Expr {
    fn from(attr_expr: AttrExpr) -> Self {
        match attr_expr {
            AttrExpr::Lit(lit) => Self::Lit(lit),
            AttrExpr::Path(path) => Self::Path(path),
            AttrExpr::Call(call) => Self::Call(call),
        }
    }
}
