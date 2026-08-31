#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]

mod generate;
mod lib_config;
mod parse;

use proc_macro::TokenStream;

#[proc_macro_derive(LibConfig, attributes(config))]
pub fn derive_lib_config(input: TokenStream) -> TokenStream {
    match lib_config::derive(input) {
        Ok(tokens) => tokens,
        Err(error) => error.into_compile_error().into(),
    }
}
