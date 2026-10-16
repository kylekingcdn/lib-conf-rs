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
    OverrideFromCopy,
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
    pub const OVERRIDE_FROM_COPY: &'static str = "override_from_copy";
    pub const OVERRIDE_VIA: &'static str = "override_via";
}

// ! Field attr trait impls

impl AttrVariant for FieldAttr {
    fn as_str(self) -> &'static str {
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
            Self::OverrideFromCopy => Self::OVERRIDE_FROM_COPY,
            Self::OverrideVia => Self::OVERRIDE_VIA,
        }
    }
    fn empty_set() -> &'static Vec<Self> {
        &FIELD_ATTR_EMPTY_VEC
    }
    fn proxied_set() -> &'static Vec<Self> {
        &FIELD_ATTR_PROXY
    }
    fn opt_incompatible_set() -> &'static Vec<Self> {
        &FIELD_ATTR_OPT_INCOMPAT
    }
    fn required_set() -> &'static Vec<Self> {
        &FIELD_ATTR_REQUIRED
    }
    fn dependency_map() -> &'static HashMap<Self, Vec<Self>> {
        &FIELD_ATTR_DEPS
    }
    fn incompatible_map() -> &'static HashMap<Self, Vec<Self>> {
        &FIELD_ATTR_MUT_EX
    }
    fn implied_map() -> &'static HashMap<Self, Vec<Self>> {
        &FIELD_ATTR_IMPLICIT
    }
    fn shapes_map() -> &'static HashMap<Self, Vec<Shape>> {
        &FIELD_ATTR_SHAPES
    }
}

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
            Self::OVERRIDE_FROM_COPY => Ok(Self::OverrideFromCopy),
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
    (FieldAttr::OverrideFromCopy, vec![FieldAttr::OverrideFrom]),
    (FieldAttr::OverrideVia,      vec![FieldAttr::OverrideFrom]),
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
        FieldAttr::OverrideFromCopy,
        FieldAttr::OverrideVia,
    ]),
    // any attributes incompatible with any of the 3 _skip attrs
    (FieldAttr::SkipAll, vec![
        FieldAttr::OverrideRequired,
        FieldAttr::OverrideFrom,
        FieldAttr::OverrideFromCopy,
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

static FIELD_ATTR_OPT_INCOMPAT: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
    FieldAttr::OverrideRequired,
].into());

static FIELD_ATTR_REQUIRED: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
].into());

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
    (FieldAttr::OverrideFromCopy,   vec![Shape::Flag]),
    (FieldAttr::OverrideVia,        vec![Shape::KeyValue]),
].into());

static FIELD_ATTR_PROXY: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
    FieldAttr::OverrideAttr,
].into());
