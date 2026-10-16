use crate::{
    parse::{
        attr::{AttrVariant, Shape},
        error::ParseError,
    },
};

use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::sync::LazyLock;
use syn::Ident;

// !- Struct attr

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) enum StructAttr {
    Derive,
    BuilderDerive,
    OverrideDerive,

    Attr,
    BuilderAttr,
    OverrideAttr,
    
    ExampleFieldName,
}
impl StructAttr {
    pub const DERIVE: &str = "derive";
    pub const BUILDER_DERIVE: &'static str = "builder_derive";
    pub const OVERRIDE_DERIVE: &'static str = "override_derive";
    pub const ATTR: &'static str = "attr";
    pub const BUILDER_ATTR: &'static str = "builder_attr";
    pub const OVERRIDE_ATTR: &'static str = "override_attr";
    pub const EXAMPLE_FIELD_NAME: &str = "example_field_name";
}

// ! Struct attr trait impls

impl AttrVariant for StructAttr {
    fn as_str(self) -> &'static str {
        match self {
            Self::Derive => Self::DERIVE,
            Self::BuilderDerive => Self::BUILDER_DERIVE,
            Self::OverrideDerive => Self::OVERRIDE_DERIVE,
            Self::Attr => Self::ATTR,
            Self::BuilderAttr => Self::BUILDER_ATTR,
            Self::OverrideAttr => Self::OVERRIDE_ATTR,
            Self::ExampleFieldName => Self::EXAMPLE_FIELD_NAME,
        }
    }
    fn empty_set() -> &'static Vec<Self> {
        &STRUCT_ATTR_EMPTY_VEC
    }
    fn proxied_set() -> &'static Vec<Self> {
        &STRUCT_ATTR_PROXY
    }
    fn required_set() -> &'static Vec<Self> {
        &STRUCT_ATTR_REQUIRED
    }
    fn opt_incompatible_set() -> &'static Vec<Self> {
        &STRUCT_ATTR_OPT_INCOMPAT
    }
    fn dependency_map() -> &'static HashMap<Self, Vec<Self>> {
        &STRUCT_ATTR_DEPS
    }
    fn incompatible_map() -> &'static HashMap<Self, Vec<Self>> {
        &STRUCT_ATTR_MUT_EX
    }
    fn implied_map() -> &'static HashMap<Self, Vec<Self>> {
        &STRUCT_ATTR_IMPLICIT
    }
    fn shapes_map() -> &'static HashMap<Self, Vec<Shape>> {
        &STRUCT_ATTR_SHAPES
    }
}
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
            Self::EXAMPLE_FIELD_NAME => Ok(Self::ExampleFieldName),
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
    (StructAttr::ExampleFieldName,  vec![Shape::KeyValue]),
].into());

/// Attributes that depend on other attrs being present
pub static STRUCT_ATTR_DEPS: LazyLock<HashMap<StructAttr, Vec<StructAttr>>> = LazyLock::new(|| [
].into());

/// (Associated) attributes that can't be used simultaneously
/// (AKA mutually exclusive)
///
/// NOTE: only one side of the exclusivity is required
pub static STRUCT_ATTR_MUT_EX: LazyLock<HashMap<StructAttr, Vec<StructAttr>>> = LazyLock::new(|| [
].into());

pub static STRUCT_ATTR_IMPLICIT: LazyLock<HashMap<StructAttr, Vec<StructAttr>>> = LazyLock::new(|| [
].into());

static STRUCT_ATTR_EMPTY_VEC: LazyLock<Vec<StructAttr>> = LazyLock::new(|| [
].into());

static STRUCT_ATTR_OPT_INCOMPAT: LazyLock<Vec<StructAttr>> = LazyLock::new(|| [
].into());

static STRUCT_ATTR_REQUIRED: LazyLock<Vec<StructAttr>> = LazyLock::new(|| [
    StructAttr::ExampleFieldName,
].into());

static STRUCT_ATTR_PROXY: LazyLock<Vec<StructAttr>> = LazyLock::new(|| [
    StructAttr::Derive,
    StructAttr::BuilderDerive,
    StructAttr::OverrideDerive,
    StructAttr::Attr,
    StructAttr::BuilderAttr,
    StructAttr::OverrideAttr,
].into());
