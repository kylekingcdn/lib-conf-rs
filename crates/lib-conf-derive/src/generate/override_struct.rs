use crate::{
    generate::{util, VariantField},
    parse::{OriginField, OriginStruct},
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
    pub fn ident(&self) -> &Ident {
        &self.ident
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn fields(&self) -> &Vec<OverrideField> {
        &self.fields
    }
    pub fn has_required_fields(&self) -> bool {
        self.fields.iter().any(|f| f.is_required())
    }
    pub fn required_fields(&self) -> Vec<&OverrideField> {
        self.fields.iter().filter(|f| f.is_required()).collect()
    }
    pub fn optional_fields(&self) -> Vec<&OverrideField> {
        self.fields.iter().filter(|f| f.is_optional()).collect()
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
    fn derive_tokens(&self) -> Option<TokenStream> {
        self.origin.attrs.has_override_derives().then(|| {
            let derives = &self.origin.attrs.override_derives;
            quote!(#[derive(#(#derives),*)])
        })
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

            out.extend(quote! {
                #[serde(default)]
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
            #[derive(Debug, Clone, ::serde::Deserialize)]
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
    pub fn is_optional(&self) -> bool {
        !self.is_required()
    }
    pub fn is_required(&self) -> bool {
        self.attrs().override_required
    }
    fn attr_tokens(&self) -> Option<TokenStream> {
        self.origin.has_override_attrs().then(|| {
            let attrs = &self.origin.override_attrs;
            quote!(#(#[#attrs])*)
        })
    }
    fn getter_ret_ty(&self) -> Type {
        // TODO: map String to &str
        // TODO: as_ref for option?
        if let Some(ref from_ty) = self.attrs().override_from {
            let mut inner = from_ty.to_token_stream();
            if !self.attrs().override_required {
                inner = quote!(Option::<#inner>)
            }
            parse_quote!(&#inner)
        } else if self.attrs().override_required {
            self.origin.as_required_return_type()
        } else {
            self.origin.as_optional_return_type()
        }
    }
    fn getter_ret_expr(&self) -> TokenStream {
        let ident = self.ident();
        match self.attrs().copy && self.attrs().override_from.is_none() {
            true  => quote!( self.#ident),
            false => quote!(&self.#ident),
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
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = self.ident();
        let ty = if let Some(ref from_ty) = self.attrs().override_from {
            let mut inner = from_ty.to_token_stream();
            if !self.attrs().override_required {
                inner = quote!(Option::<#inner>)
            }
            parse_quote!(#inner)
        } else if self.attrs().override_required {
            self.origin.as_required_type()
        } else {
            self.origin.as_optional_type()
        };

        // regular assign field
        let field_docs = self.docs();
        let attrs = self.attr_tokens();
        let field = quote! {
            #field_docs
            #attrs
            pub(crate) #ident: #ty,
        };
        tokens.extend(field);

        // unset field
        if let Some(unset_ident) = self.origin.unset_ident() {
            let aliases = self.origin.unset_aliases();
            let unset_field = quote! {
                #[serde(default, #(alias=#aliases),*)]
                /// flag allowing for reverting a builder-configured
                /// setting at runtime
                pub(crate) #unset_ident: bool,
            };
            tokens.extend(unset_field);
        }
    }
}
