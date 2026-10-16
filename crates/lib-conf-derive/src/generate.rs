pub(crate) mod builder_struct;
pub(crate) mod config_struct;
pub(crate) mod override_struct;
pub(crate) mod util;

pub(crate) use {
    builder_struct::BuilderStruct,
    config_struct::ConfigStruct,
    override_struct::OverrideStruct,
};

use crate::{
    generate::util::AppendDoc,
    parse::{FieldAttrs, IdentPool, OriginField},
};
use proc_macro2::TokenStream;
use std::marker::PhantomData;
use std::rc::Rc;
use syn::Ident;

// ! Variant-independent field

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct VariantField<T> {
    origin: Rc<OriginField>,
    _variant: PhantomData<T>,
}
impl<T> VariantField<T> {
    pub(crate) fn new(origin: Rc<OriginField>) -> Self {
        Self {
            origin,
            _variant: PhantomData,
        }
    }
    pub(crate) fn ids(&self) -> &IdentPool {
        self.origin.ids()
    }
    pub(crate) fn ident(&self) -> &Ident {
        &self.origin.ident
    }
    pub(crate) fn attrs(&self) -> &FieldAttrs {
        &self.origin.attrs
    }
    pub(crate) fn doc_default_val(&self) -> Option<String> {
        self.origin.default().map(|def|
            format!("- Library default: **`{}`**", def.to_pretty_string())
        )
    }
    pub(crate) fn append_docs(&self) -> AppendDoc {
        self.origin.passthrough_attrs().doc().clone().into()
    }
    pub(crate) fn cfg_tokens(&self) -> TokenStream {
        self.origin.passthrough_attrs().cfg_tokens()
    }
    pub(crate) fn has_cfg_attrs(&self) -> bool {
        !self.origin.passthrough_attrs().cfg.is_empty()
    }
    pub(crate) fn doc_cfg_tokens(&self) -> TokenStream {
        self.origin.passthrough_attrs().doc_cfg_tokens()
    }
}
