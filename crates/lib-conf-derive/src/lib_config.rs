use crate::{
    generate::{BuilderStruct, ConfigStruct, OverrideStruct, plan::Consensus},
    parse::OriginStruct,
};
use proc_macro2::TokenStream;
use quote::quote;
use std::rc::Rc;
use syn::Error;

pub(crate) fn derive(input: TokenStream) -> Result<TokenStream, Error> {
    // save input for later expansion
    #[cfg(feature = "expand")]
    let expand_input = input.clone();

    let origin = Rc::new(OriginStruct::try_from(input)?);
    // save origin for later expansion
    #[cfg(feature = "expand")]
    let expand_origin = origin.clone();

    let override_struct = Rc::new(OverrideStruct::new(origin.clone()));
    
    let builder_struct = Rc::new(BuilderStruct::new(origin.clone(), override_struct.clone()));
    let config_struct = ConfigStruct::new(origin.clone(), override_struct.clone(), builder_struct.clone());

    let consensus = Consensus {
        config: config_struct.build_consensus(),
        builder: builder_struct.build_consensus(),
        override_: override_struct.build_consensus(),
    };
    if origin.attrs.vote_debug {
        let report = consensus.as_rendered_report();
        eprintln!("————————————————————————————————");
        eprintln!("\n{} Consensus", &config_struct.ident());
        eprintln!("{report}");
    }

    let output = quote! {
        #config_struct
        #builder_struct
        #override_struct
    };

    // expand input (`#[config()]` stripped) and output
    #[cfg(feature = "expand")]
    crate::expand::process(expand_input, &expand_origin, &output);

    Ok(output)
}

