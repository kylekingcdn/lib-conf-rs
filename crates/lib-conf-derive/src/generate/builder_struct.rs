use crate::{
    generate::{OverrideStruct, util, VariantField},
    parse::OriginStruct,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::rc::Rc;
use syn::{Ident, Type};

#[derive(Debug, Copy, Clone)]
pub struct BuilderVariant;

// !- Builder struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub struct BuilderStruct {
    ident: Ident,
    ty: Type,
    fields: Vec<BuilderField>,

    origin: Rc<OriginStruct>,
    override_struct: Rc<OverrideStruct>,
}

impl BuilderStruct {
    pub fn new(
        origin: Rc<OriginStruct>,
        override_struct: Rc<OverrideStruct>,
    ) -> Self {
        let fields = origin.fields
            .iter()
            .filter(|f| !f.attrs.skip_builder_setter())
            .cloned()
            .map(BuilderField::new)
            .collect();

        let ident = Self::generate_builder_ident(&origin.ident);
        let ty = util::build_type(&ident, &origin.generics);
        Self {
            ident,
            ty,
            fields,

            origin,
            override_struct,
        }
    }
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn fields(&self) -> &Vec<BuilderField> {
        &self.fields
    }
    pub fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_required())
    }
    pub fn required_fields(&self) -> Vec<&BuilderField> {
        self.fields.iter().filter(|f| f.is_required()).collect()
    }
    pub fn optional_fields(&self) -> Vec<&BuilderField> {
        self.fields.iter().filter(|f| f.is_optional()).collect()
    }
    pub fn generate_builder_ident(origin_ident: &Ident) -> Ident {
        format_ident!("{}Builder", origin_ident)
    }
}

// ! Builder struct generate methods

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
        let derives = self.derive_tokens();
        let attrs = self.attr_tokens();
        let struct_ident = &self.ident;
        let origin_ty = &self.origin.ty;
        let override_ty = self.override_field_ty_tokens();
        let generics = &self.origin.generics;
        let where_clause = &generics.where_clause;
        quote! {
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
                idents.push(field.ident.to_token_stream());
            }
        }
        
        let origin_ty = &self.origin.ty;
        let override_assign = if self.override_struct.has_required_fields() {
            quote!(override_conf.clone())
        } else {
            quote!(None)
        };

        quote! {
            /// Constructs a new builder instance
            #[must_use]
            pub fn new(#(#params),*) -> Self {
                Self {
                    override_conf: #override_assign,
                    inner: <#origin_ty>::new(#(#idents),*),
                }
            }
        }
    }
    fn setter_fns_tokens(&self) -> TokenStream {
        let fields = self.fields.iter().map(BuilderField::setter_tokens);
        TokenStream::from_iter(fields)
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
        let origin_ident = &self.origin.ident;
        let origin_ty = &self.origin.ty;
        let doc_headline = util::doc_line(
            format!("Builds the [`{origin_ident}`]")
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
            /// corresponding assigments made using the builder.
            #[must_use]
            pub fn build(self) -> #origin_ty {
                #merge_expr
            }
        }
    }
    fn default_impl_tokens(&self) -> TokenStream {
        util::bare_default_impl_tokens(self.ident(), &self.origin.generics)
    }
    fn impl_tokens(&self) -> TokenStream {
        let struct_ident = &self.ident;
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

// ! Builder struct field

pub type BuilderField = VariantField<BuilderVariant>;

impl BuilderField {
    pub fn is_optional(&self) -> bool {
        self.origin.is_optional()
    }
    pub fn is_required(&self) -> bool {
        !self.is_optional()
    }
    pub(super) fn setter_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let ty = &self.origin.ty;
        let docs = self.docs();
        quote! {
            #docs
            pub fn #ident(mut self, val: #ty) -> Self {
                self.inner.#ident = val;
                self
            }
        }
    }
}
