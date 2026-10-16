use crate::parse::attr;

use syn::{
    Attribute, Expr, GenericArgument, Lit, Meta,
    PathArguments, punctuated::Punctuated, Token, Type,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ParsedDocAttr {
    Comment,
    Cfg,
    Other,
}

/// # Returns
///
/// - `None` if not `doc` attr
/// - Otherwise `Some(kind)`
pub fn parse_doc_attrs(attr: &Attribute) -> Option<ParsedDocAttr> {
    attr.path().is_ident(attr::DOC_ATTR).then(|| {
        //eprintln!("parsing doc: {}", attr.to_token_stream());
        #[allow(clippy::collapsible_if)]
        // brief validation of correct shape before insert
        if let Ok(doc_meta) = attr.meta.require_name_value() {
            if let Expr::Lit(doc_lit) = &doc_meta.value &&
               let Lit::Str(_doc_lit_str) = &doc_lit.lit {
                //eprintln!("got doc comment: {}", _doc_lit_str.value());
                return ParsedDocAttr::Comment;
            }
        }
        else if let Ok(doc_meta) = attr.meta.require_list() {
            if let Ok(metas) = doc_meta.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) &&
                let Some(first) = metas.first() &&
                first.path().is_ident(attr::CFG) &&
                first.require_list().is_ok()
            {
                //eprintln!("Passing through #[doc(cfg)] attr: {}", attr.to_token_stream());
                return ParsedDocAttr::Cfg;
            }
        }
        ParsedDocAttr::Other
    })
}

/// Parse an attribute for use, if it's a `cfg` attr
///
/// # Returns
///
/// - `None` if not `cfg` attr
/// - Otherwise `Some(keep_attr)`
pub fn parse_cfg_attribute(attr: &Attribute) -> Option<bool> {
    attr.path().is_ident(attr::CFG).then(|| {
        // brief validation of correct shape before insert
        if let Ok(_cfg_meta_list) = attr.meta.require_list() {
            // eprintln!("Passing through `#[cfg]` attr: {}", attr.to_token_stream());
            true
        } else {
            // eprintln!("got `#[cfg]` attr in unexpected shape: {}", attr.to_token_stream());
            false
        }
    })
}

// credit: taken from serde
// https://github.com/serde-rs/serde/blob/a874a1b1bb1cc16cf5ee3b1b7b527af5705742bb/serde_derive/src/internals/mod.rs#L23
pub fn ungroup(mut ty: &Type) -> &Type {
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

// credit: adapted from serde
// https://github.com/serde-rs/serde/blob/a874a1b1bb1cc16cf5ee3b1b7b527af5705742bb/serde_derive/src/internals/attr.rs#L1633
pub fn unwrap_option(ty: &Type) -> Option<&Type> {
    if let Type::Path(ty) = ungroup(ty)
        && let Some(seg) = ty.path.segments.last()
        && seg.ident == "Option"
        && let PathArguments::AngleBracketed(bracketed) = &seg.arguments
        && bracketed.args.len() == 1
        && let GenericArgument::Type(arg) = &bracketed.args[0]
    {
        Some(arg)
    } else {
        None
    }
}

pub fn is_string(ty: &Type) -> bool {
    if let Type::Path(ty) = ungroup(ty) {
        ty.path.is_ident("String")
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_is_option() {

        let opt1: Type = syn::parse_quote!(Option<bool>);
        let opt2: Type = syn::parse_quote!(Option::<bool>);
        let opt3: Type = syn::parse_quote!(::Option::<bool>);
        let opt4: Type = syn::parse_quote!(::Option::<()>);

        let res1 = unwrap_option(&opt1).unwrap();
        let res2 = unwrap_option(&opt2).unwrap();
        let res3 = unwrap_option(&opt3).unwrap();
        let res4 = unwrap_option(&opt4).unwrap();

        eprintln!("opt1: {}", quote!(#res1));
        eprintln!("opt2: {}", quote!(#res2));
        eprintln!("opt3: {}", quote!(#res3));
        eprintln!("opt4: {}", quote!(#res4));
    }
}
