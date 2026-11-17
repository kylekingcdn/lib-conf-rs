use crate::{
    generate::{
        doc::{AppendDoc, Doc, Document},
        ConfigStruct, OverrideStruct, VariantField,
        plan::{BuilderConsensus, BuilderPlan, FallibleBuild, InfallibleBuild},
        util::{self, CanonPath, FnProperties},
    },
    parse::{IdentPool, OriginStruct},
};

use arbitrait::vote::{CastVote, IdentifyVoter, Vote};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::rc::Rc;
use std::sync::OnceLock;
use syn::{Ident, parse_quote, Path, Type, TypePath};

#[derive(Debug, Copy, Clone)]
pub(crate) struct BuilderVariant;

// !- Builder struct

#[derive(Clone)]
pub(crate) struct BuilderStruct {
    ty: Type,
    fields: Vec<BuilderField>,

    origin: Rc<OriginStruct>,
    override_struct: Rc<OverrideStruct>,

    consensus: Rc <OnceLock<BuilderConsensus>>,
    plan: Rc<OnceLock<BuilderPlan>>,
}
impl BuilderStruct {
    pub(crate) fn new(
        origin: Rc<OriginStruct>,
        override_struct: Rc<OverrideStruct>,
    ) -> Self {
        let fields = origin.fields
            .iter()
            //.filter(|f| !f.attrs.skip_builder_setter())
            .cloned()
            .map(BuilderField::new)
            .collect();

        let ty = util::build_type(origin.ids().builder_ident(), &origin.generics);
        Self {
            ty,
            fields,

            origin,
            override_struct,

            consensus: Rc::new(OnceLock::new()),
            plan: Rc::new(OnceLock::new()),
        }
    }
    pub(crate) fn ids(&self) -> &IdentPool {
        self.origin.ids()
    }
    pub(crate) fn ident(&self) -> &Rc<Ident> {
        self.ids().builder_ident()
    }
    pub(crate) fn ty(&self) -> &Type {
        &self.ty
    }
    pub(crate) fn all_fields_copy(&self) -> bool {
        self.origin.all_fields_copy() && self.override_struct.plan().copy
    }

    pub(crate) fn build_trait_path(&self) -> CanonPath {
        if self.plan().fallible_build {
            CanonPath::TryBuildConfig
        } else {
            CanonPath::BuildConfig
        }
    }
    pub(crate) fn make_build_fn_properties(config_ty: impl util::IntoTypePath) -> util::MaybeFallibleFn {
        util::MaybeFallibleFn::new(format_ident!("build"), config_ty)
    }
    pub(crate) fn build_fn_properties(&self) -> FnProperties {
        Self::make_build_fn_properties(self.ids().origin_path())
            .into_fallible_when(self.plan().fallible_build, self.ids().config_error_path().clone())
    }
    pub(crate) fn build_trait_fn_properties(&self) -> FnProperties {
        let error_ty: TypePath = parse_quote!(Self::Error);
        Self::make_build_fn_properties(self.ids().origin_path())
            .into_fallible_when(self.plan().fallible_build, error_ty)
    }
    pub(crate) fn build_consensus(&self) -> BuilderConsensus {
        let infallible_build = self.fields.iter()
            .map(BuilderField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        let fallible_build = self.fields.iter()
            .map(BuilderField::cast_ballot)
            .chain(std::iter::once(self.cast_ballot()))
            .collect();
        BuilderConsensus {
            infallible_build,
            fallible_build,
        }
    }
    pub(crate) fn consensus(&self) -> &BuilderConsensus {
        self.consensus.get_or_init(|| self.build_consensus())
    }
    pub(crate) fn plan(&self) -> &BuilderPlan {
        self.plan.get_or_init(|| self.consensus().as_plan())
    }
}
impl IdentifyVoter for BuilderStruct {
    fn voter_identity(&self) -> String {
        self.ident().to_string()
    }
}
impl CastVote<InfallibleBuild> for BuilderStruct {
    fn cast_ballot(&self) -> Vote<InfallibleBuild> {
        let proxy: Vote<InfallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}
impl CastVote<FallibleBuild> for BuilderStruct {
    fn cast_ballot(&self) -> Vote<FallibleBuild> {
        let proxy: Vote<FallibleBuild> = self.origin.cast_ballot();
        self.build_vote(*proxy.ballot()).with_evidence(proxy.into_evidence())
    }
}

impl BuilderStruct {
    // ! Struct-based item docs

    fn struct_docs(&self) -> Doc {
        let origin_ident = self.ids().origin_ident();
        let origin_link = self.ids().origin_link().struct_only();
        let origin_builder_link = self.ids().origin_link().method_explicit("builder");
        let builder_new_link = self.ids().builder_link().method_explicit("new");
        let override_link = self.ids().override_link().struct_only();

        Doc::new()
        + format!("Provides a builder used to configure {origin_link} at compile-time.")
        + ()
        + format!("Can be constructed using either {origin_builder_link} or {builder_new_link}.")
        + ()
        + format!("For run-time configuration of `{origin_ident}`, see {override_link}")
    }
}

impl BuilderStruct {
    // ! Struct token output

    fn derive_tokens(&self) -> TokenStream {
        let mut base = TokenStream::new();

        if self.all_fields_copy() {
            base.extend(quote! { #[derive(Copy)] });
        }

        let custom = self.origin.attrs.has_builder_derives().then(|| {
            let derives = &self.origin.attrs.builder_derives;
            quote!(#[derive(#(#derives),*)])
        });

        quote! {
            #base
            #custom
        }
    }
    fn attr_tokens(&self) -> Option<TokenStream> {
        self.origin.attrs.has_builder_attrs().then(|| {
            let attrs = &self.origin.attrs.builder_attrs;
            quote!(#(#[#attrs])*)
        })
    }
    fn override_field_ty_tokens(&self) -> TokenStream {
        let mut ty = self.override_struct.ty().to_token_stream();
        if !self.override_struct.has_required_fields() {
            let option_path = CanonPath::Option;
            ty = quote!(#option_path<#ty>);
        }
        ty
    }
    fn struct_tokens(&self) -> TokenStream {
        let docs = self.struct_docs();
        // attrs
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let derives = self.derive_tokens();
        let attrs = self.attr_tokens();
        // struct
        let struct_ident = self.ident();
        let origin_ty = &self.origin.ty;
        let override_ty = self.override_field_ty_tokens();
        let generics = &self.origin.generics;
        let where_clause = &generics.where_clause;

        quote! {
            #docs
            #cfg_attrs
            #doc_cfg_attrs
            #[derive(Debug, Clone)]
            #derives
            #attrs
            pub struct #struct_ident #generics
            #where_clause
            {
                pub(crate) inner: #origin_ty,
                pub(crate) override_conf: #override_ty,
            }
        }
    }
    fn new_fn_tokens(&self) -> TokenStream {
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

        let origin_ty = &self.origin.ty;
        let override_assign = if self.override_struct.has_required_fields() {
            quote!(override_conf.clone())
        } else {
            let option_path = CanonPath::Option;
            quote!(#option_path::None)
        };

        quote! {
            /// Constructs a new builder instance
            #[must_use]
            #[allow(clippy::redundant_field_names, reason = "https://github.com/rust-lang/rust-clippy/issues/17525")]
            pub fn new(#(#params),*) -> Self {
                Self {
                    override_conf: #override_assign,
                    inner: <#origin_ty>::new(#(#idents),*),
                }
            }
        }
    }
    fn setter_fns_tokens(&self) -> TokenStream {
        let mut out = TokenStream::new();
        for field in &self.fields {
            if !field.attrs().skip_builder_setter {
                out.extend(field.setter_tokens());
            }
            if field.attrs().nested.is_some() {
                out.extend(field.nested_builder_method_tokens());
            }
        }
        out
    }
    fn override_fns_tokens(&self) -> TokenStream {
        let option_path = CanonPath::Option;
        let override_ty = &self.override_struct.ty();
        let assign = if self.override_struct.has_required_fields() {
            quote!(override_conf)
        } else {
            quote!(#option_path::Some(override_conf))
        };
        let mut out = quote! {
            /// If supplied, will overwrite any values present in the provided
            /// override config.
            ///
            /// This happens as the last step within the
            ///[`build()`](self.build) method.
            ///
            /// Any previous calls to [`with_override()`](Self::with_override)
            /// in the builder chain will have no effect on the resulting
            /// config.
            ///
            /// The order in which this is chained with the builder's setter
            /// methods does not matter.
            #[must_use]
            pub fn with_override(
                mut self,
                override_conf: #override_ty,
            ) -> Self {
                self.override_conf = #assign;
                self
            }
        };

        // skip override clear fn if override has required fields
        if !self.override_struct.has_required_fields() {
            out.extend(quote! {
                /// Clears the override config previously set via
                /// [`with_override()`](Self::with_override).
                #[must_use]
                pub fn clear_override(mut self) -> Self {
                    self.override_conf = #option_path::None;
                    self
                }
            });
        }

        out
    }
    fn build_fn_impl_tokens(&self) -> TokenStream {
        let origin_ident = self.ids().origin_ident();
        let doc_headline: Doc = format!("Builds the [`{origin_ident}`]").into();

        let fallible = self.plan().fallible_build;
        let result_path = CanonPath::Result;
        let trait_path = self.build_trait_path();
        let fn_props = self.build_fn_properties();
        let build_fn_name = fn_props.name();
        let config_error_path = self.ids().config_error_path();

        let mut ret_ty = self.origin.ty.to_token_stream();
        if fallible {
            ret_ty = quote!(#result_path<#ret_ty, #config_error_path>);
        }

        quote! {
            #doc_headline
            ///
            /// Values present in an override config (if supplied) will replace
            /// corresponding assignments made using the builder.
            #[must_use]
            pub fn #build_fn_name(mut self) -> #ret_ty {
                <Self as #trait_path>::#build_fn_name(self)
            }
        }
    }
    fn build_trait_impl_tokens(&self) -> TokenStream {
        let fallible = self.plan().fallible_build;
        let config_ty = &self.origin.ty;
        let trait_path = self.build_trait_path();
        let error_ty = self.ids().config_error_path();
        let fn_props = self.build_trait_fn_properties();
        let build_fn_name = fn_props.name();
        let result_path = CanonPath::Result;
        let option_path = CanonPath::Option;

        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();

        let merge_fn_props = ConfigStruct::make_merge_fn_properties()
            .into_fallible_when(fallible, error_ty.clone());
        let merge_fn_name = &merge_fn_props.name();
        let (merge_trait, merge_expr_suffix) = if fallible  {
            (CanonPath::TryMergeOverride, Some(quote!(?)))
        } else {
            (CanonPath::MergeOverride, None)
        };
        let merge_expr = quote! {
            <Self::Config as #merge_trait>::#merge_fn_name(&mut self.inner, override_conf)#merge_expr_suffix;
        };
        let merge_expr = if self.override_struct.has_required_fields() {
            quote! {
                let override_conf = self.override_conf;
                #merge_expr
            }
        } else {
            quote! {
                if let #option_path::Some(override_conf) = self.override_conf {
                    #merge_expr
                }
            }
        };

        // assign validate() call
        let validate_call = self.origin.attrs.validate_fn.as_ref().map(|val_fn|
            quote!(#val_fn(&self.inner)?;)
        );

        let ret_expr = if fallible {
            quote!(#result_path::Ok(self.inner))
        } else {
            quote!(self.inner)
        };
        let error_expr = self.plan().fallible_build.then(||
            quote!(type Error = #error_ty;)
        );

        let ret_ty = if fallible {
            quote!(#result_path<Self::Config, Self::Error>)
        } else {
            quote!(Self::Config)
        };

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #trait_path for #ident #ty_generics #where_clause {
                type Config = #config_ty;
                #error_expr

                fn #build_fn_name(mut self) -> #ret_ty {
                    #merge_expr

                    #validate_call

                    #ret_expr
                }
            }
        }
    }
    fn default_impl_tokens(&self) -> TokenStream {
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        util::bare_default_impl_tokens(self.ident(), &self.origin.generics, &cfg_attrs, &doc_cfg_attrs)
    }
    fn impl_tokens(&self) -> TokenStream {
        let struct_ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let new_fn = self.new_fn_tokens();
        let setter_fns = self.setter_fns_tokens();
        let override_fns = self.override_fns_tokens();
        let build_fn = self.build_fn_impl_tokens();

        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            impl #impl_generics #struct_ident #ty_generics #where_clause {
                #new_fn
                #setter_fns
                #override_fns
                #build_fn
            }
        }
    }
}
impl ToTokens for BuilderStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.struct_tokens());
        tokens.extend(self.impl_tokens());

        // add default impl, if supported
        if !self.origin.has_required_fields() &&
        !self.override_struct.has_required_fields() {
            tokens.extend(self.default_impl_tokens());
        }

        tokens.extend(self.build_trait_impl_tokens());
    }
}

// !- Builder struct field

pub(crate) type BuilderField = VariantField<BuilderVariant>;

impl CastVote<InfallibleBuild> for BuilderField {
    fn cast_ballot(&self) -> Vote<InfallibleBuild> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}
impl CastVote<FallibleBuild> for BuilderField {
    fn cast_ballot(&self) -> Vote<FallibleBuild> {
        // proxy vote to origin field
        self.origin.cast_ballot()
    }
}

impl BuilderField {
    // ! Field-based doc segments

    pub(crate) fn doc_unset(&self) -> Option<Doc> {
        self.origin.unset_ident().map(|unset_ident| {
            let unset_link = self.ids().override_link().method(&unset_ident);

            Doc::new().h1("Revert at run-time")
            + ()
            + format!(
                "Has run-time support for reverting back to the provided default (**`{}`**)",
                self.origin.default().unwrap().to_pretty_string(),
            )
            + ()
            + format!("See {unset_link} for more information")
        })
    }
}

impl BuilderField {
    // ! Field-based item docs

    pub(crate) fn setter_docs(&self) -> AppendDoc {
        AppendDoc::from(self.origin.passthrough_attrs().doc().clone())
        + self.doc_unset()
        + Doc::new().separator()
        + self.doc_default_val()
    }
    pub(crate) fn nested_builder_method_docs(&self) -> Option<Doc> {
        self.attrs().nested().map(|nested| {
            let nested_ids = self.origin.nested_ids().unwrap();
            let config_link = nested_ids.origin_link().struct_only();
            let builder_link = nested_ids.builder_link().struct_only();
            let override_link = nested_ids.override_link().struct_only();

            let mut docs = Doc::new()
            + format!("Allows for inline configuration of nested {config_link}")
            + ();

            if nested.builder_default {
                docs += Doc::new()
                + format!("Closure has a single {builder_link} parameter and returns the builder after configured.");
            } else {
                docs += Doc::new()
                + format!("Closure takes no parameters, and returns a compile-time configured {builder_link}.");
            }
            docs + Doc::new()
            + ()
            + format!("The {builder_link} returned by the closure will automatically be built (and associated {override_link} merged).")
        })
    }
}

impl BuilderField {
    // ! Field-based token output
    pub(super) fn setter_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.origin.ty;
        let docs = self.setter_docs();
        let cfg_attrs = self.cfg_tokens();
        let doc_cfg_attrs = self.doc_cfg_tokens();
        quote! {
            #docs
            #cfg_attrs
            #doc_cfg_attrs
            #[must_use]
            pub fn #ident(mut self, val: #ty) -> Self {
                self.inner.#ident = val;
                self
            }
        }
    }
    pub(crate) fn nested_builder_method_tokens(&self) -> TokenStream {
        fn gen_method(field: &BuilderField, is_default_fn: bool) -> TokenStream {
            let nested = field.attrs().nested().unwrap();
            if is_default_fn {
                assert!(nested.builder_default);
            }
            let ident = field.ident();
            let docs = field.nested_builder_method_docs();
            let nested_builder_path = field.origin.nested_ids().unwrap().builder_path();
            let cfg_attrs = field.cfg_tokens();
            let doc_cfg_attrs = field.doc_cfg_tokens();
            let mut method_basename = format_ident!("{ident}_builder");
            if is_default_fn {
                method_basename = format_ident!("{method_basename}_default");
            }

            // TODO: dedicated fn for nested builder method properties
            let self_path: Path = parse_quote!(Self);
            let nested_error = &nested.config_error;
            let fn_props = util::MaybeFallibleFn::new(method_basename, self_path.clone());
            let fn_props = if nested.build.is_fallible() {
                fn_props.into_fallible(nested_error.clone().unwrap())
            } else {
                fn_props.into_infallible()
            };
            let (fn_name, ret_sig) = fn_props.split_for_impl();

            let build_fn_props = field.origin.nested_build_fn_properties().expect("nested fields always have properties");
            let build_fn_name = build_fn_props.name();
            let build_fn_suffix = nested.build.is_fallible().then_some(quote!(?));
            let build_trait_path = nested.build.build_trait_path();
            let closure_param_ty = is_default_fn.then(||quote!(#nested_builder_path));
            let closure_param_val = is_default_fn.then(||quote!(<#nested_builder_path>::default()));

            let mut assign =  quote! {
                <#nested_builder_path as #build_trait_path>::#build_fn_name(builder)#build_fn_suffix
            };

            // wrap in some for option types
            if field.origin.is_option {
                let option_path = CanonPath::Option;
                assign = quote!(#option_path::Some(#assign));
            }

            let mut ret_expr = quote!(self);
            // wrap return expr in Ok if fallible
            if nested.build.is_fallible() {
                let result_path = CanonPath::Result;
                ret_expr = quote!(#result_path::Ok(#ret_expr));
            }

            // fixme: nested generics
            quote! {
                #docs
                #cfg_attrs
                #doc_cfg_attrs
                #[must_use]
                pub fn #fn_name<F>(mut self, f: F) #ret_sig
                where
                    F: Fn(#closure_param_ty) -> #nested_builder_path,
                {
                    // pass off builder to user closure body, allowing for configuration
                    let builder = f(#closure_param_val);
                    self.inner.#ident = #assign;
                    #ret_expr
                }
            }
        }

        assert!(self.attrs().nested.is_some());
        let nested = self.attrs().nested().unwrap();

        let standard_builder = gen_method(self, false);
        let default_builder = nested.builder_default.then(|| gen_method(self, true));
        quote! {
            #standard_builder
            #default_builder
        }
    }
}
