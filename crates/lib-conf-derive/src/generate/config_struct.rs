use crate::{
    generate::{
        BuilderStruct, OverrideStruct, VariantField,
        doc::{AppendDoc, Doc, Document, Link},
        plan::{
            ConfigConsensus, ConfigPlan, DeriveCopy, DeriveDefault,
            FallibleBuild, FallibleFromOverride, FallibleMerge,
            InfallibleBuild, InfallibleFromOverride, InfallibleMerge,
        },
        util::{self, CanonPath, FnProperties, IntoTypePath},
    },
    parse::{self, FromOverrideSupport, IdentPool, OriginField, OriginStruct},
};

use arbitrait::{
    IterVoteCollector,
    vote::{CastVote, CollectVotes, IdentifyVoter, Vote},
};
use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::rc::Rc;
use std::sync::OnceLock;
use syn::{Ident, parse_quote, Type, TypePath};

#[derive(Debug, Copy, Clone)]
pub(crate) struct ConfigVariant;

// !- Config struct

#[derive(Clone)]
pub(crate) struct ConfigStruct {
    fields: Vec<ConfigField>,

    origin: Rc<OriginStruct>,
    override_struct: Rc<OverrideStruct>,
    builder_struct: Rc<BuilderStruct>,

    consensus: Rc <OnceLock<ConfigConsensus>>,
    plan: Rc<OnceLock<ConfigPlan>>,
}
impl ConfigStruct {
    pub(crate) fn new(origin: Rc<OriginStruct>, override_struct: Rc<OverrideStruct>, builder_struct: Rc<BuilderStruct>) -> Self {
        let fields = origin.fields
            .iter().cloned()
            .map(ConfigField::new)
            .collect();
        Self {
            fields,

            origin,
            override_struct,
            builder_struct,

            consensus: Rc::new(OnceLock::new()),
            plan: Rc::new(OnceLock::new()),
        }
    }
    pub(crate) fn ids(&self) -> &IdentPool {
        self.origin.ids()
    }
    pub(crate) fn ident(&self) -> &Rc<Ident> {
        self.ids().origin_ident()
    }
    pub(crate) fn merge_trait_path(&self) -> CanonPath {
        if self.plan().fallible_merge {
            CanonPath::TryMergeOverride
        } else {
            CanonPath::MergeOverride
        }
    }
    pub(crate) fn make_merge_fn_properties() -> util::MaybeFallibleFn {
        util::MaybeFallibleFn::new(format_ident!("merge_with_override"), ())
    }
    pub(crate) fn merge_fn_properties(&self) -> FnProperties {
        Self::make_merge_fn_properties()
            .into_fallible_when(self.plan().fallible_merge, self.ids().config_error_path().clone())
    }
    pub(crate) fn merge_trait_fn_properties(&self) -> FnProperties {
        let error_ty: TypePath = parse_quote!(Self::Error);
        Self::make_merge_fn_properties()
            .into_fallible_when(self.plan().fallible_merge, error_ty)
    }
    pub(crate) fn build_consensus(&self) -> ConfigConsensus {
        let infallible_merge = self.fields.iter()
            .map(ConfigField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        let fallible_merge = self.fields.iter()
            .map(ConfigField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        let infallible_from_override = self.fields.iter()
            .map(ConfigField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        let fallible_from_override = self.fields.iter()
            .map(ConfigField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        ConfigConsensus {
            copy: IterVoteCollector::collect_votes(&self.fields),
            default: IterVoteCollector::collect_votes(&self.fields),
            infallible_merge,
            fallible_merge,
            infallible_from_override,
            fallible_from_override,
        }
    }
    pub(crate) fn consensus(&self) -> &ConfigConsensus {
        self.consensus.get_or_init(|| self.build_consensus())
    }
    pub(crate) fn plan(&self) -> &ConfigPlan {
        self.plan.get_or_init(|| self.consensus().as_plan())
    }
}
impl IdentifyVoter for ConfigStruct {
    fn voter_identity(&self) -> String {
        self.ident().to_string()
    }
}
impl CastVote<InfallibleMerge> for ConfigStruct {
    fn cast_ballot(&self) -> Vote<InfallibleMerge> {
        let proxy: Vote<InfallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<FallibleMerge> for ConfigStruct {
    fn cast_ballot(&self) -> Vote<FallibleMerge> {
        let proxy: Vote<FallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<FallibleFromOverride> for ConfigStruct {
    fn cast_ballot(&self) -> Vote<FallibleFromOverride> {
        let proxy: Vote<FallibleFromOverride> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<InfallibleFromOverride> for ConfigStruct {
    fn cast_ballot(&self) -> Vote<InfallibleFromOverride> {
        let proxy: Vote<InfallibleFromOverride> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl ConfigStruct {
    // ! Struct token output
    fn assert_fn_tokens(&self) -> TokenStream {
        if self.origin.has_generics() {
            eprintln!("Generics detected on Config struct.

Compatibility assertions have been disabled due to lack of support for generics.

Expect degraded error quality and reduced correctness checks.");
            return TokenStream::new();
        }

        let mut asserts = TokenStream::new();
        let mut seen = Vec::new();

        // de-dupe asserts
        for field in &self.fields {
            for field_assert in field.assert_tokens() {
                let assert_str = field_assert.to_string();
                if !seen.contains(&assert_str) {
                    asserts.extend(field_assert);
                    seen.push(assert_str);
                }
            }
            // TODO: once support for cfg fields is added to impl detection:
            // - if field has cfgs, add its asserts with the cfgs regardless of seen
        }
        quote! {
            #[doc(hidden)]
            #[allow(dead_code)]
            fn __assertions() {
                #asserts
            }
        }
    }

    /// parameters:
    /// - override config if any fields contain the `override_required` flag
    /// - all non-optional fields that do not contain the `override_required` flag
    ///
    /// assignments:
    /// - all fields, with source selection prioritized by:
    ///   - any field with `override_required`: `override_conf`
    ///     - must handle origin being optional or required
    ///     - must handle potentially mapped override type
    ///   - any required origin field: dedicated parameter
    ///   - remaining (optional): default assignment
    fn new_fn_tokens(&self) -> TokenStream {
        let mut params = Vec::new();
        let mut fields = Vec::new();
        if self.override_struct.has_required_fields() {
            let override_ty = self.override_struct.ty();
            params.push(quote!(override_conf: #override_ty));
        }
        for field in &self.fields {
            let ident = &field.ident();

            let assign =
            // assign from override_conf
            if field.attrs().override_required {
                let mut inner = quote!(override_conf.#ident.clone());
                // mapped
                if let Some(override_from) = &field.attrs().override_from {
                    if let Some(mapped_via) = &override_from.via_ty {
                        inner = quote!(<#mapped_via>::from(#inner));
                    }
                    inner = quote!(#inner.into());
                }
                // origin is optional, wrap in Some
                if field.origin.is_optional() {
                    let option_path = CanonPath::Option;
                    inner = quote!(#option_path::Some(#inner));
                }
                inner
            }
            // from param
            else if field.origin.is_required() {
                // add to params
                params.push(field.origin.as_fn_param_tokens());

                ident.to_token_stream()
            }
            // from default
            else {
                field.origin.default().unwrap().to_token_stream()
            };

            let cfg_attrs = field.cfg_tokens();
            fields.push(quote! {
                #cfg_attrs
                #ident: #assign
            });
        }

        quote! {
            #[must_use]
            #[allow(clippy::redundant_field_names, reason = "https://github.com/rust-lang/rust-clippy/issues/17525")]
            pub(crate) fn new(#(#params),*) -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    }

    // TODO: create ConstructorParams struct w/:
    // - opt override conf field containing list of idents + types of req. fields

    fn builder_fn_tokens(&self) -> TokenStream {
        let mut params = Vec::new();
        let mut idents = Vec::new();

        if self.override_struct.has_required_fields() {
            let override_ty = self.override_struct.ty();
            params.push(quote!(override_conf: #override_ty));
            idents.push(quote!(override_conf));
        }
        for field in &self.origin.fields {
            // only other params are config-required and override-optional
            if !field.attrs.override_required && field.is_required() {
                params.push(field.as_fn_param_tokens());
                let ident = &field.ident;
                let cfg_attrs = field.passthrough_attrs().cfg_tokens();
                idents.push(quote! {
                    #cfg_attrs
                    #ident
                });
            }
        }
        let builder_ty = self.builder_struct.ty();

        quote! {
            /// Creates a new config builder
            #[must_use]
            pub fn builder(#(#params),*) -> #builder_ty {
                <#builder_ty>::new(#(#idents),*)
            }
        }
    }
    fn getter_fn_tokens(&self) -> TokenStream {
        self.fields
            .iter()
            .filter(|f| !f.attrs().skip_config_getter)
            .map(ConfigField::getter_tokens)
            .collect()
    }
    fn merge_trait_impl_tokens(&self) -> TokenStream {
        let override_ty = &self.override_struct.ty();
        let error_ty = self.ids().config_error_path();
        let conf_var_ident = format_ident!("override_conf");

        let trait_path = self.merge_trait_path();
        let fn_props = self.merge_trait_fn_properties();
        let (merge_fn_name, ret_sig) = fn_props.split_for_impl();
        let result_path = CanonPath::Result;

        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        let fields: Vec<_> = self.fields
            .iter()
            .filter(|f| !f.attrs().skip_override_field)
            .map(|f| f.merge_override_tokens(&conf_var_ident))
            .collect();

        let validate_call = self.origin.attrs.validate_fn.as_ref().map(|val_fn|
            quote!(#val_fn(&self)?;)
        );
        let ret_expr = self.plan().fallible_merge.then(||
            quote!(#result_path::Ok(()))
        );
        let error_expr = self.plan().fallible_merge.then(||
            quote!(type Error = #error_ty;)
        );

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
                type Override = #override_ty;
                #error_expr

                fn #merge_fn_name(&mut self, #conf_var_ident: Self::Override) #ret_sig {
                    #(#fields)*

                    #validate_call

                    #ret_expr
                }
            }
        }
    }
    fn merge_fn_impl_tokens(&self) -> TokenStream {
        let override_ty = &self.override_struct.ty();
        let conf_var_ident = format_ident!("override_conf");

        let fn_props = self.merge_fn_properties();
        let (merge_fn_name, ret_sig) = fn_props.split_for_impl();
        let trait_path = self.merge_trait_path();

        quote! {
            /// Applies all values from override (which are `Some(_)`) to self
            pub fn #merge_fn_name(&mut self, #conf_var_ident: #override_ty) #ret_sig {
                <Self as #trait_path>::#merge_fn_name(self, #conf_var_ident)
            }
        }
    }

    fn impl_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        let assert_fn = self.assert_fn_tokens();
        let new_fn = self.new_fn_tokens();
        let builder_fn = self.builder_fn_tokens();
        let getter_fn = self.getter_fn_tokens();
        let merge_fn = self.merge_fn_impl_tokens();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            impl #impl_generics #ident #ty_generics #where_clause {
                #assert_fn
                #new_fn
                #builder_fn
                #getter_fn
                #merge_fn
            }
        }
    }

    fn copy_impl_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let copy_path = CanonPath::Copy;
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #copy_path for #ident #ty_generics #where_clause { }
        }
    }
    fn default_impl_tokens(&self) -> TokenStream {
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        util::bare_default_impl_tokens(self.ident(), self.origin.generics(), &cfg_attrs, &doc_cfg_attrs)
    }
    #[expect(clippy::wrong_self_convention, reason = "not a from_*() initializer fn")]
    fn from_override_impl_tokens(&self) -> TokenStream {
        assert!(self.plan().infallible_from_override);

        let ident = self.ident();
        let override_ty = &self.override_struct.ty();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let from_path = CanonPath::From;
        let conf_var_ident = format_ident!("override_conf");

        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        let inner_expr = if self.plan().default {
            let merge_props = self.merge_fn_properties();
            let merge_fn_name = &merge_props.name();
            let merge_expr_suffix = self.plan().fallible_merge.then(|| quote!(?));
            // note: does not need fully qualified trait syntax, calling on self
            quote! {
                let mut out = Self::default();
                out.#merge_fn_name(override_conf)#merge_expr_suffix;
                out
            }
        } else {
            let mut fields = Vec::new();
            for field in &self.fields {
                fields.push(field.from_override_tokens(&conf_var_ident));
            }
            quote! {
                Self {
                    #(#fields)*
                }
            }
        };

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #from_path<#override_ty> for #ident #ty_generics #where_clause {
                fn from(#conf_var_ident: #override_ty) -> Self {
                    #inner_expr
                }
            }
        }
    }
    fn try_from_override_impl_tokens(&self) -> TokenStream {
        assert!(self.plan().fallible_from_override);

        let ident = self.ident();
        let override_ty = &self.override_struct.ty();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let try_from_path = CanonPath::TryFrom;
        let result_path = CanonPath::Result;
        let error_path = self.ids().config_error_path().clone().into_type_path();
        let conf_var_ident = format_ident!("override_conf");

        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        let mut fields = Vec::new();
        for field in &self.fields {
            assert!(field.origin.has_default() || !field.attrs().skip_override_field);
            fields.push(field.try_from_override_tokens(&conf_var_ident, &error_path));
        }

        let validate_call = self.origin.attrs.validate_fn.as_ref().map(|val_fn|
            quote!(#val_fn(&out)?;)
        );

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #try_from_path<#override_ty> for #ident #ty_generics #where_clause {
                type Error = #error_path;
                fn try_from(#conf_var_ident: #override_ty) -> #result_path<Self, Self::Error> {
                    let out = Self {
                        #(#fields)*
                    };
                    #validate_call
                    #result_path::Ok(out)
                }
            }
        }
    }

    fn add_impl_tokens(&self) -> TokenStream {
        assert!(!self.plan().fallible_merge);

        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let add_path = CanonPath::Add;
        let override_ty = &self.override_struct.ty();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #add_path<#override_ty> for #ident #ty_generics #where_clause {
                type Output = Self;

                fn add(mut self, rhs: #override_ty) -> Self {
                    self += rhs;
                    self
                }
            }
        }
    }
    fn add_assign_impl_tokens(&self) -> TokenStream {
        assert!(!self.plan().fallible_merge);

        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let add_assign_path = CanonPath::AddAssign;
        let override_ty = &self.override_struct.ty();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics().split_for_impl();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #add_assign_path<#override_ty> for #ident #ty_generics #where_clause {
                fn add_assign(&mut self, rhs: #override_ty) {
                    self.merge_with_override(rhs);
                }
            }
        }
    }
}
impl ToTokens for ConfigStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.impl_tokens());

        if self.plan().default {
            tokens.extend(self.default_impl_tokens());
        }
        if self.plan().copy {
            tokens.extend(self.copy_impl_tokens());
        }

        tokens.extend(self.merge_trait_impl_tokens());

        if self.plan().infallible_from_override {
            tokens.extend(self.from_override_impl_tokens());
        }
        else if self.plan().fallible_from_override {
            tokens.extend(self.try_from_override_impl_tokens());
        }
        if self.plan().infallible_merge {
            tokens.extend(self.add_impl_tokens());
            tokens.extend(self.add_assign_impl_tokens());
        }

        if self.plan().fallible_merge || self.plan().fallible_from_override {
            ConfigErrorEnum::new(self).to_tokens(tokens);
        }
    }
}

// !- Config struct field

pub(crate) type ConfigField = VariantField<ConfigVariant>;

impl CastVote<DeriveCopy> for ConfigField {
    fn cast_ballot(&self) -> Vote<DeriveCopy> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}
impl CastVote<DeriveDefault> for ConfigField {
    fn cast_ballot(&self) -> Vote<DeriveDefault> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}
impl CastVote<InfallibleMerge> for ConfigField {
    fn cast_ballot(&self) -> Vote<InfallibleMerge> {
        let proxy: Vote<InfallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<FallibleMerge> for ConfigField {
    fn cast_ballot(&self) -> Vote<FallibleMerge> {
        let proxy: Vote<FallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<InfallibleFromOverride> for ConfigField {
    fn cast_ballot(&self) -> Vote<InfallibleFromOverride> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}
impl CastVote<FallibleFromOverride> for ConfigField {
    fn cast_ballot(&self) -> Vote<FallibleFromOverride> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}
impl ConfigField {
    fn getter_ret_ty(&self) -> Type {
        // copy permitted, direct return
        if self.attrs().copy {
            return self.origin.ty.clone();
        }

        let ty = &self.origin.ty;
        let option_path = CanonPath::Option;

        if let Some(inner_ty) = parse::util::unwrap_option(ty) {
            if parse::util::is_string(inner_ty) {
                // type is Option<String>
                parse_quote!(#option_path<&str>)
            } else {
                // type is Option<_>
                parse_quote!(#option_path<&#inner_ty>)
            }
        } else {
            if parse::util::is_string(ty) {
                // type is String
                parse_quote!(&str)
            } else {
                // Any other type
                parse_quote!(&#ty)
            }
        }
    }
    fn getter_ret_expr(&self) -> TokenStream {
        let ident = self.ident();
        let inner = quote!(self.#ident);

        // copy permitted, direct return
        if self.attrs().copy {
            return inner;
        }

        let ty = &self.origin.ty;
        if let Some(inner_ty) = parse::util::unwrap_option(ty) {
            if parse::util::is_string(inner_ty) {
                // type is Option<String>
                quote!(#inner.as_ref().map(|s| s.as_str()))
            } else {
                // type is Option<_>
                quote!(#inner.as_ref())
            }
        } else {
            if parse::util::is_string(ty) {
                // type is String
                quote!(#inner.as_str())
            } else {
                // Any other type
                quote!(&#inner)
            }
        }
    }
}

impl ConfigField {
    // ! Field-based item docs

    pub(crate) fn getter_docs(&self) -> AppendDoc {
        self.append_docs()
        + Doc::new().separator()
        + self.doc_default_val()
    }
}

impl ConfigField {
    // ! Field-based token output

    pub(super) fn assert_tokens(&self) -> Vec<TokenStream> {
        use util::{assert_impl_all, CanonPath, CanonTrait};

        let mut out = Vec::new();
        let ty = &self.origin.ty;
        let flat_ty = &self.origin.flat_ty;

        // ensure base traits covered
        out.push(assert_impl_all(ty, &[CanonPath::Clone, CanonPath::Debug]));

        // ensure Copy if attr set
        if self.attrs().copy {
            out.push(assert_impl_all(ty, &[CanonPath::Copy]));
        }

        // override from mappings
        if let Some(override_from) = self.attrs().override_from() {
            let from_ty = &override_from.ty;
            if let Some(via_ty) = &override_from.via_ty {
                // ensure override_via has From<override_from>
                out.push(assert_impl_all(via_ty, &[CanonTrait::From(quote!(#from_ty))]));
                // ensure ty has From<override_via>
                out.push(assert_impl_all(flat_ty, &[CanonTrait::From(quote!(#via_ty))]));
            } else {
                // ensure ty has From<override_from>
                out.push(assert_impl_all(flat_ty, &[CanonTrait::From(quote!(#from_ty))]));
            }

            // ensure override_from is Copy if attr set
            if override_from.copy {
                out.push(assert_impl_all(from_ty, &[CanonPath::Copy]));
            }
        }

        // nested
        if let Some(nested) = self.attrs().nested() {
            assert!(self.origin.nested_ids().is_some());
            let nested_ids = self.origin.nested_ids().unwrap();
            let nested_ov_path = nested_ids.override_path();
            let nested_builder_path = nested_ids.builder_path();
            let nested_error_path = nested_ids.config_error_path();
            // ensure Default if attr set
            if nested.builder_default {
                out.push(assert_impl_all(&nested_builder_path, &[CanonPath::Default]));
            }
            // ensure Build/Merge if infallible attr set
            if nested.build.is_infallible() {
                out.push(assert_impl_all(&flat_ty, &[CanonTrait::MergeOverride {
                    override_: Some(quote!(#nested_ov_path)),
                }]));
                out.push(assert_impl_all(&nested_builder_path, &[CanonTrait::BuildConfig {
                    config: Some(quote!(#flat_ty)),
                }]));
            }
            // ensure TryBuild/TryMerge if fallible attr set
            else {
                out.push(assert_impl_all(&flat_ty, &[CanonTrait::TryMergeOverride {
                    override_: Some(quote!(#nested_ov_path)),
                    error: Some(quote!(#nested_error_path)),
                }]));
                out.push(assert_impl_all(&nested_builder_path, &[CanonTrait::TryBuildConfig {
                    config: Some(quote!(#flat_ty)),
                    error: Some(quote!(#nested_error_path)),
                }]));
            }
            // ensure From<Override> if attr set
            if nested.from_override.is_infallible() {
                out.push(assert_impl_all(&flat_ty, &[CanonTrait::From(quote!(#nested_ov_path))]));
            }
            // ensure TryFrom<Override> if attr set
            else if nested.from_override.is_fallible() {
                out.push(assert_impl_all(&flat_ty, &[CanonTrait::TryFrom {
                    from: quote!(#nested_ov_path),
                    error: Some(quote!(#nested_error_path)),
                }]));
            }
        }
        out
    }
    pub(super) fn getter_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let docs = self.getter_docs();
        let ty = self.getter_ret_ty();
        let ret = self.getter_ret_expr();
        let cfg_attrs = self.cfg_tokens();
        let doc_cfg_attrs = self.doc_cfg_tokens();
        quote! {
            #docs
            #cfg_attrs
            #doc_cfg_attrs
            #[must_use]
            pub fn #ident(&self) -> #ty {
                #ret
            }
        }
    }
    pub(super) fn merge_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        if self.origin.is_nested() {
            self.nested_merge_override_tokens(override_var_ident)
        } else {
            self.standard_merge_override_tokens(override_var_ident)
        }
    }

    fn nested_merge_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        assert!(self.origin.is_nested());

        let ident = self.ident();
        let option_path = CanonPath::Option;
        let flat_ty = &self.origin.flat_ty;
        let nested_attrs = self.attrs().nested().unwrap();
        let merge_props = nested_attrs.build.merge_properties();
        let merge_fn = merge_props.name();
        let merge_suffix = self.origin.nested_build_map_err_expr();
        let merge_trait_path = nested_attrs.build.merge_trait_path();
        let from_override_suffix = self.origin.nested_from_override_map_err_expr();

        let temp_assign = quote!(let val = #override_var_ident.#ident;);
        let assign = if self.origin.is_option {
            let init_fallback = match nested_attrs.from_override {
                FromOverrideSupport::Infallible => quote!(<#flat_ty>::from(val)),
                FromOverrideSupport::Fallible => quote!(<#flat_ty>::try_from(val)#from_override_suffix),
                FromOverrideSupport::None => unreachable!("nested merge - Shouldn't be able to execute this branch if no from_override is available")
            };
            quote! {
                // can safely ignore if not present in override
                if let #option_path::Some(val) = val {
                    // nested struct is present, simply do recursive merge
                    if let #option_path::Some(current) = &mut self.#ident {
                        <#flat_ty as #merge_trait_path>::#merge_fn(current, val)#merge_suffix;
                    }
                    // supports live init and merge
                    else {
                        let mut inited = #init_fallback;
                        self.#ident = #option_path::Some(inited);
                    }
                }
            }
        }
        // stored config is non-option: trivial merges
        else {
            // stored config is non-option, trivial merge call
            if self.attrs().override_required {
                quote! {
                    <#flat_ty as #merge_trait_path>::#merge_fn(&mut self.#ident, val)#merge_suffix;
                }
            }
            // requires if let check for override presence
            else {
                // NOTE: keeping the inner portions separate to reduce maintenance burden, KISS
                quote! {
                    if let #option_path::Some(val) = val {
                        <#flat_ty as #merge_trait_path>::#merge_fn(&mut self.#ident, val)#merge_suffix;
                    }
                }
            }
        };

        let cfg_attrs = self.cfg_tokens();
        quote! {
            #cfg_attrs
            {
                #temp_assign
                #assign
            }
        }
    }
    fn standard_merge_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        assert!(!self.origin.is_nested());

        let ident = self.ident();
        let option_path = CanonPath::Option;

        // store field in temporary var with name 'val'
        let temp_assign = quote!(let val = #override_var_ident.#ident;);

        // build assignment expression
        let mut assign = quote!(val);
        if let Some(override_from) = &self.attrs().override_from {
            if let Some(via_ty) = &override_from.via_ty {
                assign = quote!(<#via_ty>::from(#assign));
            }
            assign = quote!(#assign.into());
        }

        // wrap in Some if Option
        if self.origin.is_option {
            assign = quote!(#option_path::Some(#assign));
        }

        // origin type is guaranteed optional and non nested if an unset ident is given
        let unsetter = self.origin.unset_ident().map(|unset_ident| {
            let default = self.origin.default().unwrap();
            quote! {
                if #override_var_ident.#unset_ident {
                    self.#ident = #default;
                } else
            }
        });

        // wrapped in brackets to allow for conditional preceding if {..} else
        let mut setter = quote! {
            {
                self.#ident = #assign;
            }
        };

        // wrap setter in if let Some.. to handle Option<> wrapped override fields
        if !self.attrs().override_required {
            setter = quote!(if let #option_path::Some(val) = val #setter);
        }

        // lastly wrap all in any cfg attrs and serve
        let cfg_attrs = self.cfg_tokens();
        quote! {
            #cfg_attrs
            {
                #temp_assign
                #unsetter
                #setter
            }
        }
    }
    #[expect(clippy::wrong_self_convention, reason = "not a from_*() initializer fn")]
    pub(super) fn from_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        if self.origin.is_nested() {
            self.nested_from_override_tokens(override_var_ident)
        } else {
            self.standard_from_override_tokens(override_var_ident)
        }
    }
    fn nested_from_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        fn from_expr(origin: &Rc<OriginField>, from_override: &FromOverrideSupport) -> TokenStream {
            let flat_ty = &origin.flat_ty;
            let map_err = origin.nested_from_override_map_err_expr();
            match from_override {
                FromOverrideSupport::Infallible => quote!(<#flat_ty>::from(val)),
                FromOverrideSupport::Fallible => quote!(<#flat_ty>::try_from(val)#map_err),
                FromOverrideSupport::None => unreachable!("nested from override - shouldn't be able to execute this branch if no from_override is available")
            }
        }

        assert!(self.origin.is_nested());

        let ident = self.ident();
        let nested_attrs = self.attrs().nested().unwrap();
        let option_path = CanonPath::Option;
        // scenarios
        // override_required -> from(override) (field can't be option)
        // has explicit default impl -> default + merge
        // override optional and set -> from(override)

        let temp_assign = quote!(let val = #override_var_ident.#ident;);

        let assign =
        if self.attrs().override_required {
            from_expr(&self.origin, &nested_attrs.from_override)
        }
        // guaranteed override ty is Option<>
        else if self.origin.is_option {
            let from_expr = from_expr(&self.origin, &nested_attrs.from_override);
            quote! {
                if let #option_path::Some(val) = val {
                    #option_path::Some(#from_expr)
                } else {
                    #option_path::None
                }
            }
        }
        else if let Some(default) = self.origin.default() {
            let flat_ty = &self.origin.flat_ty;
            let merge_props = nested_attrs.build.merge_properties();
            let merge_fn = merge_props.name();
            let merge_suffix = self.origin.nested_build_map_err_expr();
            let merge_trait_path = nested_attrs.build.merge_trait_path();
            quote! {
                if let #option_path::Some(val) = val {
                    let mut base = #default;
                    <#flat_ty as #merge_trait_path>::#merge_fn(&mut base, val)#merge_suffix;
                    base
                } else {
                    #default
                }
            }
        } else {
            let from_expr = from_expr(&self.origin, &nested_attrs.from_override);
            let field_name = ident.to_string();
            let error_ty = self.ids().config_error_path().clone().into_type_path();
            let result_path = CanonPath::Result;
            quote! {
                if let #option_path::Some(val) = val {
                    #from_expr
                } else {
                    #result_path::Err(#error_ty::MissingRequiredFields { field_names: vec![#field_name.to_string()] })?
                }
            } // FIXME: dont fail fast, collect all missing field names ^^^^^^^^^^^^^^
        };

        let cfg_attrs = self.cfg_tokens();
        quote! {
            #cfg_attrs
            #ident: {
                #temp_assign
                #assign
            },
        }
    }
    fn standard_from_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        assert!(!self.origin.is_nested());

        let ident = self.ident();

        let final_assign =
        // short-circuit on override-skip - field must be default
        if self.attrs().skip_override_field {
            assert!(self.origin.has_default());
            self.origin.default().unwrap().to_token_stream()
        }
        // override not skipped
        else {
            let option_path = CanonPath::Option;

            // store field in temporary var with name 'val'
            let temp_assign = quote!(let val = #override_var_ident.#ident;);

            // build assignment expression
            let mut assign = quote!(val);
            if let Some(override_from) = &self.attrs().override_from {
                if let Some(via_ty) = &override_from.via_ty {
                    assign = quote!(<#via_ty>::from(#assign));
                }
                assign = quote!(#assign.into());
            }

            // wrap in Some if Option
            if self.origin.is_option {
                assign = quote!(#option_path::Some(#assign));
            }

            // origin type is guaranteed optional if an unset ident is given
            let unsetter = self.origin.unset_ident().map(|unset_ident| {
                let default = self.origin.default().unwrap();
                quote! {
                    if #override_var_ident.#unset_ident {
                        #default
                    } else
                }
            });

            // wrapped in brackets to allow for conditional preceding if {..} else
            let mut setter = quote!({#assign});

            let mut fallback = TokenStream::new();
            // override is optional
            if !self.attrs().override_required {
                // wrap setter in if let Some..
                setter = quote! {
                    if let #option_path::Some(val) = val #setter
                };

                fallback =
                // fallback to default
                if let Some(default) = self.origin.default() {
                    quote!(#default)
                }
                // No default for fallback, error out
                else {
                    unreachable!("disqualifies_from_override should have picked up incompat standard field");
                };

                // wrap fallback in else block
                fallback = quote! {
                    else {
                        #fallback
                    }
                };
            }

            // assemble branches and wrap in brackets
            quote! {
                {
                    #temp_assign
                    #unsetter
                    #setter
                    #fallback
                }
            }
        };

        let cfg_attrs = self.cfg_tokens();
        // lastly:
        // - assign to field
        // - wrap in any cfg attrs
        quote! {
            #cfg_attrs
            #ident: #final_assign,
        }
    }
    pub(super) fn try_from_override_tokens(&self, override_var_ident: &Ident, error_ty: &TypePath) -> TokenStream {
        if self.origin.is_nested() {
            self.nested_from_override_tokens(override_var_ident)
        } else {
            self.standard_try_from_override_tokens(override_var_ident, error_ty)
        }
    }
    // FIXME: breakup into struct or helper fns, a lot of this is reused (merge, from, try_from)
    pub(super) fn standard_try_from_override_tokens(&self, override_var_ident: &Ident, error_ty: &TypePath) -> TokenStream {
        let ident = self.ident();

        let final_assign =
        // short-circuit on override-skip - field must be default
        if self.attrs().skip_override_field {
            assert!(self.origin.has_default());
            self.origin.default().unwrap().to_token_stream()
        }
        // override not skipped
        else {
            let option_path = CanonPath::Option;

            // store field in temporary var with name 'val'
            let temp_assign = quote!(let val = #override_var_ident.#ident;);

            // build assignment expression
            let mut assign = quote!(val);
            if let Some(override_from) = &self.attrs().override_from {
                if let Some(via_ty) = &override_from.via_ty {
                    assign = quote!(<#via_ty>::from(#assign));
                }
                assign = quote!(#assign.into());
            }

            // wrap in Some if Option
            if self.origin.is_option {
                assign = quote!(#option_path::Some(#assign));
            }

            // origin type is guaranteed optional if an unset ident is given
            let unsetter = self.origin.unset_ident().map(|unset_ident| {
                let default = self.origin.default().unwrap();
                quote! {
                    if #override_var_ident.#unset_ident {
                        #default
                    } else
                }
            });

            let mut setter = if self.origin.is_nested() {
                quote!(#assign) // FIXME: call from / try_from
            } else {
                assign
            };

            // wrapped in brackets to allow for conditional preceding if {..} else
            setter = quote!({#setter});

            let mut fallback = TokenStream::new();

            // override is optional
            if !self.attrs().override_required {
                // wrap setter in if let Some..
                setter = quote! {
                    if let #option_path::Some(val) = val #setter
                };

                fallback =
                // fallback to default
                if let Some(default) = self.origin.default() {
                    quote!(#default)
                }
                // No default for fallback, error out
                else {
                    let field_name = ident.to_string();
                    let result_path = CanonPath::Result;
                    // FIXME: dont fail fast
                    quote! {
                        #result_path::Err(#error_ty::MissingRequiredFields { field_names: vec![#field_name.to_string()] })?
                    }
                };

                // wrap fallback in else block
                fallback = quote! {
                    else {
                        #fallback
                    }
                };
            }

            // assemble branches and wrap in brackets
            quote! {
                {
                    #temp_assign
                    #unsetter
                    #setter
                    #fallback
                }
            }
        };

        let cfg_attrs = self.cfg_tokens();
        // lastly:
        // - assign to field
        // - wrap in any cfg attrs
        quote! {
            #cfg_attrs
            #ident: #final_assign,
        }
    }
}

// !- Merge error

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ConfigErrorEnum {
    ident: Ident,
    origin_ident: Rc<Ident>,
    origin_link: Link,
    with_self_variant: bool,
    validation_error: Option<TypePath>,
    nested_variants: Vec<ConfigErrorNestedVariant>,
}
impl ConfigErrorEnum {
    pub(crate) fn new(config: &ConfigStruct) -> Self {
        // TODO: handle field name conflicts
        let nested_variants = config.fields
            .iter()
            .filter(|f| f.origin.nested_has_config_error())
            .map(ConfigErrorNestedVariant::new)
            .collect();

        Self {
            ident: config.ids().config_error_ident().clone(),
            origin_ident: config.ident().clone(),
            origin_link: config.ids().origin_link().struct_only(),
            with_self_variant: config.origin.has_required_fields(),
            validation_error: config.origin.attrs.validate_error.clone(),
            nested_variants,
        }
    }
    pub(crate) fn missing_fields_variant_tokens(&self) -> Option<TokenStream> {
        self.with_self_variant.then(|| {
            quote! {
                /// Unable to initialize struct due to missing required fields
                MissingRequiredFields {
                    /// Names of missing fields
                    field_names: Vec<String>,
                },
            }
        })
    }
    pub(crate) fn validation_variant_tokens(&self) -> Option<TokenStream> {
        self.validation_error.as_ref().map(|error| {
            quote! {
                /// Config init failed during validation
                Validation(#error),
            }
        })
    }
    pub(crate) fn missing_fields_display_tokens(&self) -> Option<TokenStream> {
        self.with_self_variant.then(|| {
            let error_fields_path = CanonPath::private_path(parse_quote!(error_field_list));
            let origin_ident = &self.origin_ident;
            let message = format!("Missing required `{origin_ident}` fields:\n{{}}");
            quote! {
                Self::MissingRequiredFields { field_names } => format!(#message, #error_fields_path(field_names)),
            }
        })
    }
    pub(crate) fn validation_display_tokens(&self) -> Option<TokenStream> {
        self.validation_error.is_some().then(|| {
            quote! {
                Self::Validation(error) => error.to_string(),
            }
        })
    }
}
impl ToTokens for ConfigErrorEnum {
    #[expect(clippy::redundant_closure_for_method_calls, reason = "reduce imports that are otherwise unused")]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // assert?
        let ident = &self.ident;
        let debug_path = CanonPath::Debug;
        let display_path = CanonPath::Display;
        let option_path = CanonPath::Option;
        let result_path = CanonPath::Result;
        let error_path = CanonPath::Error;
        let string_path = CanonPath::String;
        let vec_path = CanonPath::Vec;
        let config_error_path = CanonPath::ConfigError;
        let origin_link = &self.origin_link;
        let docs = Doc::from(format!("Possible errors encountered during builf of {origin_link}"));
        let self_variant = self.missing_fields_variant_tokens();
        let validation_variant = self.validation_variant_tokens();
        let variants = &self.nested_variants;

        let enum_decl = quote! {
            #docs
            #[derive(#debug_path)]
            pub enum #ident {
                #self_variant
                #validation_variant
                #(#variants)*
            }
        };

        let variant_build_chains: Vec<_> = self.nested_variants.iter().map(|v| v.build_chain_match_tokens()).collect();
        let variant_origin_matches: Vec<_> = self.nested_variants.iter().map(|v| v.origin_message_match_tokens()).collect();

        let member_fns = quote! {
            /// List of field names leading to the source of a nested error.
            ///
            /// Does not include the offending field(s) themselves, just the sequence of fields leading to their containing `Struct`
            pub fn source_chain(&self) -> #vec_path<#string_path> {
                <Self as #config_error_path>::source_chain(self)
            }

            /// Same as [`source_chain()`](Self::source_chain), but returns the chain as a single dot-delimited string
            pub fn source_path(&self) -> #string_path {
                <Self as #config_error_path>::source_path(self)
            }

            /// Returns the source error message
            ///
            /// Used to generate the full error including the nested path
            pub fn origin_message(&self) -> #string_path {
                <Self as #config_error_path>::origin_message(self)
            }
        };

        let self_display = self.missing_fields_display_tokens();
        let validation_display = self.validation_display_tokens();
        let variant_display: Vec<_> = self.nested_variants.iter().map(|v| v.display_match_tokens()).collect();

        let display_impl = quote! {
            #[automatically_derived]
            impl #display_path for #ident {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let text = match self {
                        #self_display
                        #validation_display
                        #(#variant_display)*
                    };
                    write!(f, "{text}")?;
                    #result_path::Ok(())
                }
            }
        };

        let error_impl = quote! {
            #[automatically_derived]
            impl #error_path for #ident {}
        };

        let config_error_impl = quote! {
            #[automatically_derived]
            impl #config_error_path for #ident {
                fn build_field_chain(&self, chain: &mut #vec_path<#string_path>, self_field_name: #option_path<&str>) {
                    if let #option_path::Some(name) = self_field_name {
                        chain.push(name.to_string());
                    }

                    match self {
                        #(#variant_build_chains)*
                        _ => {},
                    };
                }
                fn origin_message(&self) -> #string_path {
                    match self {
                        #(#variant_origin_matches)*
                        _ => self.to_string(),
                    }
                }
            }
        };

        let from_validation_error_impl = self.validation_error.as_ref().map(|error| {
            let from_path = CanonPath::From;
            quote! {
                #[automatically_derived]
                impl #from_path<#error> for #ident {
                    fn from(validation_error: #error) -> Self {
                        Self::Validation(validation_error)
                    }
                }
            }
        });

        tokens.extend(quote! {
            #enum_decl

            impl #ident {
                #member_fns
            }
            #display_impl
            #error_impl
            #config_error_impl
            #from_validation_error_impl
        });
    }
}

/// Merge error variant for a single nested config field
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ConfigErrorNestedVariant {
    ident: Ident,
    field_ident: Rc<Ident>,

    error_ty: Rc<TypePath>,

    nested_link: Link,
    nested_merge_link: Link,

    cfg_attrs: TokenStream,
    doc_cfg_attrs: TokenStream,
}
impl ConfigErrorNestedVariant {
    pub(crate) fn new(field: &ConfigField) -> Self {
        let error_ty = field.origin.nested_config_error().expect(
            "Merge nested error set already checked"
        ).clone();
        let nested_ids = field.origin.nested_ids().unwrap();
        let nested_link = nested_ids.origin_link().struct_only();
        let nested_merge_link = nested_ids.origin_link().method_explicit("try_merge");

        Self {
            ident: format_ident!("{}Field", field.origin.ident_upper_pascal()),
            field_ident: field.origin.ident.clone(),
            error_ty,
            nested_link,
            nested_merge_link,
            cfg_attrs: field.cfg_tokens(),
            doc_cfg_attrs: field.doc_cfg_tokens(),
        }
    }
    pub(crate) fn build_chain_match_tokens(&self) -> TokenStream {
        let name = &self.ident;
        let field_name = self.field_ident.to_string();
        let cfg_attrs = &self.cfg_attrs;
        let option_path = CanonPath::Option;
        quote! {
            #cfg_attrs
            Self::#name { error } => error.build_field_chain(chain, #option_path::Some(#field_name)),
        }
    }
    pub(crate) fn origin_message_match_tokens(&self) -> TokenStream {
        let name = &self.ident;
        let cfg_attrs = &self.cfg_attrs;
        quote! {
            #cfg_attrs
            Self::#name { error } => error.origin_message(),
        }
    }
    pub(crate) fn display_match_tokens(&self) -> TokenStream {
        let name = &self.ident;
        let cfg_attrs = &self.cfg_attrs;
        quote! {
            #cfg_attrs
            Self::#name { error } => {
                let path = error.source_path();
                let origin_message = error.origin_message();
                format!("Error in nested config at `{path}`: {origin_message}")
            }
        }
    }
}
impl ToTokens for ConfigErrorNestedVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let error_ty = &self.error_ty;
        let nested_link = &self.nested_link;
        let nested_merge_link = &self.nested_merge_link;
        let cfg_attrs = &self.cfg_attrs;
        let doc_cfg_attrs = &self.doc_cfg_attrs;

        let docs = Doc::new() + format!("Failed to merge nested {nested_link}");
        let error_docs = Doc::new() + format!("Corresponds to the error type returned by {nested_merge_link}");

        tokens.extend(quote! {
            #docs
            #cfg_attrs
            #doc_cfg_attrs
            #ident {
                #error_docs
                error: #error_ty,
            },
        });
    }
}
