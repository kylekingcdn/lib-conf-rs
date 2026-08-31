use crate::parse::attr::DOC_ATTR;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Display;
use std::ops::AddAssign;
use syn::{Attribute, Generics, Ident, Path, Type, parse_quote};

// !- Token manipulation

pub fn build_type(ident: &Ident, generics: &Generics) -> Type {
    let ty_gen = generics.split_for_impl().1;
    parse_quote!(#ident #ty_gen)
}

// !- Derive tokens helper

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct Derives(Vec<Path>);

impl ToTokens for Derives {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if !self.0.is_empty() {
            let derives = &self.0;
            tokens.extend(quote! { #[derive(#( #derives ),*)] });
        }
    }
}
impl From<Vec<TokenStream>> for Derives {
    fn from(tokens: Vec<TokenStream>) -> Self {
        Self(tokens.into_iter().map(|t| parse_quote!(#t)).collect())
    }
}
impl From<Vec<Path>> for Derives {
    fn from(paths: Vec<Path>) -> Self {
        Self(paths)
    }
}

// !- Doc attr addition helper

/// Handles trim + prefix of leading space
pub fn doc_line(text: impl Display) -> TokenStream {
    let text = text.to_string();
    let line = if text.is_empty() {
        String::new()
    } else {
        format!(" {}", text.clone().trim())
    };

    quote! {
        #[doc = #line]
    }
}

/// Handles trim + prefix of leading space
pub fn doc_lines(lines: &[String]) -> TokenStream {
    let lines: Vec<_> = lines.iter().map(|s|
        if s.is_empty() {
            s.clone()
        } else {
            format!(" {}", s.trim())
        }
    ).collect();

    quote! {
        #(#[doc = #lines])*
    }
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct AppendDoc {
    source: Vec<Attribute>,
    append: Vec<String>,
}
impl AppendDoc {
    pub fn new(source: Vec<Attribute>) -> Self {
        for attr in &source {
            assert!(attr.path().is_ident(DOC_ATTR));
        }
        Self {
            source,
            append: Vec::new(),
        }
    }

    pub fn source(&self) -> &Vec<Attribute> {
        &self.source
    }
    pub fn append(&self) -> &Vec<String> {
        &self.append
    }
    pub fn is_empty(&self) -> bool {
        self.source.is_empty() && self.append.is_empty()
    }

    pub fn line(&mut self, line: impl Display) {
        // TODO: assert has no newlines?
        self.append.push(line.to_string());
    }
    /// iterates over provided lines and adds each as a new element
    pub fn lines(&mut self, text: impl Display) {
        let text = text.to_string();
        for line in text.lines() {
            self.line(line);
        }
    }
    pub fn newl(&mut self) {
        self.append.push(String::new());
    }
    pub fn to_append_tokens(&self) -> TokenStream {
        doc_lines(&self.append)
    }
}
impl From<Vec<Attribute>> for AppendDoc {
    fn from(value: Vec<Attribute>) -> Self {
        Self::new(value)
    }
}
impl AddAssign<String> for AppendDoc {
    fn add_assign(&mut self, rhs: String) {
        self.lines(rhs);
    }
}
impl AddAssign<Vec<String>> for AppendDoc {
    fn add_assign(&mut self, rhs: Vec<String>) {
        for line in rhs {
            self.line(line);
        }
    }
}
impl ToTokens for AppendDoc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let source = &self.source;
        let append = self.to_append_tokens();
        tokens.extend(quote! {
            #(#source)*
            #append
        });
    }
}

// !- Pretty expression rendering

// NOTE: these must match the output provided by prettyplease
// TODO: unit tests to prevent breakage from prettyplease changes
static RENDER_PREFIX: &str = "static x: x = ";
static RENDER_SUFFIX: &str = ";";

// this feels so very wrong..
// dtolnay, please forgive me for the flagrant crimes I'm committing against
// your APIs
//
// hopefully some parse()/peek() work on the attrs can replace this without
// falling back to str attr values.
//
/// Pretty renders non-litstr expressions pulled in from atts, in an incredibly
///hacky fashion
pub fn render_expr(tokens: &TokenStream) -> Option<String> {
    let content = format!("{RENDER_PREFIX}{tokens}{RENDER_SUFFIX}");
    if let Ok(parsed) = syn::parse_file(&content) {
        let pretty = prettyplease::unparse(&parsed);

        if pretty.len() > RENDER_PREFIX.len() + RENDER_SUFFIX.len() {
            let inner_i = RENDER_PREFIX.len();
            let inner_f = pretty.len() - RENDER_SUFFIX.len();

            return Some(pretty[inner_i..inner_f - 1].to_string());
        }
    }

    None
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn extract_generics_works() {
//         let empty: Generics = parse_quote!();
//         let extracted = extract_generic_params_for_type(&empty);
//         assert_eq!(extracted.to_string(), quote!().to_string());

//         let basic: Generics = parse_quote!(<A,B,C>);
//         let extracted = extract_generic_params_for_type(&basic);
//         assert_eq!(extracted.to_string(), quote!(<A,B,C>).to_string());

//         let bounded: Generics = parse_quote!(<A: Copy, B: Copy + Clone>);
//         let extracted = extract_generic_params_for_type(&bounded);
//         assert_eq!(extracted.to_string(), quote!(<A, B>).to_string());
//     }
// }
