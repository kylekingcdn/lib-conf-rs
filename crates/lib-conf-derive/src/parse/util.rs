use crate::parse::attr;

use quote::ToTokens;
use syn::{Attribute, Error, Expr, GenericArgument, Lit, PathArguments, spanned::Spanned, Type};

// FIXME: ParseError types
pub fn try_parse_doc_attrs(attr: &Attribute) -> Result<Option<Attribute>, Error> {
    if attr.path().is_ident(attr::DOC_ATTR) {
        //eprintln!("Attaching doc attr: {}", attr.meta.to_token_stream());
        // brief  validation of correct shape before insert
        if let Ok(doc_meta) = attr.meta.require_name_value() {
            if let Expr::Lit(doc_lit) = &doc_meta.value {
                if let Lit::Str(doc_lit_str) = &doc_lit.lit {
                    //let doc_str = doc_lit_str.value();
                    //eprintln!("Got doc string: {doc_str}");
                    Ok(Some(attr.clone()))
                } else {
                    Err(Error::new(attr.span(), format!("Unexpected doc literal type: {}", doc_lit.into_token_stream())))
                }
            } else {
                Err(Error::new(attr.span(), format!("Unexpected doc attr expression: {}", doc_meta.into_token_stream())))
            }
        } else {
            Err(Error::new(attr.span(), format!("Unexpected doc attr shape: {}", attr.meta.to_token_stream())))
        }
    } else {
        Ok(None) // ignore non-doc attr
    }
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
