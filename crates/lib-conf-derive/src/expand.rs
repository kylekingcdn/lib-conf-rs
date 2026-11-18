use crate::parse::{
    attr::HELPER_ATTR_CONFIG,
    OriginStruct,
    util,
};

use proc_macro2::TokenStream;
use quote::quote;
use std::fs;
use std::rc::Rc;
use syn::{parse2, Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed};

const EXPAND_VAR: &str = "LIBCONF_EXPAND";
const EXPAND_DIR_VAR: &str = "LIBCONF_EXPAND_DIR";

pub(super) fn process(input: TokenStream, origin: &Rc<OriginStruct>, output: &TokenStream) {
    let expanded = expand(input, output);
    let pretty = prettify(expanded.to_string());

    if let Some(enabled) = option_env!("LIBCONF_EXPAND")
    && (enabled == "1" || enabled == "true") {
        let origin_ident = origin.ids().origin_ident();
        if let Some(out_dir) = option_env!("LIBCONF_EXPAND_DIR") {
            let out_dir = format!("{out_dir}/_expanded");

            if let Err(error) = fs::create_dir_all(&out_dir) {
                eprintln!("\nFailed to create '_expanded' dir at '{out_dir}'\n\nError: {error:#?}");
                return;
            }

            let basename = util::upper_pascal_to_snake_case(origin.ids().origin_ident().to_string().as_str());
            let out_path = format!("{out_dir}/{basename}.rs");

            if let Err(error) = fs::write(&out_path, &pretty) {
                eprintln!("\nFailed to write expanded config to file '{out_path}'\n\nError: {error:#?}");
                return;
            }

            eprintln!("    \x1B[1:92mExpanding\x1B[0m \x1B[1;3m{origin_ident}\x1B[0m");
            eprintln!("              \x1B[2m{out_path}\x1B[0m");
        }
        else {
            let origin_ident = origin.ids().origin_ident();
            eprintln!("\n  ------------------------------------------------");
            eprintln!("    \x1B[1mlib-conf expanded output\x1B[0m");
            eprintln!("    \x1B[96m{origin_ident}\x1B[0m");
            eprintln!("  ------------------------------------------------");
            for line in pretty.lines() {
                eprintln!("        {line}");
            }
            eprintln!("\n  \x1B[1;35mNote\x1B[0m:");
            eprintln!("    Output can be saved directly to files with env var:");
            eprintln!("    - \x1B[34m{EXPAND_DIR_VAR}\x1B[0m");
        }
    }
    else {
        if option_env!("LIBCONF_EXPAND_NO_TIPS").is_none() {
            eprintln!("lib-conf expansion disabled. It can be enabled with env var: \x1B[34m{EXPAND_VAR}\x1B[0m");
        }
    }
}

fn prettify(input: String) -> String {
    match syn::parse_file(&input) {
        Ok(parsed) => prettyplease::unparse(&parsed),
        Err(error) => {
            eprintln!("Failed to make output pretty. Error: {error:#?}");
            input
        }
    }

}
fn expand(input: TokenStream, output: &TokenStream) -> TokenStream {
    let input = parse2(input).unwrap(); // validated by initial parse pass
    let input_stripped = strip_input(input);

    quote! {
        use super::*;

        ///
        /// -------------------------------------
        /// LibConf input (config attr cleaned):
        /// -------------------------------------
        ///
        #[derive(Debug, Clone)]
        #input_stripped
        ///
        /// ----------------
        /// LibConf output:
        /// ----------------
        ///
        #output
    }
}

fn strip_input(mut input: DeriveInput) -> DeriveInput {
    input.attrs = prune_attrs(input.attrs);

    input.data = match input.data {
        Data::Struct(inner) => Data::Struct(strip_struct(inner)),
        _ => unreachable!()
    };

    input
}

fn strip_struct(mut input: DataStruct) -> DataStruct {
    input.fields = match input.fields {
        Fields::Named(fields) => Fields::Named(FieldsNamed {
            named: fields.named.into_iter().map(strip_field).collect(),
            ..fields
        }),
        _ => unreachable!()
    };

    input
}

fn strip_field(mut input: Field) -> Field {
    input.attrs = prune_attrs(input.attrs);
    input
}

fn prune_attrs(attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.into_iter().filter(is_not_config_attr).collect()
}

fn is_not_config_attr(attr: &Attribute) -> bool {
    !attr.path().is_ident(HELPER_ATTR_CONFIG)
}
