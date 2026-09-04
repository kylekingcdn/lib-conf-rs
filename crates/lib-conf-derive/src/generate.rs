pub(crate) mod builder_struct;
pub(crate) mod config_struct;
pub(crate) mod override_struct;
pub(crate) mod util;

pub(crate) use {
    builder_struct::BuilderStruct,
    config_struct::ConfigStruct,
    override_struct::OverrideStruct,
};

use crate::parse::{OriginField, FieldAttrs};

use std::marker::PhantomData;
use std::rc::Rc;
use syn::Ident;

// ! Variant-independent field

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub struct VariantField<T> {
    origin: Rc<OriginField>,
    _variant: PhantomData<T>,
}
impl<T> VariantField<T> {
    pub fn new(origin: Rc<OriginField>) -> Self {
        Self {
            origin,
            _variant: PhantomData,
        }
    }
    pub fn ident(&self) -> &Ident {
        &self.origin.ident
    }
    pub fn attrs(&self) -> &FieldAttrs {
        &self.origin.attrs
    }
    pub fn docs(&self) -> util::AppendDoc {
        let mut out = util::AppendDoc::new(self.origin.doc_attrs.clone());
        if let Some(default) = self.origin.default() {
            // separate existing docs, if any
            if !out.source().is_empty() {
                out.newl();
                out.line("---");
                out.newl();
            }
            out.line(format!("- Library default: **`{}`**", default.to_pretty_string()));
        }

        out
    }
}
