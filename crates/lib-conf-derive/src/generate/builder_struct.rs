use crate::{
    generate::{
        OverrideStruct, VariantField,
        util::{self, AppendDoc, Doc, Document},
    },
    parse::{IdentPool, OriginStruct},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::rc::Rc;
use syn::{Ident, Type};

#[derive(Debug, Copy, Clone)]
pub(crate) struct BuilderVariant;

// !- Builder struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct BuilderStruct {
    ty: Type,
    fields: Vec<BuilderField>,

    origin: Rc<OriginStruct>,
    override_struct: Rc<OverrideStruct>,
}
impl BuilderStruct {
    pub(crate) fn new(
        origin: Rc<OriginStruct>,
        override_struct: Rc<OverrideStruct>,
    ) -> Self {
        let fields = origin.fields
            .iter()
            .filter(|f| !f.attrs.skip_builder_setter())
            .cloned()
            .map(BuilderField::new)
            .collect();

        let ty = util::build_type(origin.ids().builder_ident(), &origin.generics);
        Self {
            ty,
            fields,

            origin,
            override_struct,
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
}

// ! Struct-based doc segments

impl BuilderStruct {
}

// ! Struct-based item docs

impl BuilderStruct {
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

// ! Struct token output

impl BuilderStruct {
    fn derive_tokens(&self) -> Option<TokenStream> {
        self.origin.attrs.has_builder_derives().then(|| {
            let derives = &self.origin.attrs.builder_derives;
            quote!(#[derive(#(#derives),*)])
        })
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
            ty = quote!(Option<#ty>);
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
            quote!(None)
        };

        // #[allow(clippy::redundant_field_names)] added for:
        //   https://github.com/rust-lang/rust-clippy/issues/17525
        quote! {
            /// Constructs a new builder instance
            #[must_use]
            #[allow(clippy::redundant_field_names)]
            pub fn new(#(#params),*) -> Self {
                Self {
                    override_conf: #override_assign,
                    inner: <#origin_ty>::new(#(#idents),*),
                }
            }
        }
    }
    fn setter_fns_tokens(&self) -> TokenStream {
        self.fields.iter().map(BuilderField::setter_tokens).collect()
    }
    fn override_fns_tokens(&self) -> TokenStream {
        let override_ty = &self.override_struct.ty();
        let assign = if self.override_struct.has_required_fields() {
            quote!(override_conf)
        } else {
            quote!(Some(override_conf))
        };
        let mut out = quote! {
            /// If supplied, will overwrite any values present in the provided
            /// override config.
            ///
            /// This happens as the last step within the
            ///[`build()`](Self::build) method.
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
                    self.override_conf = None;
                    self
                }
            });
        }

        out
    }
    fn build_fn_tokens(&self) -> TokenStream {
        let origin_ident = self.ids().origin_ident();
        let origin_ty = &self.origin.ty;
        let doc_headline = util::doc_line(
            format!("Builds the [`{origin_ident}`]"), true
        );

        let merge_expr =
        if self.override_struct.has_required_fields() {
            quote! {
                self.inner + self.override_conf
            }
        } else {
            quote! {
                if let Some(override_conf) = self.override_conf {
                    self.inner + override_conf
                } else {
                    self.inner
                }
            }
        };

        quote! {
            #doc_headline
            ///
            /// Values present in an override config (if supplied) will replace
            /// corresponding assignments made using the builder.
            #[must_use]
            pub fn build(self) -> #origin_ty {
                #merge_expr
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
        let build_fn = self.build_fn_tokens();

        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();

        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.struct_tokens());
        tokens.extend(self.impl_tokens());
        if !self.origin.has_required_fields() &&
        !self.override_struct.has_required_fields() {
            tokens.extend(self.default_impl_tokens());
        }
    }
}

// !- Builder struct field

pub(crate) type BuilderField = VariantField<BuilderVariant>;

// ! Field-based doc segments

impl BuilderField {
    pub(crate) fn doc_unset(&self) -> Option<Doc> {
        self.origin.unset_ident().map(|unset_ident| {
            let unset_link = self.ids().override_link().method(&unset_ident);
            let _alias_ref = format!("{unset_link}#unset-fields");
            let doc = Doc::new()
                .h1("Revert at run-time")
                .newl();
                // TODO: add h1, h2, h3, etc
            doc + "# Runtime "

            // let override_ident = self.ids().override_ident();
            // let link_name = format!("{override_ident}::{unset_ident}()");
            // let link_ref = format!("method@{override_ident}::{unset_ident}");
            // format!("[{link_name}]({link_ref})"); // !- TODO: helper fns
            //format!("Supports reverting to default at run-time via {unset_link}")
        })
    }
}

// ! Field-based item docs

impl BuilderField {
    pub(crate) fn setter_docs(&self) -> AppendDoc {
        AppendDoc::from(self.origin.passthrough_attrs().doc().clone())
        + self.doc_unset()
        + Doc::new().separator()
        + self.doc_default_val()
    }
}

// ! Field-based token output

impl BuilderField {
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
}
