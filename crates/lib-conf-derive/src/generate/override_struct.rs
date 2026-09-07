use crate::{
    generate::{util, VariantField},
    parse::{self, OriginField, OriginStruct},
};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::rc::Rc;
use syn::{Ident, parse_quote, Type};

#[derive(Debug, Copy, Clone)]
pub struct OverrideVariant;

// !- Override struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub struct OverrideStruct {
    ident: Ident,
    ty: Type,
    fields: Vec<OverrideField>,

    origin: Rc<OriginStruct>,
    phantom_fields: Vec<Rc<OriginField>>,
}
impl OverrideStruct {
    pub fn new(origin: Rc<OriginStruct>) -> Self {
        let with_phantoms = origin.use_phantom_fields();
        let mut fields = Vec::new();
        let mut phantom_fields = Vec::new();
        for field in &origin.fields {
            if !field.attrs.skip_override_field() {
                fields.push(OverrideField::new(field.clone()));
            } else if with_phantoms {
                phantom_fields.push(field.clone());
            }
        }
        let ident = Self::generate_ident(&origin.ident, origin.suffix);
        let ty = util::build_type(&ident, &origin.generics);
        Self {
            ident,
            ty,
            fields,

            origin,
            phantom_fields,
        }
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn has_required_fields(&self) -> bool {
        self.fields.iter().any(super::VariantField::is_required)
    }
    fn generate_ident(origin_ident: &Ident, suffix: &'static str) -> Ident {
        let ident_str = origin_ident.to_string();
        assert!(ident_str.ends_with(suffix));

        let suf_index = ident_str.rfind(suffix).unwrap();
        let prefix = &ident_str[..suf_index];
        format_ident!("{prefix}Override{suffix}")
    }
}
// ! Override struct generate methods
impl OverrideStruct {
    fn derive_tokens(&self) -> TokenStream {
        #[cfg(not(feature = "serde"))]
        let base = quote! { #[derive(Debug, Clone)] };
        #[cfg(feature = "serde")]
        let base = quote! { #[derive(Debug, Clone, ::serde::Deserialize)] };

        let custom = self.origin.attrs.has_override_derives().then(|| {
            let derives = &self.origin.attrs.override_derives;
            quote!(#[derive(#(#derives),*)])
        });
        quote! {
            #base
            #custom
        }
    }
    fn attr_tokens(&self) -> Option<TokenStream> {
        self.origin.attrs.has_override_attrs().then(|| {
            let attrs = &self.origin.attrs.override_attrs;
            quote!(#(#[#attrs])*)
        })
    }
    fn phantom_fields_tokens(&self) -> TokenStream {
        let mut out = TokenStream::new();
        for field in &self.phantom_fields {
            let ident = field.phantom_ident();
            let ty = &field.ty;

            #[cfg(feature = "serde")]
            out.extend(quote!(#[serde(default)]));

            out.extend(quote! {
                #ident: ::std::marker::PhantomData<#ty>,
            });
        }
        out
    }
    fn struct_tokens(&self) -> TokenStream {
        let derives = self.derive_tokens();
        let attrs = self.attr_tokens();
        let struct_ident = &self.ident;
        let fields: Vec<_> = self.fields
            .iter()
            .map(ToTokens::into_token_stream)
            .collect();
        let phantom_fields = self.phantom_fields_tokens();
        let generics = &self.origin.generics;
        let where_clause = &generics.where_clause;
        quote! {
            #derives
            #attrs
            pub struct #struct_ident #generics
            #where_clause
            {
                #(#fields)*

                #phantom_fields
            }
        }
    }
    fn getter_fns_tokens(&self) -> TokenStream {
        // TODO: add unset getter
        let fields = self.fields
            .iter()
            .map(OverrideField::getter_tokens);
        TokenStream::from_iter(fields)
    }
    fn impl_tokens(&self) -> TokenStream {
        let ident = &self.ident;
        let getters = self.getter_fns_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();
        quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                #getters
            }
        }
    }
}
impl ToTokens for OverrideStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.struct_tokens());
        tokens.extend(self.impl_tokens());
    }
}

// ! Override struct fields

pub type OverrideField = VariantField<OverrideVariant>;

impl OverrideField {
    pub fn is_required(&self) -> bool {
        self.attrs().override_required
    }
    fn attr_tokens(&self) -> Option<TokenStream> {
        self.origin.has_override_attrs().then(|| {
            let attrs = &self.origin.override_attrs;
            quote!(#(#[#attrs])*)
        })
    }
    pub fn ty(&self) -> Type {
        let mut ty = match self.attrs().override_from {
            Some(ref ty) => Type::Path(ty.clone()),
            None => self.origin.flat_ty.clone(),
        };
        if !self.attrs().override_required {
            ty = parse_quote!(Option<#ty>);
        }
        ty
    }
    fn getter_ret_ty(&self) -> Type {
        // copy permitted, direct return
        if self.attrs().copy && self.attrs().override_from.is_none() {
            if self.attrs().override_required {
                return self.origin.flat_ty.clone();
            }
            let inner = &self.origin.flat_ty;
            return parse_quote!(Option<#inner>);
        }

        let ty = self.ty();
        if let Some(inner_ty) = parse::util::unwrap_option(&ty) {
            if parse::util::is_string(inner_ty) {
                // type is Option<String>
                parse_quote!(Option<&str>)
            } else {
                // type is Option<_>
                parse_quote!(Option<&#inner_ty>)
            }
        } else {
            if parse::util::is_string(&ty) {
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
        if self.attrs().copy && self.attrs().override_from.is_none() {
            return inner;
        }

        let ty = self.ty();
        if let Some(inner_ty) = parse::util::unwrap_option(&ty) {
            if parse::util::is_string(inner_ty) {
                // type is Option<String>
                quote!(#inner.as_ref().map(|s| s.as_str()))
            } else {
                // type is Option<_>
                quote!(#inner.as_ref())
            }
        } else {
            if parse::util::is_string(&ty) {
                // type is String
                quote!(#inner.as_str())
            } else {
                // Any other type
                quote!(&#inner)
            }
        }
    }
    pub(super) fn getter_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let docs = self.docs();
        let ty = self.getter_ret_ty();
        let ret = self.getter_ret_expr();

        quote! {
            #docs
            #[must_use]
            pub fn #ident(&self) -> #ty {
                #ret
            }
        }
    }
}
impl ToTokens for OverrideField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = self.ident();
        let ty = self.ty();

        #[cfg(feature = "serde")]
        let vis = quote!(pub(crate));
        #[cfg(not(feature = "serde"))]
        let vis = quote!(pub);

        // regular assign field
        let field_docs = self.docs();
        let attrs = self.attr_tokens();
        let field = quote! {
            #field_docs
            #attrs
            #vis #ident: #ty,
        };
        tokens.extend(field);

        // unset field
        if let Some(unset_ident) = self.origin.unset_ident() {
            #[cfg(feature = "serde")]
            {
                let aliases = self.origin.unset_aliases();
                tokens.extend(quote! {
                    #[serde(default, #(alias=#aliases),*)]
                });
            }

            tokens.extend(quote! {
                /// flag allowing for reverting a builder-configured
                /// setting at runtime
                #vis #unset_ident: bool,
            });
        }
    }
}
