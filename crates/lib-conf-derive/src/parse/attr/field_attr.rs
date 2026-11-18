use crate::parse::{
    attr::{AttrVariant, Shape},
    error::ParseError,
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
    OverrideSkip,

    OverrideAttr,
    OverrideRequired,
    OverrideFrom,
    OverrideFromCopy,
    OverrideVia,

    Nested,
    NestedBuilderDefault,
    NestedPubSetter,
    NestedConfigError,
    NestedBuild,
    NestedTryBuild,
    NestedFromOverride,
    NestedTryFromOverride,
    NestedNoFromOverride
}
impl FieldAttr {
    pub(crate) const COPY: &'static str = "copy";
    pub(crate) const DEFAULT: &'static str = "default";

    pub(crate) const SKIP_ALL: &'static str = "skip_all";
    pub(crate) const CONFIG_SKIP_GETTER: &'static str = "config_skip_getter";
    pub(crate) const BUILDER_SKIP: &'static str = "builder_skip";
    pub(crate) const OVERRIDE_SKIP: &'static str = "override_skip";

    pub(crate) const OVERRIDE_ATTR: &'static str = "override_attr";
    pub(crate) const OVERRIDE_REQUIRED: &'static str = "override_required";
    pub(crate) const OVERRIDE_FROM: &'static str = "override_from";
    pub(crate) const OVERRIDE_FROM_COPY: &'static str = "override_from_copy";
    pub(crate) const OVERRIDE_VIA: &'static str = "override_via";

    pub(crate) const NESTED: &'static str = "nested";
    pub(crate) const PUB_SETTER: &'static str = "pub_setter";
    pub(crate) const BUILDER_DEFAULT: &'static str = "builder_default";
    pub(crate) const CONFIG_ERROR: &'static str = "config_error";
    pub(crate) const BUILD: &'static str = "build";
    pub(crate) const TRY_BUILD: &'static str = "try_build";
    pub(crate) const FROM_OVERRIDE: &'static str = "from_override";
    pub(crate) const TRY_FROM_OVERRIDE: &'static str = "try_from_override";
    pub(crate) const NO_FROM_OVERRIDE: &'static str = "no_from_override";
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
            Self::Nested => Self::NESTED,
            Self::NestedBuilderDefault => "nested(builder_default)",
            Self::NestedPubSetter => "nested(pub_setter)",
            Self::NestedConfigError => "nested(config_error)",
            Self::NestedBuild => "nested(build)",
            Self::NestedTryBuild => "nested(try_build)",
            Self::NestedFromOverride => "nested(from_override)",
            Self::NestedTryFromOverride => "nested(try_from_override)",
            Self::NestedNoFromOverride => "nested(no_from_override)",
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
    // fn exclusive_dependency_map() -> &'static HashMap<Self, Vec<Self>> {
    //     &FIELD_ATTR_EXCLUSIVE_DEPS
    // }
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
impl TryFrom<(Option<FieldAttr>, &Ident)> for FieldAttr {
    type Error = ParseError;

    fn try_from((parent, ident): (Option<FieldAttr>, &Ident)) -> Result<Self, Self::Error> {
        let ident_name = ident.to_string();
        match parent {
            None => match ident_name.as_str() {
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
                Self::NESTED => Ok(Self::Nested),
                _unknown => Err(ParseError::UnknownAttr { ident: ident.clone() }),
            },
            Some(Self::Nested) => match ident_name.as_str() {
                Self::BUILDER_DEFAULT => Ok(Self::NestedBuilderDefault),
                Self::PUB_SETTER => Ok(Self::NestedPubSetter),
                Self::CONFIG_ERROR => Ok(Self::NestedConfigError),
                Self::BUILD => Ok(Self::NestedBuild),
                Self::TRY_BUILD => Ok(Self::NestedTryBuild),
                Self::FROM_OVERRIDE => Ok(Self::NestedFromOverride),
                Self::TRY_FROM_OVERRIDE => Ok(Self::NestedTryFromOverride),
                Self::NO_FROM_OVERRIDE => Ok(Self::NestedNoFromOverride),
                _unknown => Err(ParseError::UnknownAttr { ident: ident.clone() }),
            },
            _unknown => Err(ParseError::UnknownAttr { ident: ident.clone() }),
        }
    }
}
impl TryFrom<(Option<FieldAttr>, Ident)> for FieldAttr {
    type Error = ParseError;

    fn try_from((parent, ident): (Option<FieldAttr>, Ident)) -> Result<Self, Self::Error> {
        (parent, &ident).try_into()
    }
}

// ! Field attr statics

/// Attributes that depend on other attrs being present
pub(super) static FIELD_ATTR_DEPS: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
    (FieldAttr::OverrideFromCopy,       vec![FieldAttr::OverrideFrom]),
    (FieldAttr::OverrideVia,            vec![FieldAttr::OverrideFrom]),
    (FieldAttr::NestedTryBuild,         vec![FieldAttr::NestedConfigError]),
    (FieldAttr::NestedTryFromOverride,  vec![FieldAttr::NestedConfigError]),
].into());

// /// Dependencies where if the first is present, exactly 1 from the associated list must also be present
// pub(super) static FIELD_ATTR_EXCLUSIVE_DEPS: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
//     (FieldAttr::Nested,                 vec![FieldAttr::NestedBuild, FieldAttr::NestedTryBuild]),
// ].into());

/// (Associated) attributes that can't be used simultaneously
/// (AKA mutually exclusive)
///
/// NOTE: only one side of the exclusivity is required
pub(super) static FIELD_ATTR_MUT_EX: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
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
        FieldAttr::Nested,
    ]),
    (FieldAttr::OverrideAttr, vec![
        FieldAttr::SkipAll,
        FieldAttr::OverrideSkip,
    ]),
    (FieldAttr::Nested, vec![
        FieldAttr::OverrideFrom,
        FieldAttr::OverrideFromCopy,
        FieldAttr::OverrideVia,
        FieldAttr::BuilderSkip,
        // FieldAttr::Default,
    ]),
    
    // nested validation handles these, covered for posterity / additional future usage of mut ex
    (FieldAttr::NestedBuild, vec![
        FieldAttr::NestedTryBuild,
    ]),
    (FieldAttr::NestedFromOverride, vec![
        FieldAttr::NestedTryFromOverride,
        FieldAttr::NestedNoFromOverride,
    ]),
    (FieldAttr::NestedTryFromOverride, vec![
        FieldAttr::NestedNoFromOverride,
    ]),
].into());

pub(super) static FIELD_ATTR_IMPLICIT: LazyLock<HashMap<FieldAttr, Vec<FieldAttr>>> = LazyLock::new(|| [
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
    FieldAttr::Default,
    FieldAttr::OverrideRequired,
].into());

static FIELD_ATTR_REQUIRED: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
].into());

pub(super) static FIELD_ATTR_SHAPES: LazyLock<HashMap<FieldAttr, Vec<Shape>>> = LazyLock::new(|| [
    (FieldAttr::Copy,                   vec![Shape::Flag]),
    (FieldAttr::Default,                vec![Shape::KeyValue]),
    (FieldAttr::SkipAll,                vec![Shape::Flag]),
    (FieldAttr::ConfigSkipGetter,       vec![Shape::Flag]),
    (FieldAttr::BuilderSkip,            vec![Shape::Flag]),
    (FieldAttr::OverrideAttr,           vec![Shape::List]),
    (FieldAttr::OverrideSkip,           vec![Shape::Flag]),
    (FieldAttr::OverrideRequired,       vec![Shape::Flag]),
    (FieldAttr::OverrideFrom,           vec![Shape::KeyValue]),
    (FieldAttr::OverrideFromCopy,       vec![Shape::Flag]),
    (FieldAttr::OverrideVia,            vec![Shape::KeyValue]),
    (FieldAttr::Nested,                 vec![Shape::List]),
    (FieldAttr::NestedBuilderDefault, vec![Shape::Flag]),
    (FieldAttr::NestedPubSetter,        vec![Shape::Flag]),
    (FieldAttr::NestedConfigError,      vec![Shape::KeyValue]),
    (FieldAttr::NestedFromOverride,     vec![Shape::Flag]),
    (FieldAttr::NestedBuild,            vec![Shape::Flag]),
    (FieldAttr::NestedTryBuild,         vec![Shape::Flag]),
    (FieldAttr::NestedFromOverride,     vec![Shape::Flag]),
    (FieldAttr::NestedTryFromOverride,  vec![Shape::Flag]),
    (FieldAttr::NestedNoFromOverride,   vec![Shape::Flag]),
].into());

static FIELD_ATTR_PROXY: LazyLock<Vec<FieldAttr>> = LazyLock::new(|| [
    FieldAttr::OverrideAttr,
].into());
