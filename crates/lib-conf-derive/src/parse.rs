pub(crate) mod attr;
pub(crate) mod error;
pub(crate) mod util;

use crate::{
    generate::{self, util::DerivedLink},
    parse::{
        attr::{
            AttrExpr, AttrVariant, FieldAttr, GroupedAttrs,
            PassthroughAttrs, ProxiedAttr, StandardAttr, StructAttr,
        },
        error::ParseError,
    },
};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use std::rc::Rc;
use std::sync::LazyLock;
use syn::{
    Attribute, Data, DeriveInput, Error, Field,
    Fields,Generics, Ident, Meta, Type, TypePath,
};

// !- Statics

static STRUCT_SUFFIXES: LazyLock<&[&'static str; 7]> = LazyLock::new(|| &[
    "Config",
    "Conf",
    "Options",
    "Opts",
    "Parameters",
    "Params",
    "Settings",
]);
static STRUCT_SUFFIX_CSV: LazyLock<String> = LazyLock::new(||
    STRUCT_SUFFIXES.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
);

#[cfg(feature = "serde")]
pub(crate) static UNSET_ALIASES: LazyLock<&[&'static str; 4]> = LazyLock::new(|| &[
    "reset",
    "revert",
    "clear",
    "default",
]);

// !- Source config struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginStruct {
    pub ty: Type,
    pub generics: Generics,

    pub fields: Vec<Rc<OriginField>>,

    pub ids: Rc<IdentPool>,

    /// `#[config(..)]` attrs
    pub attrs: OriginStructAttrs,

    /// (doc, cfg, doc(cfg)) attrs
    passthrough_attrs: PassthroughAttrs,
}
impl OriginStruct {
    pub(crate) fn ids(&self) -> &IdentPool {
        &self.ids
    }
    // ! Attributes
    pub(crate) fn _passthrough_attrs(&self) -> &PassthroughAttrs {
        &self.passthrough_attrs
    }
    pub(crate) fn cfg_tokens(&self) -> TokenStream2 {
        self.passthrough_attrs.cfg_tokens()
    }
    pub(crate) fn doc_cfg_tokens(&self) -> TokenStream2 {
        self.passthrough_attrs.doc_cfg_tokens()
    }
    pub(crate) fn _doc_attrs(&self) -> &Vec<Attribute> {
        &self.passthrough_attrs.doc
    }
    // ! Helpers
    pub(crate) fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_required())
    }
    #[allow(dead_code)] // only used w/ serde feature
    pub(crate) fn has_optional_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_optional())
    }
    pub(crate) fn has_generics(&self) -> bool {
        self.generics.type_params().next().is_some()
    }
    pub(crate) fn use_phantom_fields(&self) -> bool {
        self.has_generics()
    }
}
impl TryFrom<TokenStream> for OriginStruct {
    type Error = Error;

    fn try_from(input: TokenStream) -> Result<Self, Error> {
        // do essential shape validation first
        let derive_input: DeriveInput = syn::parse(input)?;
        let data_struct = match derive_input.data {
            Data::Struct(inner) => Ok(inner),
            Data::Enum(inner) => Err(ParseError::EnumUnsupported(inner.enum_token.span).error()),
            Data::Union(inner) => Err(ParseError::UnionUnsupported(inner.union_token.span).error()),
        }?;

        let fields = match data_struct.fields {
            Fields::Named(fields) => Ok(fields),
            Fields::Unnamed(fields) => Err(ParseError::UnnamedStructUnsupported(fields.paren_token.span.open())),
            Fields::Unit => Err(ParseError::UnitStructUnsupported(derive_input.ident.span())),
        }?;
        // done shape validation

        let ty = generate::util::build_type(&derive_input.ident, &derive_input.generics);
        let ids = Rc::new(IdentPool::try_new(derive_input.ident.into())?);
        let grouped = GroupedAttrs::<StructAttr>::try_from(derive_input.attrs)?;
        if grouped.standard.is_empty() {
            Err(ParseError::MissingRequiredAttrHelper { ident: (**ids.origin_ident()).clone() })?;
        }
        // parse #[config(..)] attrs into structured opts
        let attrs = OriginStructAttrs::try_from_grouped(grouped.standard, grouped.proxied)?;
        
        // parse fields
        let fields: Vec<OriginField> = fields.named.into_iter().map(|f|
            OriginField::try_from_field(f, ids.clone())
        ).collect::<Result<_,_>>()?;

        let parsed = Self {
            ty,
            generics: derive_input.generics,
            fields: fields.into_iter().map(Into::into).collect(),
            ids,
            attrs,
            passthrough_attrs: grouped.passthrough,
        };

        Ok(parsed)
    }
}

// ! Config struct attrs

#[allow(clippy::struct_excessive_bools)]
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct OriginStructAttrs {
    pub builder_derives: Vec<TokenStream2>,
    pub builder_attrs: Vec<TokenStream2>,

    pub override_derives: Vec<TokenStream2>,
    pub override_attrs: Vec<TokenStream2>,
    
    pub example_field_name: String,
}
impl OriginStructAttrs {
    pub(crate) fn has_builder_derives(&self) -> bool {
        !self.builder_derives.is_empty()
    }
    pub(crate) fn has_override_derives(&self) -> bool {
        !self.override_derives.is_empty()
    }
    pub(crate) fn has_builder_attrs(&self) -> bool {
        !self.builder_attrs.is_empty()
    }
    pub(crate) fn has_override_attrs(&self) -> bool {
        !self.override_attrs.is_empty()
    }
    fn try_from_grouped(
        standard: Vec<StandardAttr<StructAttr>>,
        proxied: Vec<ProxiedAttr<StructAttr>>,
    ) -> Result<Self, syn::Error> {
        let mut out = Self::default();
        
        for attr in standard {
            match attr.meta {
                Meta::Path(_path) => {
                    #[allow(clippy::match_single_binding)]
                    match attr.variant {
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
                Meta::List(_meta_list) => {
                    #[allow(clippy::match_single_binding)]
                    match attr.variant {
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                },
                Meta::NameValue(meta_name_value) => {
                    match attr.variant {
                        StructAttr::ExampleFieldName =>
                            out.example_field_name = AttrExpr::try_from(meta_name_value.value)?.require_str(&attr.ident)?,

                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
            }
        }
        
        for attr in proxied {
            #[allow(clippy::match_wildcard_for_single_variants)]
            match attr.variant {
                StructAttr::Attr => {
                    out.builder_attrs.push(attr.inner.clone());
                    out.override_attrs.push(attr.inner);
                }
                StructAttr::BuilderAttr => out.builder_attrs.push(attr.inner),
                StructAttr::OverrideAttr => out.override_attrs.push(attr.inner),
                StructAttr::Derive => {
                    out.builder_derives.push(attr.inner.clone());
                    out.override_derives.push(attr.inner);
                }
                StructAttr::BuilderDerive => out.builder_derives.push(attr.inner),
                StructAttr::OverrideDerive => out.override_derives.push(attr.inner),

                _unknown => unreachable!("shape validation covers unknown"),
            }
        }
        Ok(out)
    }
}

// !- Config fields

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginField {
    pub ident: Ident,
    pub ty: Type,
    pub flat_ty: Type,
    pub is_option: bool,
    pub default: Option<AttrExpr>,

    pub ids: Rc<IdentPool>,

    /// `#[config(..)]` attrs
    pub attrs: FieldAttrs,

    /// passthrough attrs (doc, cfg, doc(cfg))
    pub passthrough_attrs: PassthroughAttrs,
}
impl OriginField {
    pub(crate) fn ids(&self) -> &IdentPool {
        &self.ids
    }
    // ! Attributes
    pub(crate) fn passthrough_attrs(&self) -> &PassthroughAttrs {
        &self.passthrough_attrs
    }
    // ! Helpers
    /// Returns true if either `self.is_option` or `self.attrs.has_default()`
    pub(crate) fn is_optional(&self) -> bool {
        self.has_default()
    }
    pub(crate) fn is_required(&self) -> bool {
        !self.is_optional()
    }
    pub(crate) fn has_default(&self) -> bool {
        self.default.is_some()
    }
    pub(crate) fn default(&self) -> Option<&AttrExpr> {
        self.default.as_ref()
    }
    pub(crate) fn has_override_attrs(&self) -> bool {
        !self.attrs.override_attrs.is_empty()
    }
    
    pub(crate) fn is_conditional(&self) -> bool {
        !self.passthrough_attrs.cfg.is_empty()
    }
    
    /// Whether the override struct should have an unset field
    pub(crate) fn with_unset_field(&self) -> bool {
        self.is_optional() && !self.attrs.skip_override_field()
    }

    /// Returns the ident of the unset field, if one should be used
    pub(crate) fn unset_ident(&self) -> Option<Ident> {
        if self.with_unset_field() {
            let ident = &self.ident;
            Some(format_ident!("{ident}_unset"))
        } else {
            None
        }
    }

    /// Returns the ident of the unset field, if one should be used
    #[cfg(feature = "serde")]
    pub(crate) fn unset_aliases(&self) -> Vec<String> {
        if self.with_unset_field() {
            let ident = &self.ident;
            UNSET_ALIASES
                .iter()
                .map(|alias| format!("{ident}_{alias}"))
                .collect()
        } else {
            Vec::new()
        }
    }
    /// Returns the ident used for corresponding phantom field
    pub(crate) fn phantom_ident(&self) -> Ident {
        let ident = &self.ident;
        format_ident!("_{ident}")
    }
    /// Returns the field as a function param declaration
    ///
    /// e.g. `my_field: MyType<T>`
    pub(crate) fn as_fn_param_tokens(&self) -> TokenStream2 {
        let ident = &self.ident;
        let ty = &self.ty;

        quote!(#ident: #ty)
    }
    fn try_from_field(field: Field, ids: Rc<IdentPool>) -> Result<Self, Error> {
        let unwrapped_option = util::unwrap_option(&field.ty);
        let is_option = unwrapped_option.is_some();

        // first attr pass, group into vecs by normal attrs or passthrough
        let grouped = GroupedAttrs::<FieldAttr>::try_from(field.attrs)?;

        // validate Option<Field> compatibility
        if is_option {
            for attr in &grouped.standard {
                if attr.variant.is_option_incompatible() {
                    Err(ParseError::AttrOptionUnsupported { ident: attr.ident.clone() })?;
                }
            }
        }

        // parse #[config(..)] attrs into structured opts
        let attrs = FieldAttrs::try_from_grouped(grouped.standard, grouped.proxied)?;

        // store default expr for Option types
        let default = match &attrs.default {
            Some(def) => Some(def.clone()),
            None if is_option => Some(AttrExpr::none_expr()),
            None => None,
        };

        let parsed = OriginField {
            ident: field.ident.unwrap(), // already checked fields are named
            flat_ty: unwrapped_option.unwrap_or(&field.ty).clone(),
            ty: field.ty,
            is_option,
            default,

            ids,

            attrs,
            passthrough_attrs: grouped.passthrough,
        };

        Ok(parsed)
    }
}

// ! Config field attrs

#[allow(clippy::struct_excessive_bools)]
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct FieldAttrs {
    /// no caveats
    pub copy: bool,

    /// incompatible attrs:
    /// - `override_required`
    pub default: Option<AttrExpr>,

    /// implies:
    /// - `builder_skip`
    /// - `override_skip`
    /// - `config_skip_getter`
    ///
    /// incompatible (+above):
    /// - `override_required`
    /// - `override_from`
    /// - `override_via`
    pub skip_all: bool,

    /// incompatible:
    /// - `skip_all`
    pub config_skip_getter: bool,

    /// incompatible:
    /// - `skip_all`
    /// - `override_required`
    pub builder_skip: bool,

    /// incompatible:
    /// - `skip_all`
    /// - `override_required`
    /// - `override_from`
    /// - `override_via`
    pub override_skip: bool,

    /// implies:
    /// - `builder_skip`
    ///
    /// incompatible(+above):
    /// - default
    /// - `skip_all`
    /// - `override_skip`
    ///
    /// incompatible types:
    /// - Option<_>
    pub override_required: bool,

    /// incompatible:
    /// - `skip_all`
    /// - `override_skip`
    pub override_from: Option<TypePath>,
    
    pub override_from_copy: bool,

    /// requires:
    /// - `override_from`
    ///
    /// incompatible:
    /// - `skip_all`
    /// - `override_skip`
    pub override_via: Option<TypePath>,

    /// custom attrs to attach to override fields
    pub override_attrs: Vec<TokenStream2>,
}
impl FieldAttrs {
    pub(crate) fn skip_config_getter(&self) -> bool {
        self.skip_all || self.config_skip_getter
    }
    pub(crate) fn skip_builder_setter(&self) -> bool {
        self.skip_all || self.builder_skip || self.override_required
    }
    pub(crate) fn skip_override_field(&self) -> bool {
        self.skip_all || self.override_skip
    }

    fn try_from_grouped(
        standard: Vec<StandardAttr<FieldAttr>>,
        proxied: Vec<ProxiedAttr<FieldAttr>>,
    ) -> Result<Self, syn::Error> {
        let mut out = Self::default();
        for attr in standard {
            match attr.meta {
                Meta::Path(_path) => {
                    match attr.variant {
                        FieldAttr::Copy => out.copy = true,
                        FieldAttr::SkipAll => out.skip_all = true,
                        FieldAttr::ConfigSkipGetter => out.config_skip_getter = true,
                        FieldAttr::BuilderSkip => out.builder_skip = true,
                        FieldAttr::OverrideSkip => out.override_skip = true,
                        FieldAttr::OverrideFromCopy => out.override_from_copy = true,
                        FieldAttr::OverrideRequired => out.override_required = true,

                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
                Meta::List(_meta_list) => {
                    #[allow(clippy::match_single_binding)]
                    match attr.variant {
                        // FieldAttr::OverrideAttr => out.override_attrs.push(meta_list.tokens),

                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                },
                Meta::NameValue(meta_name_value) => {
                    match attr.variant {
                        FieldAttr::Default => out.default = Some(AttrExpr::try_from(meta_name_value.value)?),
                        FieldAttr::OverrideFrom => out.override_from = Some(syn::parse(meta_name_value.value.to_token_stream().into())?),
                        FieldAttr::OverrideVia => out.override_via = Some(syn::parse(meta_name_value.value.to_token_stream().into())?),

                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
            }
        }

        for attr in proxied {
            match attr.variant {
                FieldAttr::OverrideAttr => out.override_attrs.push(attr.inner),

                _unknown => unreachable!("shape validation covers unknown"),
            }
        }

        Ok(out)
    }
}

// !- Ident pool

/// Primary type idents, shared by all parse / generate structs to aid with doc refs
#[cfg_attr(feature = "syn-debug", derive(Debug))]
pub(crate) struct IdentPool {
    prefix: Rc<Ident>,
    suffix: Rc<Ident>,

    origin_ident: Rc<Ident>,
    builder_ident: Rc<Ident>,
    override_ident: Rc<Ident>,
}
#[allow(dead_code)]
impl IdentPool {
    pub(crate) fn try_new(origin_ident: Rc<Ident>) -> Result<Self, ParseError> {
        let (prefix, suffix) = Self::split_origin_ident(&origin_ident)?;

        Ok(Self {
            prefix: format_ident!("{prefix}").into(),
            suffix: format_ident!("{suffix}").into(),

            builder_ident: format_ident!("{origin_ident}Builder").into(),
            override_ident: format_ident!("{prefix}Override{suffix}").into(),
            origin_ident,
        })
    }
    fn extract_suffix(ident_str: &str) -> Option<&'static str> {
        for suffix in *STRUCT_SUFFIXES {
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
                suffixes: (*STRUCT_SUFFIX_CSV).clone(),
                span: ident.span(),
            })
        }
    }
    pub(crate) fn prefix(&self) -> &Rc<Ident> {
        &self.prefix
    }
    pub(crate) fn suffix(&self) -> &Rc<Ident> {
        &self.suffix
    }

    pub(crate) fn origin_ident(&self) -> &Rc<Ident> {
        &self.origin_ident
    }
    pub(crate) fn origin_link(&self) -> DerivedLink {
        DerivedLink::new(&self.origin_ident)
    }

    pub(crate) fn builder_ident(&self) -> &Rc<Ident> {
        &self.builder_ident
    }
    pub(crate) fn builder_link(&self) -> DerivedLink {
        DerivedLink::new(&self.builder_ident)
    }

    pub(crate) fn override_ident(&self) -> &Rc<Ident> {
        &self.override_ident
    }
    pub(crate) fn override_link(&self) -> DerivedLink {
        DerivedLink::new(&self.override_ident)
    }
}
