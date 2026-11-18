#![cfg_attr(docsrs, feature(doc_cfg))]
//#![deny(rustdoc::broken_intra_doc_links)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]

#![doc = include_str!("../README.md")]

#[cfg(doctest)]
mod doctests;

#[cfg(feature = "expand")]
mod expand;

mod generate;
mod lib_config;
mod parse;

use proc_macro::TokenStream;

#[doc = include_str!("../doc/attributes.md")]
///
#[doc = include_str!("../doc/nested.md")]
#[proc_macro_derive(LibConfig, attributes(config))]
pub fn derive_lib_config(input: TokenStream) -> TokenStream {
    // derive output
    let output = lib_config::derive(input.into());
    output.unwrap_or_else(syn::Error::into_compile_error).into()
}