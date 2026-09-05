use crate::{
    generate::{BuilderStruct, OverrideStruct, util, VariantField},
    parse::OriginStruct,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::rc::Rc;
use syn::Ident;

#[derive(Debug, Copy, Clone)]
pub struct ConfigVariant;

// !- Config struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub struct ConfigStruct {
    fields: Vec<ConfigField>,

    origin: Rc<OriginStruct>,
    override_struct: Rc<OverrideStruct>,
    builder_struct: Rc<BuilderStruct>,
}
impl ConfigStruct {
    pub fn new(origin: Rc<OriginStruct>, override_struct: Rc<OverrideStruct>, builder_struct: Rc<BuilderStruct>) -> Self {
        let fields = origin.fields
            .iter().cloned()
            .map(ConfigField::new)
            .collect();
        Self {
            fields,

            origin,
            override_struct,
            builder_struct,
        }
    }
    pub fn ident(&self) -> &Ident {
        &self.origin.ident
    }
}
// ! Config struct generate methods
impl ConfigStruct {
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
                if let Some(ref _mapped_ty) = field.attrs().override_from {
                    if let Some(ref mapped_via) = field.attrs().override_via {
                        inner = quote!(<#mapped_via>::from(#inner));
                    }
                    inner = quote!(#inner.into());
                }
                // origin is optional, wrap in Some
                if field.origin.is_optional() {
                    inner = quote!(Some(#inner));
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
            
            fields.push(quote!(#ident: #assign));
        }
        quote! {
            #[must_use]
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
                idents.push(field.ident.to_token_stream());
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
    fn getter_fns_tokens(&self) -> TokenStream {
        TokenStream::from_iter(
            self.fields
                .iter()
                .filter(|f| !f.attrs().skip_config_getter())
                .map(ConfigField::getter_tokens)
        )
    }
    fn merge_fn_tokens(&self) -> TokenStream {
        let override_ty = &self.override_struct.ty();
        let conf_var_ident = format_ident!("override_conf");
        let fields: Vec<_> = self.fields
            .iter()
            .filter(|f| !f.attrs().skip_override_field())
            .map(|f| f.merge_override_tokens(&conf_var_ident))
            .collect();
        quote! {
            /// Applies all values from override (which are `Some(_)`) to self
            pub fn merge_with_override(&mut self, #conf_var_ident: #override_ty) {
                #(#fields)*
            }
        }
    }
    fn impl_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let new_fn = self.new_fn_tokens();
        let builder_fn = self.builder_fn_tokens();
        let getter_fns = self.getter_fns_tokens();
        let merge_fns = self.merge_fn_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();
        quote! {
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                #new_fn
                #builder_fn
                #getter_fns
                #merge_fns
            }
        }
    }
    fn default_impl_tokens(&self) -> TokenStream {
        util::bare_default_impl_tokens(self.ident(), &self.origin.generics)
    }
    fn add_impl_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let override_ty = &self.override_struct.ty();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();

        quote! {
            #[automatically_derived]
            impl #impl_generics ::std::ops::Add<#override_ty> for #ident #ty_generics #where_clause {
                type Output = Self;

                fn add(self, other: #override_ty) -> Self {
                    let mut merged = self;
                    merged.merge_with_override(other);
                    merged
                }
            }
        }
    }
}
impl ToTokens for ConfigStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.impl_tokens());
        
        if !self.override_struct.has_required_fields()
        && !self.origin.has_required_fields() {
            // all config + override fields must be optional for default impl
            tokens.extend(self.default_impl_tokens());
        }
        tokens.extend(self.add_impl_tokens());
    }
}

// ! Config struct fields

pub type ConfigField = VariantField<ConfigVariant>;

impl ConfigField {
    fn getter_ret_ty(&self) -> TokenStream {
        let src_ty = &self.origin.ty;
        // TODO: map String to &str
        // TODO: as_ref for option?
        match self.attrs().copy {
            true  => quote!( #src_ty),
            false => quote!(&#src_ty),
        }
    }
    fn getter_ret_expr(&self) -> TokenStream {
        let ident = self.ident();
        match self.attrs().copy {
            true =>  quote!( self.#ident),
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
    pub(super) fn merge_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        let ident = self.ident();

        let temp_assign = quote!(let val = #override_var_ident.#ident.clone(););
        
        let mut assign = if let Some(_from_ty) = &self.attrs().override_from {
            let mut inner = quote!(val);
            if let Some(via_ty) = &self.attrs().override_via {
                inner = quote!(<#via_ty>::from(#inner));
            }
            quote!(#inner.into())
        } else {
            quote!(val)
        };
        // wrap in Some if Option
        if self.origin.is_option {
            assign = quote!(Some(#assign));
        }
        // origin type is guaranteed option if an unset ident is given
        let unsetter = self.origin.unset_ident().map(|unset_ident| {
            let default = self.origin.default().unwrap();
            quote! {
                if #override_var_ident.#unset_ident {
                    self.#ident = #default
                } else
            }
        });
        // wrapped in brackets to allow for conditional preceeding if {..} else
        let mut setter = quote! {
            {
                self.#ident = #assign;
            }
        };
        // wrap setter in if let Some..
        if !self.attrs().override_required {
            setter = quote! {
                if let Some(val) = val #setter
            }
        }

        quote! {
            #temp_assign
            #unsetter
            #setter
        }
    }
}
