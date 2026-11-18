use crate::{
    generate::{
        BuilderStruct, ConfigStruct, util::FnProperties,
        plan::{
            Cause, DeriveCopy, DeriveDefault, FallibleBuild, FallibleFromOverride,
            InfallibleBuild, InfallibleFromOverride, Topic,
        },
        util::CanonPath,
    },
    parse::{
        attr::{
            AttrVariant, DefaultExpr, FieldAttr, GroupedAttrs, PassthroughAttrs,
            ProxiedAttr, StandardAttr, UnvalidatedGroupedAttrs, ValidateAttrs,
        },
        error::ParseError,
        IdentPool, util,
    },
};

use arbitrait::{
    Propose,
    ballot::unscored::MandateForbidBallot::{Abstain, Forbid, Mandate},
    vote::{CastVote, IdentifyVoter, Vote},
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::rc::Rc;
use std::sync::LazyLock;
use syn::{
    AngleBracketedGenericArguments, Error, Field, Ident, Meta,
    parse_quote, PathArguments, spanned::Spanned, Type, TypePath,
};

// !- Statics

pub(crate) static UNSET_ALIASES: LazyLock<&[&'static str; 4]> = LazyLock::new(|| &[
    "reset",
    "revert",
    "clear",
    "default",
]);

// !- Config fields

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OriginField {
    pub ident: Rc<Ident>,
    pub ty: Type,
    pub flat_ty: Type,
    pub is_option: bool,

    ids: Rc<IdentPool>,
    nested_ids: Option<Rc<IdentPool>>,

    /// `#[config(..)]` attrs
    pub attrs: FieldAttrs,

    /// passthrough attrs (doc, cfg, doc(cfg))
    pub passthrough_attrs: PassthroughAttrs,
}
impl OriginField {
    pub(crate) fn ident_upper_pascal(&self) -> Ident {
        let name = self.ident.to_string();
        let pascal_name = util::snake_case_to_upper_pascal(&name);
        format_ident!("{pascal_name}")
    }
    pub(crate) fn ids(&self) -> &IdentPool {
        &self.ids
    }
    pub(crate) fn ty_generics(&self) -> Option<&AngleBracketedGenericArguments> {
        if let Type::Path(ty) = &self.flat_ty
        && let Some(tail) = ty.path.segments.last()
        && let PathArguments::AngleBracketed(args) = &tail.arguments {
            Some(args)
        } else {
            None
        }
    }
    pub(crate) fn nested_ids(&self) -> Option<&IdentPool> {
        self.nested_ids.as_deref()
    }
    // ! Attributes

    pub(crate) fn passthrough_attrs(&self) -> &PassthroughAttrs {
        &self.passthrough_attrs
    }
    pub(crate) fn has_override_attrs(&self) -> bool {
        !self.attrs.override_attrs.is_empty()
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
        self.attrs.default.is_some()
    }
    pub(crate) fn default(&self) -> Option<&DefaultExpr> {
        self.attrs.default.as_ref()
    }
    pub(crate) fn is_nested(&self) -> bool {
        self.attrs.nested.is_some()
    }
    pub(crate) fn is_conditional(&self) -> bool {
        !self.passthrough_attrs.cfg.is_empty()
    }

    // ! Conditional impls

    /// Whether the override struct should have an unset field
    pub(crate) fn with_unset_field(&self) -> bool {
        self.is_optional()
        && !self.is_nested()
        && !self.attrs.skip_builder_setter
        && !self.attrs.skip_override_field
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

    // ! Nested helpers

    pub(crate) fn nested_config_error(&self) -> Option<Rc<TypePath>> {
        self.attrs.nested().and_then(|n| n.config_error.clone())
    }
    pub(crate) fn nested_has_config_error(&self) -> bool {
        self.attrs.nested().is_some_and(|n| n.config_error.is_some())
    }
    pub(crate) fn nested_has_fallible_build(&self) -> bool {
        self.attrs.nested().is_some_and(|n| n.build.is_fallible())
    }
    pub(crate) fn nested_has_fallible_from_override(&self) -> bool {
        self.attrs.nested().is_some_and(|n| n.from_override.is_fallible())
    }
    pub(crate) fn nested_build_fn_properties(&self) -> Option<FnProperties> {
        self.attrs.nested().map(|nested| {
            assert!(self.nested_ids.is_some());
            let nested_ty_path = self.nested_ids().unwrap().origin_path();
            let props = BuilderStruct::make_build_fn_properties(nested_ty_path);
            if nested.build.is_fallible() {
                props.into_fallible(nested.config_error.clone().unwrap())
            } else {
                props.into_infallible()
            }
        })
    }
    pub(crate) fn nested_error_variant_name(&self) -> Option<Ident> {
        self.nested_has_config_error().then(||
            format_ident!("{}Field", self.ident_upper_pascal())
        )
    }
    pub(crate) fn nested_map_err_expr(&self) -> Option<TokenStream> {
        self.nested_error_variant_name().map(|variant| {
            let error_path = self.ids().config_error_path();
            quote! {
                .map_err(|error| #error_path::#variant { error })?
            }
        })
    }
    pub(crate) fn nested_build_map_err_expr(&self) -> Option<TokenStream> {
        self.nested_has_fallible_build().then(|| self.nested_map_err_expr()).flatten()
    }
    pub(crate) fn nested_from_override_map_err_expr(&self) -> Option<TokenStream> {
        self.nested_has_fallible_from_override().then(|| self.nested_map_err_expr()).flatten()
    }

    /// Returns the field as a function param declaration
    ///
    /// e.g. `my_field: MyType<T>`
    pub(crate) fn as_fn_param_tokens(&self) -> TokenStream {
        let ident = &self.ident;
        let ty = &self.ty;

        quote!(#ident: #ty)
    }
    pub(crate) fn try_from_field(field: Field, ids: Rc<IdentPool>) -> Result<Self, Error> {
        let ident: Rc<Ident> = field.ident.unwrap().into(); // already checked fields are named

        let unwrapped_option = util::unwrap_option(&field.ty);
        let is_option = unwrapped_option.is_some();
        let flat_ty = unwrapped_option.unwrap_or(&field.ty).clone();

        // first attr pass, group into vecs by normal attrs or passthrough + validate
        let mut ctx = ValidationContext {
            is_option,
            flat_ty: flat_ty.clone(),
            nested_ident: None,
            nested_fallible_ident: None,
        };
        let grouped = GroupedAttrs::<FieldAttr>::try_from_attrs(field.attrs, &mut ctx)?;

        // parse #[config(..)] attrs into structured opts - 2 passes
        let attrs_in = FieldAttrsIn::try_from_grouped(grouped.standard, grouped.proxied, &ctx)?;
        let attrs = FieldAttrs::new(attrs_in, is_option);

        let nested_ids = attrs.nested().map(|nested| {
            let nested_ty = parse_quote!(#flat_ty);
            IdentPool::try_from_path(nested_ty, nested.config_error.as_ref())
        }).transpose()?.map(Rc::new);

        let parsed = OriginField {
            ident,
            flat_ty,
            ty: field.ty,
            is_option,

            ids,
            nested_ids,

            attrs,
            passthrough_attrs: grouped.passthrough,
        };

        Ok(parsed)
    }
}

// !- Origin field votes

impl IdentifyVoter for OriginField {
    fn voter_identity(&self) -> String {
        self.ident.to_string()
    }
}
impl CastVote<DeriveCopy> for OriginField {
    fn cast_ballot(&self) -> Vote<DeriveCopy> {
        let flag = self.attrs.copy;
        let evidence = DeriveCopy::new_evidence()
            .with_piece(Topic::Attr, Cause::from_flag_attr(flag, FieldAttr::Copy));
        self.build_vote(Some(flag).into()).with_evidence(evidence)
    }
}
impl CastVote<DeriveDefault> for OriginField {
    fn cast_ballot(&self) -> Vote<DeriveDefault> {
        let mut evidence = DeriveDefault::new_evidence();
        let res = if self.is_option {
            evidence.add_piece(Topic::Type, Cause::HasType("Option<_>"));
            Mandate
        } else if self.has_default() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent("default"));
            Mandate
        } else {
            evidence.add_piece(Topic::Attr, Cause::AttrMissing("default"));
            evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
            Forbid
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<InfallibleBuild> for OriginField {
    fn cast_ballot(&self) -> Vote<InfallibleBuild> {
        let mut evidence = InfallibleBuild::new_evidence();
        let res =
        if let Some(nested) = self.attrs.nested() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::Nested.into()));
            // nested build is fallible
            if nested.build.is_fallible() {
                evidence.add_piece(
                    Topic::Attr,
                    Cause::AttrPresent(FieldAttr::NestedTryBuild.into())
                );
                Forbid
            }
            // nested build is infallible
            else if self.is_option {
                evidence.add_piece(Topic::Type, Cause::HasType("Option<_>"));
                if nested.from_override.is_fallible() {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrPresent(FieldAttr::NestedTryFromOverride.into())
                    );
                    Forbid
                } else if nested.from_override.is_infallible() {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrPresent(FieldAttr::NestedFromOverride.into())
                    );
                    Mandate
                } else {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrMissing(FieldAttr::NestedNoFromOverride.into())
                    );
                    Forbid
                }
            } else {
                evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
                Mandate
            }
        } else {
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Nested.into()));
            Mandate
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<FallibleBuild> for OriginField {
    fn cast_ballot(&self) -> Vote<FallibleBuild> {
        let mut evidence = FallibleBuild::new_evidence();
        let res =
        if let Some(nested) = self.attrs.nested() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::Nested.into()));
            // nested build is fallible
            if nested.build.is_fallible() {
                evidence.add_piece(
                    Topic::Attr,
                    Cause::AttrPresent(FieldAttr::NestedTryBuild.into())
                );
                Mandate
            }
            // nested build is infallible
            else if self.is_option {
                evidence.add_piece(Topic::Type, Cause::HasType("Option<_>"));
                if nested.from_override.is_fallible() {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrPresent(FieldAttr::NestedTryFromOverride.into())
                    );
                    Mandate
                } else if nested.from_override.is_infallible() {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrPresent(FieldAttr::NestedFromOverride.into())
                    );
                    Abstain
                } else {
                    evidence.add_piece(
                        Topic::Attr,
                        Cause::AttrMissing(FieldAttr::NestedNoFromOverride.into())
                    );
                    Forbid
                }
            } else {
                evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
                Abstain
            }
        } else {
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Nested.into()));
            Abstain
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<InfallibleFromOverride> for OriginField {
    fn cast_ballot(&self) -> Vote<InfallibleFromOverride> {
        let mut evidence = InfallibleFromOverride::new_evidence();

        let infallible_build: Vote<InfallibleBuild> = self.cast_ballot();
        // always block if build/merge is fallible
        let res = if infallible_build.ballot().is_forbid() {
            // TODO: evidence
            Forbid
        } else if self.is_option {
            evidence.add_piece(Topic::Type, Cause::HasType("Option<_>"));
            Mandate
        } else if self.attrs.default.is_some() {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::Default.into()));
            Mandate
        } else if self.attrs.override_required {
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::OverrideRequired.into()));
            Mandate
        // }
        // // INCORRECT?
        //
        // else if let Some(nested) = self.attrs.nested() {
        //     if nested.from_override.is_infallible() {
        //         evidence.add_piece(
        //             Topic::Attr,
        //             Cause::AttrPresent(FieldAttr::NestedFromOverride.into())
        //         );
        //         Mandate
        //     } else {
        //         evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
        //         evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Default.into()));
        //         evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::OverrideRequired.into()));
        //         evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::Nested.into()));
        //         evidence.add_piece(
        //             Topic::Attr,
        //             Cause::AttrMissing(FieldAttr::NestedFromOverride.into())
        //         );
//
        //         Forbid
        //     }
        } else {
            evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Default.into()));
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::OverrideRequired.into()));
            // evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Nested.into()));
            Forbid
        };
        self.build_vote(res).with_evidence(evidence)
    }
}
impl CastVote<FallibleFromOverride> for OriginField {
    fn cast_ballot(&self) -> Vote<FallibleFromOverride> {
        let mut evidence = FallibleFromOverride::new_evidence();
        // TODO: forbid if nested && no nested from override variants
        // && no default
        let res =
        if !self.has_default() && self.attrs.skip_override_field {
            evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Default.into()));
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::OverrideSkip.into()));
            Forbid
        } else if let Some(nested) = self.attrs.nested() && nested.from_override.is_none() && !self.has_default() {
            evidence.add_piece(Topic::Type, Cause::NotType("Option<_>"));
            evidence.add_piece(Topic::Attr, Cause::AttrMissing(FieldAttr::Default.into()));
            evidence.add_piece(Topic::Attr, Cause::AttrPresent(FieldAttr::NestedNoFromOverride.into()));
            Forbid
        } else {
            let infallible: Vote<InfallibleFromOverride> = self.cast_ballot();
            if infallible.ballot().is_mandate() {
                Abstain
            } else {
                evidence.add_piece(Topic::Proposal, Cause::Forbid("impl From<Override>"));
                Mandate
            }
        };
        self.build_vote(res).with_evidence(evidence)
    }
}

struct ValidationContext {
    is_option: bool,
    flat_ty: Type,
    nested_ident: Option<Ident>,
    nested_fallible_ident: Option<Ident>,
}
impl ValidateAttrs<FieldAttr, ValidationContext> for UnvalidatedGroupedAttrs<FieldAttr> {
    fn validate_scoped(&self, context: &mut ValidationContext) -> Result<(), ParseError> {
        // validate option incompat
        if context.is_option {
            for attr in &self.grouped.standard {
                if attr.variant.is_option_incompatible() {
                    Err(ParseError::AttrOptionUnsupported { ident: attr.ident.clone() })?;
                }
            }
        }

        // hack: persist idents for nested validation
        context.nested_ident = self.seen.get(&FieldAttr::Nested).cloned();
        context.nested_fallible_ident = self.seen.get(&FieldAttr::NestedTryBuild).or_else(||
            self.seen.get(&FieldAttr::NestedTryFromOverride)
        ).cloned();

        Ok(())
    }
    fn validate_base(&self) -> Result<(), ParseError> {
        self.validate_common()
    }
    fn into_validated(self) -> GroupedAttrs<FieldAttr> {
        self.grouped
    }
}

// !- First pass field attrs

/// Field attrs during parsing / prior to validation
#[expect(clippy::struct_excessive_bools, reason = "parsing input flags permitted")]
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct FieldAttrsIn {
    pub copy: bool,

    pub default: Option<DefaultExpr>,

    skip_all: bool,
    config_skip_getter: bool,
    builder_skip: bool,
    override_skip: bool,

    pub override_required: bool,

    pub override_from: Option<TypePath>,
    pub override_from_copy: bool,
    pub override_via: Option<TypePath>,

    /// custom attrs to attach to override fields
    pub override_attrs: Vec<TokenStream>,

    pub nested: bool,
    pub nested_builder_default: bool,
    pub nested_pub_setter: bool,
    pub nested_config_error: Option<TypePath>,
    pub nested_from_override: bool,
    pub nested_try_from_override: bool,
    pub nested_no_from_override: bool,
    pub nested_build: bool,
    pub nested_try_build: bool,
}
impl FieldAttrsIn {
    #[allow(
        clippy::allow_attributes,
        clippy::match_single_binding,
        clippy::match_wildcard_for_single_variants,
        reason = "consistent formatting",
    )]
    fn try_from_grouped(
        standard: Vec<StandardAttr<FieldAttr>>,
        proxied: Vec<ProxiedAttr<FieldAttr>>,
        context: &ValidationContext,
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
                        FieldAttr::Nested => out.nested = true,
                        FieldAttr::NestedBuilderDefault => out.nested_builder_default = true,
                        FieldAttr::NestedPubSetter => out.nested_pub_setter = true,
                        FieldAttr::NestedBuild => out.nested_build = true,
                        FieldAttr::NestedTryBuild => out.nested_try_build = true,
                        FieldAttr::NestedFromOverride => out.nested_from_override = true,
                        FieldAttr::NestedTryFromOverride => out.nested_try_from_override = true,
                        FieldAttr::NestedNoFromOverride => out.nested_no_from_override = true,

                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                }
                Meta::List(_meta_list) => {
                    match attr.variant {
                        _unknown => unreachable!("shape validation covers unknown"),
                    }
                },
                Meta::NameValue(meta_name_value) => {
                    match attr.variant {
                        FieldAttr::Default => out.default = Some(meta_name_value.value.try_into()?),
                        FieldAttr::OverrideFrom => out.override_from = Some(syn::parse2(meta_name_value.value.to_token_stream())?),
                        FieldAttr::OverrideVia => out.override_via = Some(syn::parse2(meta_name_value.value.to_token_stream())?),
                        FieldAttr::NestedConfigError => out.nested_config_error = Some(syn::parse2(meta_name_value.value.to_token_stream())?),

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

        if out.nested {
            out.validate_nested(context)?;
        }

        Ok(out)
    }
    fn validate_nested(&self, ctx: &ValidationContext) -> Result<(), ParseError> {
        assert!(self.nested);
        assert!(ctx.nested_ident.is_some());
        let flat_ty = &ctx.flat_ty;
        let nested_attr_ident = ctx.nested_ident.as_ref().unwrap();
        let nested_ty = parse_quote!(#flat_ty);
        let nested_ids = IdentPool::try_from_path(nested_ty, self.nested_config_error.as_ref())?;

        if !self.nested_build && !self.nested_try_build {
            Err(ParseError::NestedMissingBuildVariant {
                ident: nested_attr_ident.clone(),
                builder_ident: nested_ids.builder_ident().clone(),
            })?;
        }

        if !self.nested_from_override &&
            !self.nested_try_from_override &&
            !self.nested_no_from_override
        {
            Err(ParseError::NestedMissingFromOverrideVariant {
                ident: nested_attr_ident.clone(),
                config_ident: nested_ids.origin_ident().clone(),
                override_ident: nested_ids.override_ident().clone(),
            })?;
        }

        if let Some(fallible_attr_ident) = ctx.nested_fallible_ident.clone() &&
            self.nested_config_error.is_none()
        {
            Err(ParseError::NestedFallibleMissingError {
                ident: nested_attr_ident.clone(),
                config_ident: nested_ids.origin_ident().clone(),
                fallible_attr_ident,
                fallible_item_path: if self.nested_try_build {
                    "nested(try_build)"
                } else {
                    "nested(try_from_override)"
                }.to_string(),
            })?;
        }

        if ctx.is_option && self.nested_no_from_override {
            Err(ParseError::NestedOptionNoFromOverride {
                field_type_span: ctx.flat_ty.span(),
                config_ident: nested_ids.origin_ident().clone(),
                override_ident: nested_ids.override_ident().clone(),
            })?;
        }
        Ok(())
    }
}

// ! Second pass field attrs

/// Field attrs post-validation with improved ergonomics
#[expect(clippy::struct_excessive_bools, reason = "bools are correct here")]
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct FieldAttrs {
    pub copy: bool,
    pub default: Option<DefaultExpr>,
    pub nested: Option<NestedAttrs>,

    pub skip_config_getter: bool,
    pub skip_builder_setter: bool,
    pub skip_override_field: bool,

    pub override_required: bool,
    pub override_from: Option<OverrideFromAttrs>,

    /// custom attrs to attach to override fields
    pub override_attrs: Vec<TokenStream>,
}
impl FieldAttrs {
    fn new(attrs: FieldAttrsIn, is_option: bool) -> Self {
        Self {
            copy: attrs.copy,

            // if option, infer default as None
            default: match &attrs.default {
                Some(def) => Some(def.clone()),
                None if is_option => Some(DefaultExpr::none_expr()),
                None => None,
            },

            nested: attrs.nested.then(|| {
                let config_error = attrs.nested_config_error.map(Rc::new);
                NestedAttrs {
                    builder_default: attrs.nested_builder_default,
                    pub_setter: attrs.nested_pub_setter,
                    config_error: config_error.clone(),
                    build: if attrs.nested_build {
                        BuildSupport::Infallible
                    } else {
                        BuildSupport::Fallible(config_error.clone().unwrap())
                    },
                    from_override: if attrs.nested_from_override {
                        FromOverrideSupport::Infallible
                    } else if attrs.nested_try_from_override {
                        FromOverrideSupport::Fallible
                    } else {
                        assert!(attrs.nested_no_from_override);
                        FromOverrideSupport::None
                    },
                }
            }),

            skip_config_getter: attrs.skip_all || attrs.config_skip_getter,
            skip_builder_setter: attrs.skip_all || attrs.builder_skip || attrs.override_required,
            skip_override_field: attrs.skip_all || attrs.override_skip,

            override_required: attrs.override_required,
            override_from: attrs.override_from.map(|ty|
                OverrideFromAttrs {
                    ty,
                    copy: attrs.override_from_copy,
                    via_ty: attrs.override_via,
                }
            ),
            override_attrs: attrs.override_attrs,
        }
    }
    pub(crate) fn override_from(&self) -> Option<&OverrideFromAttrs> {
        self.override_from.as_ref()
    }
    pub(crate) fn nested(&self) -> Option<&NestedAttrs> {
        self.nested.as_ref()
    }
}

// !- Field attr util types

// ! OverrideFrom attrs

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OverrideFromAttrs {
    pub ty: TypePath,
    pub copy: bool,
    pub via_ty: Option<TypePath>,
}

// ! NestedAttrs

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct NestedAttrs {
    pub builder_default: bool,
    pub pub_setter: bool,
    pub build: BuildSupport,
    pub from_override: FromOverrideSupport,
    pub config_error: Option<Rc<TypePath>>,
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) enum BuildSupport {
    Infallible,
    Fallible(Rc<TypePath>),
}
impl BuildSupport {
    pub(crate) fn is_infallible(&self) -> bool {
        matches!(self, &Self::Infallible)
    }
    pub(crate) fn is_fallible(&self) -> bool {
        matches!(self, &Self::Fallible(_))
    }
    pub(crate) fn as_error(&self) -> Option<Rc<TypePath>> {
        if let Self::Fallible(path) = self {
            Some(path.clone())
        } else {
            None
        }
    }
    pub(crate) fn build_trait_path(&self) -> CanonPath {
        if self.is_fallible() {
            CanonPath::TryBuildConfig
        } else {
            CanonPath::BuildConfig
        }
    }
    pub(crate) fn merge_trait_path(&self) -> CanonPath {
        if self.is_fallible() {
            CanonPath::TryMergeOverride
        } else {
            CanonPath::MergeOverride
        }
    }
    pub(crate) fn merge_properties(&self) -> FnProperties {
        ConfigStruct::make_merge_fn_properties().into_fallible_if_err(self.as_error())
    }
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) enum FromOverrideSupport {
    Infallible,
    Fallible,
    None,
}
impl FromOverrideSupport {
    pub(crate) fn is_infallible(&self) -> bool {
        matches!(self, &Self::Infallible)
    }
    pub(crate) fn is_fallible(&self) -> bool {
        matches!(self, &Self::Fallible)
    }
    pub(crate) fn is_none(&self) -> bool {
        matches!(self, &Self::None)
    }
}
