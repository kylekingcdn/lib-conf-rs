use crate::{
    generate::{
        VariantField,
        util::{self, AppendDoc, DerivedLink, Doc, Document, Link, Table},
    },
    parse::{self, IdentPool, OriginField, OriginStruct},
};

#[cfg(feature = "serde")]
use {
    quote::format_ident,
    crate::generate::util::CodeBlock,
};

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use std::rc::Rc;
use syn::{Ident, parse_quote, Type};

#[derive(Debug, Copy, Clone)]
pub(crate) struct OverrideVariant;

// !- Override struct

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct OverrideStruct {
    ty: Type,
    fields: Vec<OverrideField>,

    origin: Rc<OriginStruct>,
    phantom_fields: Vec<Rc<OriginField>>,
}
impl OverrideStruct {
    pub(crate) fn new(origin: Rc<OriginStruct>) -> Self {
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
        let ty = util::build_type(origin.ids().override_ident(), &origin.generics);
        Self {
            ty,
            fields,

            origin,
            phantom_fields,
        }
    }
    pub(crate) fn ids(&self) -> &IdentPool {
        self.origin.ids()
    }
    pub(crate) fn ident(&self) -> &Rc<Ident> {
        self.ids().override_ident()
    }
    pub(crate) fn ty(&self) -> &Type {
        &self.ty
    }
    pub(crate) fn has_required_fields(&self) -> bool {
        self.fields.iter().any(OverrideField::is_required)
    }
    #[allow(dead_code)] // only used w/ serde feature
    pub(crate) fn has_optional_fields(&self) -> bool {
        self.fields.iter().any(OverrideField::is_optional)
    }
    pub(crate) fn has_unset_fields(&self) -> bool {
        self.fields.iter().any(|f| f.origin.with_unset_field())
    }
    pub(crate) fn has_conditional_fields(&self) -> bool {
        self.fields.iter().any(|f| f.origin.is_conditional())
    }
    pub(crate) fn has_conditional_unset_fields(&self) -> bool {
        self.fields.iter().any(|f| f.origin.with_unset_field() && f.origin.is_conditional())
    }
}


impl OverrideStruct {
    // ! Struct-based doc segments

    fn doc_headline(&self) -> String {
        let origin_link = self.ids().origin_link().struct_only();
        format!("Provides support for configuring {origin_link} at run-time.")
    }
    fn doc_features(&self) -> Doc {
        let override_ident = self.ids().override_ident();
        let origin_link = self.ids().origin_link().struct_only();
        let builder_link = self.ids().origin_link().struct_only();
        Doc::new()
        + format!("`{override_ident}` provides run-time support for:")
        + format!("- *Setting* {origin_link} fields")
        + format!("- *Overriding* compile-time assignments (made using {builder_link})")
        + self.origin.has_optional_fields().then_some(
            "- *Reverting* compile-time (or run-time) assignments back to provided defaults"
        )
    }
    #[cfg(feature = "serde")]
    fn doc_example(&self) -> CodeBlock {
        let ident = self.ident();
        let field_name = format_ident!("{}", &self.origin.attrs.example_field_name);
        CodeBlock::new("rust,ignore")
        + "#[derive(Debug, Clone, Deserialize)]"
        + "pub struct Conf {"
        + "    api: ApiConf,"
        + "    // .."
        + (!self.has_required_fields()).then_some(
          "    #[serde(default)]"
        )
        + format!(
          "    {field_name}: {ident}," // TODO: add struct opt for specifying field name
        )
        + "}"
    }
    fn doc_fields_table(&self) -> Option<Doc> {
        if self.fields.is_empty() {
            return None;
        }

        let has_required = self.has_required_fields();
        let has_conditional = self.has_conditional_fields();

        // Add a notes column if required or conditional fields exist
        let doc = if has_required || has_conditional {
            let mut table = Table::new_3("Name", "Type", "Notes");

            for field in &self.fields {
                let field_ident = field.ident();
                let ty = field.flat_ty_pretty();
                let note = field.doc_notes();
                table += (format!("`{field_ident}`"), format!("`{ty}`"), note);
            }
            Doc::new()
            + table
            + ()
            + "**Notes:**"
            + has_required.then_some(
                "- ❗️: This field is mandatory."
            )
            + has_conditional.then_some(
                "- ❔: Inclusion of this field is conditional. Check the associated method for more information."
            )
            + (!has_required).then_some(
                "- All override fields are optional"
            )
        } else {
            let mut table = Table::new_2("Name", "Type");

            for field in &self.fields {
                let field_ident = field.ident();
                let ty = field.flat_ty_pretty();

                table += (format!("`{field_ident}`"), format!("`{ty}`"));
            }
            table.into()
        };

        Some(doc)
    }
    fn doc_unset(&self) -> Option<Doc> {
        let (example_ident, example_default) = self.doc_unset_example()?;
        let origin_link = self.ids().origin_link().struct_only();

        let mut doc = Doc::new().h2("Revert to default")
        + ()
        + format!("For each of {origin_link}'s optional fields, there is a corresponding **`bool`** with a name in the form of `*_unset`.")
        + ()
        + "If set to `true`, the associated field will have its value reverted to the provided default, e.g."
        + format!("- if `{example_ident}_unset` deserializes to `true`:")
        + format!("  - `{example_ident}` will be reverted back to the provided default: `{example_default}`");


        doc += Doc::new().newl().h3("Unset fields").newl()
        + "The following fields are supported:"
        + ()
        + self.doc_unset_fields_table();

        #[cfg(feature = "serde")]
        {
            doc += Doc::new()
            + ()
            + Self::doc_unset_aliases(&example_ident);
        }

        Some(doc)
    }
    fn doc_unset_example(&self) -> Option<(Ident, String)> {
        for field in &self.fields {
            if field.origin.with_unset_field() {
                return Some((
                    field.ident().clone(),
                    field.origin.default().unwrap().to_pretty_string(),
                ));
            }
        }
        None
    }
    fn doc_unset_fields_table(&self) -> Doc {
        assert!(self.has_unset_fields());

        let has_conditional = self.has_conditional_unset_fields();

        // Add a notes column if conditional fields exist
        if has_conditional {
            let mut table = Table::new_2("Name", "Notes");

            for field in &self.fields {
                if let Some(ident) = field.origin.unset_ident() {
                    let note = field.has_cfg_attrs().then_some("<span align=\"center\">❔</span>");
                    table += (format!("`{ident}`"), note.unwrap_or_default());
                }
            }
            Doc::new()
            + table
            + ()
            + "**Notes:**"
            + has_conditional.then_some(
                "- ❔: Inclusion of this field is conditional. Check the associated method for more information."
            )
            + "- All `unset` fields are optional"
        } else {
            let mut table = Table::new_1("Name");

            for field in &self.fields {
                if let Some(ident) = field.origin.unset_ident() {
                    table += format!("`{ident}`");
                }
            }
            table.into()
        }
    }
    #[cfg(feature = "serde")]
    fn doc_unset_aliases(example_ident: &Ident) -> Doc {
        let mut alias_table = Table::new_1("Alias");
        let mut example_list = Doc::new();

        for alias in **parse::UNSET_ALIASES {
            alias_table += format!("`{alias}`");
            example_list += format!("- `{example_ident}_{alias}`");
        }

        Doc::new().h3("Unset aliases")
        + ()
        + "Defaults can also be restored using the following aliases"
        + ()
        + alias_table
        + ()
        + format!("For example, `{example_ident}` could be reset using either `{example_ident}_unset`, or any of the following:")
        + example_list
    }
}


impl OverrideStruct {
    // ! Struct-based item docs

    fn struct_docs(&self) -> Doc {
        let mut docs = Doc::new()
        + self.doc_headline();

        #[cfg(feature = "serde")]
        {
            let override_ident = self.ids().override_ident();
            docs += Doc::new().newl()
            + "A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time."
            + ()
            + format!("`{override_ident}` is provided for just this case.");
        }
        #[cfg(not(feature = "serde"))]
        {
            docs += Doc::new().newl()
            + "**Note**: `serde` support is disabled. This struct must be manually initialized."
            + "It is provided to allow for retained override/unset support";
        }

        docs = docs.separator()
        + self.doc_features();

        // add deserialization example when serde support is enabled
        #[cfg(feature = "serde")]
        {
            docs += Doc::new().h1("Example").newl()
            + self.doc_example();
        }

        if let Some(fields_table) = self.doc_fields_table() {
            docs += Doc::new().newl().h1("Fields").newl()
            + fields_table;

            if let Some(unset_docs) = self.doc_unset() {
                docs += Doc::new().newl()
                + unset_docs;
            }
        }

        docs
    }
}


impl OverrideStruct {
    // ! Struct token output

    fn derive_tokens(&self) -> TokenStream {
        let mut base = TokenStream::new();

        base.extend(quote! { #[derive(Debug, Clone)] });

        #[cfg(feature = "serde")]
        base.extend(quote! { #[derive(::serde::Deserialize)] });

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
        let docs = util::doc_lines_split(self.struct_docs(), false);
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let derives = self.derive_tokens();
        let attrs = self.attr_tokens();
        let struct_ident = self.ident();
        let fields: Vec<_> = self.fields
            .iter()
            .map(ToTokens::into_token_stream)
            .collect();
        let phantom_fields = self.phantom_fields_tokens();
        let generics = &self.origin.generics;
        let where_clause = &generics.where_clause;
        quote! {
            #docs
            #cfg_attrs
            #doc_cfg_attrs
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
        self.fields
            .iter()
            .map(OverrideField::getter_tokens)
            .collect()
    }
    fn impl_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();
        let getters = self.getter_fns_tokens();
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();
        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            impl #impl_generics #ident #ty_generics #where_clause {
                #getters
            }
        }
    }
    fn default_impl_tokens(&self) -> TokenStream {
        assert!(!self.has_required_fields());

        let struct_ident = self.ident();
        let cfg_attrs = self.origin.cfg_tokens();
        let doc_cfg_attrs = self.origin.doc_cfg_tokens();

        // fields
        let mut fields = TokenStream::new();
        for field in &self.fields {
            // guaranteed to be Option since there are no required fields
            let ident = field.ident();
            let cfg_attrs = field.cfg_tokens();
            fields.extend(quote! {
                #cfg_attrs
                #ident: None,
            });

            // unset field
            if let Some(unset_ident) = field.origin.unset_ident() {
                fields.extend(quote! {
                    #cfg_attrs
                    #unset_ident: false,
                });
            }
        }
        // phantom fields
        let mut phantoms = TokenStream::new();
        for field in &self.phantom_fields {
            let ident = field.phantom_ident();
            let cfg_attrs = field.passthrough_attrs().cfg_tokens();
            phantoms.extend(quote! {
                #cfg_attrs
                #ident: ::std::marker::PhantomData,
            });
        }
        // generics
        let (
            impl_generics,
            ty_generics,
            where_clause,
        ) = self.origin.generics.split_for_impl();

        // #[allow(clippy::redundant_field_names)] added for:
        //   https://github.com/rust-lang/rust-clippy/issues/17525
        quote! {
            #cfg_attrs
            #doc_cfg_attrs
            #[automatically_derived]
            #[allow(clippy::redundant_field_names)]
            impl #impl_generics Default for #struct_ident #ty_generics #where_clause {
                fn default() -> Self {
                    Self {
                        #fields
                        #phantoms
                    }
                }
            }
        }
    }
}
impl ToTokens for OverrideStruct {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.extend(self.struct_tokens());
        tokens.extend(self.impl_tokens());
        if !self.has_required_fields() {
            // all fields must be optional for default impl
            tokens.extend(self.default_impl_tokens());
        }
    }
}

// !- Override struct field

pub(crate) type OverrideField = VariantField<OverrideVariant>;

impl OverrideField {
    pub(crate) fn is_required(&self) -> bool {
        self.attrs().override_required
    }
    #[allow(dead_code)] // only used w/ serde feature
    pub(crate) fn is_optional(&self) -> bool {
        !self.attrs().override_required
    }
    pub(crate) fn supports_copy(&self) -> bool {
        if self.attrs().override_from.is_some() {
            self.attrs().override_from_copy
        } else {
            self.attrs().copy
        }
    }
    pub(crate) fn flat_ty(&self) -> Type {
        match self.attrs().override_from {
            Some(ref ty) => Type::Path(ty.clone()),
            None => self.origin.flat_ty.clone(),
        }
    }
    pub(crate) fn flat_ty_pretty(&self) -> String {
        self.flat_ty().to_token_stream().to_string().replace(' ', "")
    }
    pub(crate) fn ty(&self) -> Type {
        let mut ty = self.flat_ty();
        if !self.attrs().override_required {
            ty = parse_quote!(Option<#ty>);
        }
        ty
    }
    fn getter_link(&self) -> Link {
        DerivedLink::new(self.ids().override_ident()).method(self.ident())
    }
    fn _unset_getter_link(&self) -> Option<Link> {
        self.origin.unset_ident().map(|unset_ident|
            DerivedLink::new(self.ids().override_ident()).method(unset_ident)
        )
    }
    fn getter_ret_ty(&self) -> Type {
        let ty = self.ty();

        // copy permitted, direct return
        if self.supports_copy() {
            return ty;
        }

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
        if self.supports_copy() {
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
}

impl OverrideField {
    // ! Field-based doc segments

    fn doc_notes(&self) -> String {
        let mut out = String::new();
        if self.is_required() {
            out.push_str("❗️");
        }
        if self.origin.is_conditional() {
            out.push('❔');
        }
        format!("<span align=\"center\">{out}</span>")
    }
}

impl OverrideField {
    // ! Field-based item docs

    pub(crate) fn field_docs(&self) -> AppendDoc {
        self.append_docs()
        + Doc::new().separator()
        + self.doc_default_val()
    }
    // TODO: add expandable unset aliases list
    pub(crate) fn getter_docs(&self) -> AppendDoc {
        self.append_docs()
        + Doc::new().separator()
        + self.doc_default_val()
    }
    fn unset_getter_docs(&self) -> Doc {
        let getter_link = self.getter_link();
        Doc::new()
        + format!("Returns `true` if the unset field for {getter_link} has been explicitly set to `true`")
    }
}

impl OverrideField {
    // ! Field-based token output

    fn attr_tokens(&self) -> Option<TokenStream> {
        self.origin.has_override_attrs().then(|| {
            let attrs = &self.origin.attrs.override_attrs;
            quote!(#(#[#attrs])*)
        })
    }
    pub(super) fn getter_tokens(&self) -> TokenStream {
        let ident = self.ident();
        let docs = self.getter_docs();
        let ty = self.getter_ret_ty();
        let ret = self.getter_ret_expr();
        // include the unset_getter here so that it appears immediately after
        // the normal getter
        let unset_getter = self.unset_getter_tokens();
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

            #unset_getter
        }

    }
    pub(super) fn unset_getter_tokens(&self) -> Option<TokenStream> {
        self.origin.unset_ident().map(|unset_ident| {
            let docs = self.unset_getter_docs();
            let cfg_attrs = self.cfg_tokens();
            let doc_cfg_attrs = self.doc_cfg_tokens();

            quote! {
                #docs
                #cfg_attrs
                #doc_cfg_attrs
                #[must_use]
                pub fn #unset_ident(&self) -> bool {
                    self.#unset_ident
                }
            }
        })
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
        let docs = self.field_docs();
        let attrs = self.attr_tokens();
        let cfg_attrs = self.cfg_tokens();
        let field = quote! {
            #docs
            #cfg_attrs
            #attrs
            #vis #ident: #ty,
        };
        tokens.extend(field);

        // unset field
        if let Some(unset_ident) = self.origin.unset_ident() {
            tokens.extend(quote! {
                #cfg_attrs
            });
            // !- FIXME: use cfg_attr and allow user to provide gate expression
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
