use crate::parse::OriginStruct;
use crate::generate::{BuilderStruct, ConfigStruct, OverrideStruct};
use proc_macro::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::Error;

pub fn derive(input: TokenStream) -> Result<TokenStream, Error> {
    let output = {
        let origin = Rc::new(OriginStruct::try_from(input)?);

        let override_struct = Rc::new(OverrideStruct::new(origin.clone()));
        let builder_struct = Rc::new(BuilderStruct::new(origin.clone(), override_struct.clone()));
        let config_struct = ConfigStruct::new(origin, override_struct.clone(), builder_struct.clone());

        quote! {
            #config_struct
            #builder_struct
            #override_struct
        }
    };

    Ok(output.into())
}
