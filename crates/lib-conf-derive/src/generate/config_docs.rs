use crate::{
    generate::doc::{AppendDoc, Doc, Document},
    parse::{OriginStruct, util},
};

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::DeriveInput;

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ConfigDocs {
    origin: OriginStruct,
}
impl ConfigDocs {
    pub(crate) fn docs(&self) -> AppendDoc {
        let mut docs = self.origin.append_docs();
        if !docs.source().is_empty() {
            if self.origin.attrs.lib_conf_attribution {
                docs = docs
                + Doc::new().separator()
                + "*Config powered by [lib-conf](https://crates.io/crates/lib-conf).*";
            }
        }
        docs
    }
    pub(crate) fn build(self) -> ConfigStructWithDocs {
        self.into()
    }
}
impl From<OriginStruct> for ConfigDocs {
    fn from(origin: OriginStruct) -> Self {
        Self {
            origin
        }
    }
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ConfigStructWithDocs {
    docs: AppendDoc,
    input: DeriveInput,
}

impl From<ConfigDocs> for ConfigStructWithDocs {
    fn from(config: ConfigDocs) -> Self {
        let docs = config.docs();
        let mut input = config.origin.into_input();
        input.attrs = util::strip_doc_comments(input.attrs);
        Self {
            docs,
            input,
        }
    }
}
impl ToTokens for ConfigStructWithDocs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.docs.to_tokens(tokens);
        self.input.to_tokens(tokens);
    }
}