pub(crate) mod attr;
pub(crate) mod error;
pub(crate) mod origin_field;
pub(crate) mod origin_struct;
pub(crate) mod shared;
pub(crate) mod util;

pub(crate) use origin_field::{FieldAttrs, FromOverrideSupport, OriginField};
pub(crate) use origin_struct::OriginStruct;

use crate::{
    generate::{doc::DerivedLink, util::IntoTypePath},
    parse::error::ParseError,
};

use quote::format_ident;
use std::rc::Rc;
use syn::{Ident, Path, PathSegment};

// !- Ident pool

/// Primary type idents, shared by all parse / generate structs to aid with doc refs
#[cfg_attr(feature = "syn-debug", derive(Debug))]
pub(crate) struct IdentPool {
    #[cfg(feature = "serde")]
    prefix: Rc<Ident>,
    // suffix: Rc<Ident>,

    origin_ident: Rc<Ident>,
    builder_ident: Rc<Ident>,
    override_ident: Rc<Ident>,

    config_error_path: Path,

    leading_path: Option<Path>,
}
impl IdentPool {
    pub(crate) fn try_from_path(mut path: Path, config_error: Option<&impl IntoTypePath>) -> Result<Self, ParseError> {
        assert!(!path.segments.is_empty());
        let last_segment = path.segments.pop().unwrap();
        let ident = last_segment.ident; // FIXME: loss of path arguments

        let mut pool = Self::try_new(ident.into())?;
        pool.leading_path = Some(path);
        // override to provided config error if present
        if let Some(error) = config_error {
            let error = error.to_owned().into_type_path();
            assert!(!error.path.segments.is_empty());
            pool.config_error_path = error.path;
        }
        Ok(pool)
    }
    pub(crate) fn try_new(origin_ident: Rc<Ident>) -> Result<Self, ParseError> {
        let (prefix, suffix) = Self::split_origin_ident(&origin_ident)?;

        Ok(Self {
            #[cfg(feature = "serde")]
            prefix: format_ident!("{prefix}").into(),
            // suffix: format_ident!("{suffix}").into(),

            builder_ident: format_ident!("{origin_ident}Builder").into(),
            override_ident: format_ident!("{prefix}Override{suffix}").into(),

            config_error_path: format_ident!("{origin_ident}Error").into(),

            origin_ident,

            leading_path: None,
        })
    }
    fn extract_suffix(ident_str: &str) -> Option<&'static str> {
        for suffix in *origin_struct::STRUCT_SUFFIXES {
            if ident_str.ends_with(suffix) {
                return Some(suffix)
            }
        }
        None
    }
    fn split_origin_ident(ident: &Ident) -> Result<(String, String), ParseError> {
        let ident_str = ident.to_string();
        if let Some(suffix) = Self::extract_suffix(&ident_str) {
            let suf_index = ident_str.rfind(suffix).unwrap();
            let prefix = &ident_str[..suf_index];

            Ok((prefix.to_string(), suffix.to_string()))
        } else {
            Err(ParseError::UnknownStructSuffix {
                name: ident_str,
                suffixes: (*origin_struct::STRUCT_SUFFIX_CSV).clone(),
                span: ident.span(),
            })
        }
    }
    #[cfg(feature = "serde")]
    pub(crate) fn prefix(&self) -> &Rc<Ident> {
        &self.prefix
    }
    // pub(crate) fn suffix(&self) -> &Rc<Ident> {
    //     &self.suffix
    // }

    pub(crate) fn origin_ident(&self) -> &Rc<Ident> {
        &self.origin_ident
    }
    pub(crate) fn origin_path(&self) -> Path {
        self.build_path((*self.origin_ident).clone())
    }
    pub(crate) fn origin_link(&self) -> DerivedLink {
        DerivedLink::new(&self.origin_ident)
    }

    pub(crate) fn builder_ident(&self) -> &Rc<Ident> {
        &self.builder_ident
    }
    pub(crate) fn builder_path(&self) -> Path {
        self.build_path((*self.builder_ident).clone())
    }
    pub(crate) fn builder_link(&self) -> DerivedLink {
        DerivedLink::new(&self.builder_ident)
    }

    pub(crate) fn override_ident(&self) -> &Rc<Ident> {
        &self.override_ident
    }
    pub(crate) fn override_path(&self) -> Path {
        self.build_path((*self.override_ident).clone())
    }
    pub(crate) fn override_link(&self) -> DerivedLink {
        DerivedLink::new(&self.override_ident)
    }

    pub(crate) fn config_error_ident(&self) -> &Ident {
        let last_segment = self.config_error_path.segments.last();
        assert!(last_segment.is_some());

        &last_segment.unwrap().ident
    }
    pub(crate) fn config_error_path(&self) -> &Path {
        &self.config_error_path
    }
    // pub(crate) fn config_error_link(&self) -> DerivedLink {
    //     DerivedLink::new(self.config_error_path.clone())
    // }

    fn build_path(&self, ident: Ident) -> Path {
        let ident_segment = PathSegment::from(ident);
        if let Some(mut path) = self.leading_path.clone() {
            path.segments.push(ident_segment);
            path
        } else {
            ident_segment.into()
        }
    }
}
