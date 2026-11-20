use crate::parse::attr;

use syn::{
    Attribute, Expr, GenericArgument, Lit, Meta, MetaList,
    PathArguments, punctuated::Punctuated, token, Type,
};

// !- String manipulation

pub(crate) fn snake_case_to_upper_pascal(snek: &str) -> String {
    let mut pasc = String::with_capacity(snek.len());
    let mut up_flag = true;

    for mut letter in snek.chars() {
        if letter == '_' {
            up_flag = true;
        } else {
            // if up flag was set by prev char (or start):
            // - convert to upper & toggle flag
            if up_flag {
                letter = letter.to_ascii_uppercase();
                up_flag = false;
            }
            pasc.push(letter);
        }
    }

    pasc
}

#[cfg(feature = "serde")]
pub(crate) fn upper_pascal_to_snake_case(pasc: &str) -> String {
    let mut snek = String::with_capacity((pasc.len()*5)/4);

    for mut letter in pasc.chars() {
        if letter.is_uppercase() {
            if !snek.is_empty() {
                snek.push('_');
            }
            letter = letter.to_ascii_lowercase();
        }
        snek.push(letter);
    }

    snek
}

// !- General attr parsing utils

pub(crate) type MetaListInner = Punctuated::<Meta, token::Comma>;

pub(crate) fn parse_inner_meta_list(meta_list: &MetaList) -> Result<MetaListInner, syn::Error> {
    meta_list.parse_args_with(MetaListInner::parse_terminated)
}

// !- Doc attr parsing

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum ParsedDocAttr {
    Comment,
    Cfg,
    Other,
}
impl ParsedDocAttr {
    pub(crate) fn is_comment(self) -> bool {
        matches!(self, Self::Comment)
    }
}
/// # Returns
///
/// - `None` if not `doc` attr
/// - Otherwise `Some(kind)`
pub(crate) fn parse_doc_attrs(attr: &Attribute) -> Option<ParsedDocAttr> {
    attr.path().is_ident(attr::DOC_ATTR).then(|| {
        #[expect(clippy::collapsible_if, reason = "improved readability")]
        // brief validation of correct shape before insert
        if let Ok(doc_meta) = attr.meta.require_name_value() {
            if let Expr::Lit(doc_lit) = &doc_meta.value &&
               let Lit::Str(_doc_lit_str) = &doc_lit.lit {
                return ParsedDocAttr::Comment;
            }
        }
        else if let Ok(meta_list) = attr.meta.require_list() {
            if let Ok(metas) = parse_inner_meta_list(meta_list) &&
                let Some(first) = metas.first() &&
                first.path().is_ident(attr::CFG) &&
                first.require_list().is_ok()
            {
                return ParsedDocAttr::Cfg;
            }
        }
        ParsedDocAttr::Other
    })
}

pub(crate) fn strip_doc_comments(attrs: Vec<Attribute>) -> Vec<Attribute> {
    attrs.into_iter().filter(|attr| {
        let attr_ty = parse_doc_attrs(attr);
        attr_ty.is_none_or(|doc_attr| !doc_attr.is_comment())
    }).collect()
}

/// Parse an attribute for use, if it's a `cfg` attr
///
/// # Returns
///
/// - `None` if not `cfg` attr
/// - Otherwise `Some(keep_attr)`
pub(crate) fn parse_cfg_attribute(attr: &Attribute) -> Option<bool> {
    attr.path().is_ident(attr::CFG).then(|| {
        // brief validation of correct shape before insert
        attr.meta.require_list().is_ok()
    })
}

// credit: taken from serde
// https://github.com/serde-rs/serde/blob/a874a1b1bb1cc16cf5ee3b1b7b527af5705742bb/serde_derive/src/internals/mod.rs#L23
pub(crate) fn ungroup(mut ty: &Type) -> &Type {
    while let Type::Group(group) = ty {
        ty = &group.elem;
    }
    ty
}

// credit: adapted from serde
// https://github.com/serde-rs/serde/blob/a874a1b1bb1cc16cf5ee3b1b7b527af5705742bb/serde_derive/src/internals/attr.rs#L1633
pub(crate) fn unwrap_option(ty: &Type) -> Option<&Type> {
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

pub(crate) fn is_string(ty: &Type) -> bool {
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
