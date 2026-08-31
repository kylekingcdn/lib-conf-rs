use crate::{
    generate::{BuilderStruct, OverrideStruct, util, VariantField},
    parse::OriginStruct,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use std::rc::Rc;
use syn::{Ident, Type};

#[derive(Debug, Copy, Clone)]
pub struct ConfigVariant;

// !- Config struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub struct ConfigStruct {
    ty: Type,
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
        let ty = util::build_type(&origin.ident, &origin.generics);

        Self {
            ty,
            fields,

            origin,
            override_struct,
            builder_struct,
        }
    }
    pub fn ident(&self) -> &Ident {
        &self.origin.ident
    }
    pub fn ty(&self) -> &Type {
        &self.ty
    }
    pub fn fields(&self) -> &Vec<ConfigField> {
        &self.fields
    }
}
// ! Config struct generate methods
impl ConfigStruct {
    fn new_fn_tokens(&self) -> TokenStream {
        // TODO: handle required fields
        let fields: Vec<_> = self.fields.iter().map(ConfigField::assign_default_tokens).collect();
        quote! {
            fn new() -> Self {
                Self {
                    #(#fields)*
                }
            }
        }
    }
    fn builder_fn_tokens(&self) -> TokenStream {
        let builder_ty = self.builder_struct.ty();
        // TODO: handle required fields
        quote! {
            /// Creates a new config builder
            #[must_use]
            pub fn builder() -> #builder_ty {
                <#builder_ty>::new()
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
        let ident = &self.ident();
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
        let ident = &self.ident();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();
        // TODO: handle required fields
        quote! {
            #[automatically_derived]
            impl #impl_generics Default for #ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self::new()
                }
            }
        }
    }
    fn add_impl_tokens(&self) -> TokenStream {
        let ident = &self.ident();
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
        let struct_impl = self.impl_tokens();
        let default_impl = self.default_impl_tokens();
        let add_impl = self.add_impl_tokens();
        let out = quote! {
            #struct_impl
            #default_impl
            #add_impl
        };
        tokens.extend(out);
    }
}

// ! Config struct fields

pub type ConfigField = VariantField<ConfigVariant>;

impl ConfigField {
    fn getter_ret_ty(&self) -> TokenStream {
        let src_ty = &self.source.ty;
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
    pub(super) fn assign_default_tokens(&self) -> TokenStream {
        assert!(self.source.is_optional());
        let ident = self.ident();
        let def = self.source.default().unwrap();

        quote!(#ident: #def,)
    }
    pub(super) fn merge_override_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        match self.attrs().override_required {
            true  => self.merge_override_required_tokens(override_var_ident),
            false => self.merge_override_optional_tokens(override_var_ident),
        }
    }
    fn merge_override_required_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        let ident = self.ident();
        let override_field = match self.attrs().copy {
            true  => quote!(#override_var_ident.#ident),
            false => quote!(#override_var_ident.#ident.clone()),
        };
        let assign = match self.source.is_option {
            true  => quote!(Some(#override_field)),
            false => override_field,
        };
        // source type is guaranteed option if an undet ident is given
        let unsetter = self.source.unset_ident().map(|unset_ident| {
            let default = self.source.default().unwrap();
            quote! {
                if #override_var_ident.#unset_ident {
                    self.#ident = #default
                } else
            }
        });

        // wrapped in brackets to allow for condutional preceeding if {..} else
        quote! {
            #unsetter
            {
                self.#ident = #assign;
            }
        }
    }
    fn merge_override_optional_tokens(&self, override_var_ident: &Ident) -> TokenStream {
        let ident = self.ident();
        let override_field = match self.attrs().copy {
            true  => quote!(#override_var_ident.#ident),
            false => quote!(#override_var_ident.#ident.clone()),
        };
        let assign = match self.source.is_option {
            true  => quote!(Some(val)),
            false => quote!(     val ),
        };
        // source type is guaranteed option if an undet ident is given
        let unsetter = self.source.unset_ident().map(|unset_ident| {
            let default = self.source.default().unwrap();
            quote! {
                if #override_var_ident.#unset_ident {
                    self.#ident = #default
                } else
            }
        });
        quote! {
            #unsetter
            if let Some(val) = #override_field {
                self.#ident = #assign;
            }
        }
    }
}
