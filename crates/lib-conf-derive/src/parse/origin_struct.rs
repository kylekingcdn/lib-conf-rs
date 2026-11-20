use crate::{
    generate::{
        self,
        doc::AppendDoc,
        plan::{
            Cause, DeriveCopy, DeriveDefault, FallibleBuild, FallibleFromOverride,
            InfallibleBuild, InfallibleFromOverride, Topic,
        },
    },
    parse::{
        attr::{
            CallExpr, GroupedAttrs, PassthroughAttrs, ProxiedAttr, StandardAttr,
            StrExpr, StructAttr, UnvalidatedGroupedAttrs, ValidateAttrs,
        },
        error::ParseError, IdentPool, OriginField, shared::SharedAttrs,
    },
};

use arbitrait::{
    Propose,
    ballot::unscored::MandateForbidBallot::{Abstain, Forbid, Mandate},
    vote::{CastVote, IdentifyVoter, Vote},
};
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::rc::Rc;
use std::sync::LazyLock;
use syn::{Data, DeriveInput, Error, Fields, Generics, Meta, Type, TypePath};

#[cfg(feature = "serde")]
use {
    crate::parse::util,
    quote::format_ident,
    syn::Ident,
};

// !- Statics

pub(crate) static STRUCT_SUFFIXES: LazyLock<&[&'static str; 7]> = LazyLock::new(|| &[
    "Config",
    "Conf",
    "Options",
    "Opts",
    "Parameters",
    "Params",
    "Settings",
]);
pub(crate) static STRUCT_SUFFIX_CSV: LazyLock<String> = LazyLock::new(||
    STRUCT_SUFFIXES.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
);

// !- Source config struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginStruct {
    input: DeriveInput,

    pub ty: Type,
    //pub generics: Generics,

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
    pub(crate) fn generics(&self) -> &Generics {
        &self.input.generics
    }
    pub(crate) fn into_input(self) -> DeriveInput {
        self.input
    }
    // ! Attributes
    #[cfg(feature = "serde")]
    pub(crate) fn example_field_name(&self) -> Ident {
        if let Some(name) = self.attrs.example_field_name() {
            format_ident!("{name}")
        } else {
            let prefix = self.ids.prefix().to_string();
            let name = util::upper_pascal_to_snake_case(&prefix);
            format_ident!("{name}")
        }
    }

    pub(crate) fn _passthrough_attrs(&self) -> &PassthroughAttrs {
        &self.passthrough_attrs
    }
    pub(crate) fn cfg_tokens(&self) -> TokenStream {
        self.passthrough_attrs.cfg_tokens()
    }
    pub(crate) fn doc_cfg_tokens(&self) -> TokenStream {
        self.passthrough_attrs.doc_cfg_tokens()
    }
    pub(crate) fn append_docs(&self) -> AppendDoc {
        self.passthrough_attrs.doc().clone().into()
    }
    // ! Helpers
    pub(crate) fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_required())
    }
    pub(crate) fn has_optional_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_optional())
    }
    pub(crate) fn has_generics(&self) -> bool {
        self.generics().type_params().next().is_some()
    }
    pub(crate) fn use_phantom_fields(&self) -> bool {
        self.has_generics()
    }
}

impl IdentifyVoter for OriginStruct {
    fn voter_identity(&self) -> String {
        self.ids.origin_ident().to_string()
    }
}
impl CastVote<DeriveDefault> for OriginStruct {
    fn cast_ballot(&self) -> Vote<DeriveDefault> {
        let mut evidence = DeriveDefault::new_evidence();
        let res = if self.fields.is_empty() {
            evidence.add_piece(Topic::Struct, Cause::NoFields);
            Mandate
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<DeriveCopy> for OriginStruct {
    fn cast_ballot(&self) -> Vote<DeriveCopy> {
        let mut evidence = DeriveCopy::new_evidence();
        let res = if self.attrs.no_autoderive {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(StructAttr::NoAutoderive.into()));
            Forbid
        } else if self.fields.is_empty() {
            evidence.add_piece(Topic::Struct, Cause::NoFields);
            Mandate
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<InfallibleBuild> for OriginStruct {
    fn cast_ballot(&self) -> Vote<InfallibleBuild> {
        let mut evidence = InfallibleBuild::new_evidence();
        let res =
        if self.attrs.validate_fn.is_some() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(StructAttr::Validate.into()));
            Forbid
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<FallibleBuild> for OriginStruct {
    fn cast_ballot(&self) -> Vote<FallibleBuild> {
        let mut evidence = FallibleBuild::new_evidence();
        let res =
        if self.attrs.validate_fn.is_some() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(StructAttr::Validate.into()));
            Mandate
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<InfallibleFromOverride> for OriginStruct {
    fn cast_ballot(&self) -> Vote<InfallibleFromOverride> {
        let mut evidence = InfallibleFromOverride::new_evidence();

        let fallible_build: Vote<FallibleBuild> = self.cast_ballot();

        // always block if build/merge is fallible
        let res =
        if fallible_build.ballot().is_mandate() {
            // TODO: evidence
            Forbid
        } else if self.attrs.validate_fn.is_some() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(StructAttr::Validate.into()));
            Forbid
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<FallibleFromOverride> for OriginStruct {
    fn cast_ballot(&self) -> Vote<FallibleFromOverride> {
        let mut evidence = FallibleFromOverride::new_evidence();
        let res =
        if self.attrs.validate_fn.is_some() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(StructAttr::Validate.into()));
            Mandate
        } else {
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}

impl TryFrom<TokenStream> for OriginStruct {
    type Error = Error;

    fn try_from(input: TokenStream) -> Result<Self, Error> {
        // do essential shape validation first
        let derive_input: DeriveInput = syn::parse2(input)?;
        let data_struct = match &derive_input.data {
            Data::Struct(inner) => Ok(inner.clone()),
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
        let ids = Rc::new(IdentPool::try_new(derive_input.ident.clone().into())?);

        // first attr pass, group into vecs by normal attrs or passthrough + validate
        let grouped = GroupedAttrs::<StructAttr>::try_from_attrs(derive_input.attrs.clone(), &mut ())?;

        // parse #[config(..)] attrs into structured opts
        let attrs = OriginStructAttrs::try_from_grouped(grouped.standard, grouped.proxied)?;

        let fields: Vec<OriginField> = fields.named.into_iter().map(|f|
            OriginField::try_from_field(f, ids.clone(), attrs.shared.clone())
        ).collect::<Result<_,_>>()?;

        let parsed = Self {
            input: derive_input,
            ty,
            fields: fields.into_iter().map(Into::into).collect(),
            ids,
            attrs,
            passthrough_attrs: grouped.passthrough,
        };

        Ok(parsed)
    }
}

impl ValidateAttrs<StructAttr, ()> for UnvalidatedGroupedAttrs<StructAttr> {
    fn validate_scoped(&self, _context: &mut ()) -> Result<(), ParseError> {
        Ok(())
    }
    fn validate_base(&self) -> Result<(), ParseError> {
        self.validate_common()
    }
    fn into_validated(self) -> GroupedAttrs<StructAttr> {
        self.grouped
    }
}

// ! First pass config struct attrs

/// Struct attrs
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct OriginStructAttrs {
    pub no_autoderive: bool,
    pub builder_derives: Vec<TokenStream>,
    pub builder_attrs: Vec<TokenStream>,

    pub override_derives: Vec<TokenStream>,
    pub override_attrs: Vec<TokenStream>,

    example_field_name: Option<String>,
    pub validate_fn: Option<CallExpr>,
    pub validate_error: Option<TypePath>,

    pub vote_debug: bool,
    pub lib_conf_attribution: bool,

    pub shared: SharedAttrs,
}
impl OriginStructAttrs {
    #[cfg(feature = "serde")]
    fn example_field_name(&self) -> Option<&str> {
        self.example_field_name.as_deref()
    }
    pub(crate) fn autoderive(&self) -> bool {
        !self.no_autoderive
    }
    pub(crate) fn has_builder_attrs(&self) -> bool {
        !self.builder_attrs.is_empty()
    }
    pub(crate) fn has_override_attrs(&self) -> bool {
        !self.override_attrs.is_empty()
    }

    #[allow(
        clippy::allow_attributes,
        clippy::match_single_binding,
        clippy::match_wildcard_for_single_variants,
        reason = "consistent formatting",
    )]
    fn try_from_grouped(
        standard: Vec<StandardAttr<StructAttr>>,
        proxied: Vec<ProxiedAttr<StructAttr>>,
    ) -> Result<Self, syn::Error> {
        let mut out = Self::default();

        for attr in standard {
            match attr.meta {
                Meta::Path(_mpath) => {
                    match attr.variant {
                        StructAttr::NoAutoderive => out.no_autoderive = true,
                        StructAttr::VoteDebug => out.vote_debug = true,
                        StructAttr::LibConfAttribution => out.lib_conf_attribution = true,
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
                Meta::List(_meta_list) => {
                    match attr.variant {
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
                Meta::NameValue(meta_name_value) => {
                    match attr.variant {
                        StructAttr::ExampleFieldName => out.example_field_name = Some(StrExpr::try_to_string(meta_name_value.value)?),
                        StructAttr::Validate => out.validate_fn = Some(meta_name_value.value.try_into()?),
                        StructAttr::ValidateError => out.validate_error = Some(syn::parse2(meta_name_value.value.to_token_stream())?),
                        StructAttr::TraitsCrate => out.shared.traits_path = Rc::new(syn::parse2(meta_name_value.value.to_token_stream())?),
                        StructAttr::StaticAssertionsCrate => out.shared.static_assertions_path = Rc::new(syn::parse2(meta_name_value.value.to_token_stream())?),
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
            }
        }

        for attr in proxied {
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
