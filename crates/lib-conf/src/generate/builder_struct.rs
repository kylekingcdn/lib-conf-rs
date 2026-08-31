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
    pub fn generate_builder_ident(origin_ident: &Ident) -> Ident {
        format_ident!("{}Builder", origin_ident)
    }
}

// ! Builder struct generate methods

impl BuilderStruct {
    fn struct_tokens(&self) -> TokenStream {
        let struct_ident = &self.ident;
        let origin_ty = &self.origin.ty;
        let override_ty = &self.override_struct.ty();
        let generics = &self.origin.generics;

        quote! {
            #[derive(Debug, Clone)]
            pub struct #struct_ident #generics {
                pub(crate) inner: #origin_ty,
                pub(crate) override_conf: Option<#override_ty>,
            }
        }
    }
    fn new_fn_tokens(&self) -> TokenStream {
        // TODO: handle required fields
        let origin_ty = &self.origin.ty;
        quote! {
            fn new() -> Self {
                Self {
                    inner: <#origin_ty>::default(),
                    override_conf: None,
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
        quote! {
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
                self.override_conf = Some(override_conf);
                self
            }
            /// Clears the override config previously set via
            /// [`with_override()`](Self::with_override).
            #[must_use]
            pub fn clear_override(mut self) -> Self {
                self.override_conf = None;
                self
            }
        }
    }
    fn build_fn_tokens(&self) -> TokenStream {
        let origin_ident = &self.origin.ident;
        let origin_ty = &self.origin.ty;
        let doc_headline = util::doc_line(
            format!("Builds the [`{origin_ident}`]")
        );

        quote! {
            #doc_headline
            ///
            /// Values present in an override config (if supplied) will replace
            /// corresponding assigments made using the builder.
            #[must_use]
            pub fn build(self) -> #origin_ty {
                if let Some(override_conf) = self.override_conf {
                    self.inner + override_conf
                } else {
                    self.inner
                }
            }
        }
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
    }
}

// ! Builder struct field

pub type BuilderField = VariantField<BuilderVariant>;

impl BuilderField {
    pub(super) fn setter_tokens(&self) -> TokenStream {
        let ident = &self.ident();
        let ty = &self.source.ty;
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
