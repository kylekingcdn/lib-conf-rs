pub mod attr;
pub mod error;
mod util;

use crate::{
    generate,
    parse::{
        attr::{AttrExpr, FieldAttr},
        error::ParseError,
    },
};

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::LazyLock;
use syn::{
    Attribute, Data, DeriveInput, Error, Field, Fields, Generics, Ident, Meta,
    Token, Type, TypePath, parse_quote, punctuated::Punctuated, token
};

// !- Statics

static STRUCT_SUFFIXES: LazyLock<&[&'static str; 6]> = LazyLock::new(|| &[
    "Config",
    "Conf",
    "Options",
    "Opts",
    "Parameters",
    "Params",
]);
static STRUCT_SUFFIX_CSV: LazyLock<String> = LazyLock::new(||
    STRUCT_SUFFIXES.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
);

// !- Source config struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginStruct {
    pub ident: Ident,
    pub ty: Type,
    pub suffix: &'static str,
    pub fields: Vec<Rc<OriginField>>,
    pub doc_attrs: Vec<Attribute>,
    pub generics: Generics,
}
impl OriginStruct {
    pub fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_required())
    }
    pub fn required_fields(&self) -> Vec<Rc<OriginField>> {
        self.fields.iter().filter(|f| f.is_required()).cloned().collect()
    }
    pub fn optional_fields(&self) -> Vec<Rc<OriginField>> {
        self.fields.iter().filter(|f| f.is_optional()).cloned().collect()
    }
    pub fn has_generics(&self) -> bool {
        self.generics.type_params().next().is_some()
    }
    pub fn use_phantom_fields(&self) -> bool {
        self.has_generics()
    }
    fn resolve_suffix(ident: &Ident) -> Result<&'static str, ParseError> {
        let ident_str = ident.to_string();
        for suffix in *STRUCT_SUFFIXES {
            if ident_str.ends_with(suffix) {
                return Ok(suffix)
            }
        }
        Err(ParseError::UnknownStructSuffix {
            name: ident_str,
            suffixes: (*STRUCT_SUFFIX_CSV).clone(),
            span: ident.span(),
        })
    }
}
impl TryFrom<TokenStream> for OriginStruct {
    type Error = Error;

    fn try_from(input: TokenStream) -> Result<Self, Error> {
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

        let mut doc_attrs = Vec::new();
        for attr in &derive_input.attrs {
            // Err(error) for invalid #[doc...] attrs
            // Ok(Some(attr) for valid #[doc...] attrs
            // Ok(None for non-`doc` attrs
            if let Some(doc_attr) = util::try_parse_doc_attrs(attr)? {
                doc_attrs.push(doc_attr);
            }
        }
        let fields: Vec<OriginField> = fields.named.into_iter().map(OriginField::try_from).collect::<Result<_,_>>()?;

        let ty = generate::util::build_type(&derive_input.ident, &derive_input.generics);

        let parsed = Self {
            suffix: Self::resolve_suffix(&derive_input.ident)?,
            ident: derive_input.ident,
            ty,
            fields: fields.into_iter().map(Into::into).collect(),
            doc_attrs,
            generics: derive_input.generics,
        };

        Ok(parsed)
    }
}

// !- Config struct fields

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginField {
    pub ident: Ident,
    pub ty: Type,
    pub flat_ty: Type,
    pub is_option: bool,
    pub default: Option<AttrExpr>,
    pub attrs: FieldAttrs,
    pub doc_attrs: Vec<Attribute>,
}
impl OriginField {
    /// Returns true if either `self.is_option` or `self.attrs.has_default()`
    pub fn is_optional(&self) -> bool {
        self.has_default()
    }
    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }
    pub fn has_default(&self) -> bool {
        self.default.is_some()
    }
    pub fn default(&self) -> Option<&AttrExpr> {
        self.default.as_ref()
    }
    /// Whether the override struct should have an unset field
    pub fn with_unset_field(&self) -> bool {
        self.is_optional() && !self.attrs.skip_override_field()
    }
    /// Returns the ident of the unset field, if one should be used
    pub fn unset_ident(&self) -> Option<Ident> {
        if self.with_unset_field() {
            let ident = &self.ident;
            Some(format_ident!("{ident}_unset"))
        } else {
            None
        }
    }
    /// Returns the ident used for corresponding phantom field
    pub fn phantom_ident(&self) -> Ident {
        let ident = &self.ident;
        format_ident!("_{ident}")
    }
    /// Returns the field as a function param declaration
    ///
    /// e.g. `my_field: MyType<T>`
    pub fn as_fn_param_tokens(&self) -> TokenStream2 {
        let ident = &self.ident;
        let ty = &self.ty;
        
        quote!(#ident: #ty)
    }
    pub fn as_required_type(&self) -> Type {
        self.flat_ty.clone()
    }
    /// If `self.is_option`, returns the original type, otherwise wraps the type in `Option<_>(_)`
    pub fn as_optional_type(&self) -> Type {
        if self.is_option {
            self.ty.clone()
        } else {
            let ty = &self.ty;
            parse_quote!(Option::<#ty>)
        }
    }
    /// Same as `as_required_type`, except prefixes `&` if the copy flag wasn't provided
    pub fn as_required_return_type(&self) -> Type {
        let ty = self.as_required_type();
        Self::build_return_type(ty, self.attrs.copy)
    }
    /// Same as `as_optional_type`, except prefixes `&` if the copy flag wasn't provided
    pub fn as_optional_return_type(&self) -> Type {
        let ty = self.as_optional_type();
        Self::build_return_type(ty, self.attrs.copy)
    }
    pub fn build_return_type(base_ty: Type, supports_copy: bool) -> Type {
        if supports_copy {
            base_ty
        } else {
            parse_quote!(&#base_ty)
        }
    }
}
impl TryFrom<Field> for OriginField {
    type Error = Error;

    fn try_from(field: Field) -> Result<Self, Self::Error> {
        let unwrapped_option = util::unwrap_option(&field.ty);
        let mut parsed = OriginField {
            ident: field.ident.unwrap(), // already checked fields are named
            is_option: unwrapped_option.is_some(),
            default: None,
            flat_ty: unwrapped_option.unwrap_or(&field.ty).clone(),
            ty: field.ty,
            attrs: FieldAttrs::default(),
            doc_attrs: Vec::new(),
        };

        let mut seen = Vec::new();
        for attr in &field.attrs {
            if attr.path().is_ident(attr::HELPER_ATTR_CONFIG) {
                let ident = attr.path().require_ident().unwrap(); // already checked is ident
                if seen.contains(ident) {
                    Err(ParseError::DuplicateAttr { ident: ident.clone() })?;
                }

                let scope_attrs = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                parsed.attrs = FieldAttrs::try_from(scope_attrs)?;
                seen.push(ident.clone());
            }
            else {
                // Err(error) for invalid #[doc...] attrs
                // Ok(Some(attr) for valid #[doc...] attrs
                // Ok(None for non-`doc` attrs
                if let Some(doc_attr) = util::try_parse_doc_attrs(attr)? {
                    parsed.doc_attrs.push(doc_attr);
                }
                // unknown attr that we were able to get a key of
                else if let Some(ident) = attr.path().get_ident() {
                    eprintln!("Ignoring unknown attribute: {ident}");
                }
            }
        }
        parsed.default = match &parsed.attrs.default {
            Some(def) => Some(def.clone()),
            None if parsed.is_option => Some(AttrExpr::none_expr()),
            None => None,
        };
        Ok(parsed)
    }
}

// !- Field attributes

#[allow(clippy::struct_excessive_bools)]
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct FieldAttrs {
    pub copy: bool, // default: false
    pub default: Option<AttrExpr>, // default: None
    pub skip_all: bool, // default: false

    pub config_skip_getter: bool, // default: false

    pub builder_skip: bool, // default: false

    pub override_skip: bool, // default: false
    pub override_required: bool, // default: false
    pub override_from: Option<TypePath>, // default: None
    pub override_via: Option<TypePath>, // default: None
}
impl FieldAttrs {
    pub fn skip_config_getter(&self) -> bool {
        self.skip_all || self.config_skip_getter
    }
    pub fn skip_builder_setter(&self) -> bool {
        self.skip_all || self.builder_skip
    }
    pub fn skip_override_field(&self) -> bool {
        self.skip_all || self.override_skip
    }
    pub fn has_mapped_type(&self) -> bool {
        self.override_from.is_some()
    }
    pub fn has_intermediate_type(&self) -> bool {
        self.override_via.is_some()
    }
}
impl TryFrom<Punctuated<Meta, token::Comma>> for FieldAttrs {
    type Error = syn::Error;

    fn try_from(items: Punctuated<Meta, token::Comma>) -> Result<Self, Self::Error> {
        let mut seen = HashMap::<FieldAttr, Ident>::new();
        let mut out = Self::default();
        for item in &items {
            let (field_attr, ident) = match item {
                Meta::Path(path) => {
                    let ident = path.require_ident()?;
                    let field_attr = FieldAttr::try_from(ident)?;
                    field_attr.validate_shape(attr::Shape::Flag, ident)?;

                    match field_attr {
                        FieldAttr::Copy => out.copy = true,
                        FieldAttr::SkipAll => out.skip_all = true,
                        FieldAttr::ConfigSkipGetter => out.config_skip_getter = true,
                        FieldAttr::BuilderSkip => out.builder_skip = true,
                        FieldAttr::OverrideSkip => out.override_skip = true,
                        FieldAttr::OverrideRequired => out.override_required = true,
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                    (field_attr, ident.clone())
                }
                Meta::List(meta_list) => {
                    let ident = meta_list.path.require_ident()?;
                    let field_attr = FieldAttr::try_from(ident)?;
                    field_attr.validate_shape(attr::Shape::List, ident)?;

                    #[allow(clippy::match_single_binding)]
                    match field_attr {
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                },
                Meta::NameValue(meta_name_value) => {
                    let ident = meta_name_value.path.require_ident()?;
                    let field_attr = FieldAttr::try_from(ident)?;
                    field_attr.validate_shape(attr::Shape::KeyValue, ident)?;

                    match field_attr {
                        FieldAttr::Default => out.default = Some(AttrExpr::try_from(meta_name_value.value.clone())?),
                        FieldAttr::OverrideFrom => out.override_from = Some(syn::parse(meta_name_value.value.to_token_stream().into())?),
                        FieldAttr::OverrideVia => out.override_via = Some(syn::parse(meta_name_value.value.to_token_stream().into())?),
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                    (field_attr, ident.clone())
                }
            };
            // avoid redundant calls in each shape
            attr::validate_unique(field_attr, &seen)?;
            seen.insert(field_attr, ident);
        }
        // error on incompatible attrs
        attr::validate_mutex(&seen)?;
        // validate deps once all attributes are parsed
        attr::validate_deps(&seen)?;
        Ok(out)
    }
}
