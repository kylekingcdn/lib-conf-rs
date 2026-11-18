mod field_attr;
mod struct_attr;

pub(crate) use self::{
    field_attr::FieldAttr,
    struct_attr::StructAttr,
};

use crate::{
    generate::util::render_expr,
    parse::{error::ParseError, util},
};

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::marker::PhantomData;
use syn::{
    Attribute, Expr, Ident, Lit, Meta, parse_quote,
    spanned::Spanned, Token, punctuated::Punctuated,
};

// !- Attribute keys

pub(crate) static HELPER_ATTR_CONFIG: &str = "config";
pub(crate) static CFG: &str = "cfg";
pub(crate) static DOC_ATTR: &str = "doc";

// !- Grouped attrs

/// Helper attribute (`#[config(..)]`) kinds, scoped by usage
///
/// - `Standard`: Typical attribute used to customize macro expansion behaviour
/// - `Proxy`: An attribute which has its contents wrapped in `#[config(scopename_attrs(..))]`.
///   - Used to specify attributes to include on specific derived structs/fields/etc.
/// - `Passthrough`: directly
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct GroupedAttrs<Variant: AttrVariant> {
    /// Standard configuration attr
    ///
    /// E.g. `copy`, `#[config(copy)]`
    pub standard: Vec<StandardAttr<Variant>>,

    /// Proxied attrs
    pub proxied: Vec<ProxiedAttr<Variant>>,

    /// Passthrough attrs (doc, cfg, doc(cfg))
    pub passthrough: PassthroughAttrs,
}
impl<Variant: AttrVariant> Default for GroupedAttrs<Variant> {
    fn default() -> Self {
        Self {
            standard: Vec::new(),
            proxied: Vec::new(),
            passthrough: PassthroughAttrs::default(),
        }
    }
}
impl<Variant: AttrVariant> TryFrom<Vec<Attribute>> for GroupedAttrs<Variant> {
    type Error = syn::Error;

    fn try_from(attrs: Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        let mut seen = HashMap::<Variant, Ident>::new(); // Map of Variant types seen on first pass
        let mut other_attrs = Vec::new(); // temp storage for non #[config] attrs before filter & set
        let mut first_helper_span = None;

        for attr in attrs {
            // handle `#[config(..)]` attrs
            if attr.path().is_ident(HELPER_ATTR_CONFIG) {
                if first_helper_span.is_none() {
                    first_helper_span = Some(attr.span());
                }
                let metas = attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)?;
                for meta in metas {
                    let ident = meta.path().require_ident()?.clone();
                    let variant = Variant::try_from(&ident).map_err(Into::into)?;

                    // handle proxied attr
                    if variant.is_proxied() {
                        variant.validate_shape(Shape::List, &ident)?;
                        out.proxied.push(ProxiedAttr {
                            variant,
                            // ident: ident.clone(),
                            inner: meta.require_list()?.tokens.clone(), // FIXME: use match for full extract
                        });
                        seen.insert(variant, ident);
                    }

                    // handle standard attr
                    else {
                        let shape = Shape::from(&meta);
                        variant.validate_shape(shape, &ident)?;
                        Self::validate_unique(variant, &seen)?;
                        out.standard.push(StandardAttr {
                            variant,
                            ident: ident.clone(),
                            meta,
                        });
                        seen.insert(variant, ident);
                    }
                }
            }

            // handle passthrough attr
            // not `#[config(..)]` -  add to to passthrough list if it passes filter
            else {
                other_attrs.push(attr);
            }
        }

        // do actual filter and set of passthrough attrs
        out.passthrough = other_attrs.into();

        out.validate_proxied_compat(&seen)?;

        // error on missing required
        if !Variant::required_set().is_empty()
            && let Some(span) = first_helper_span {
                Self::validate_required_present(&seen, span)?;
            }
        // error on implicit attr specified
        Self::validate_implicit(&seen)?;
        // error on incompatible attrs
        Self::validate_mutex(&seen)?;
        // validate deps
        Self::validate_deps(&seen)?;

        Ok(out)
    }
}
impl<Variant: AttrVariant> GroupedAttrs<Variant> {
    fn validate_unique(attr: Variant, seen: &HashMap<Variant, Ident>) -> Result<(), ParseError> {
        if let Some(ident) = seen.get(&attr) {
            Err(ParseError::DuplicateAttr { ident: ident.clone() })
        } else {
            Ok(())
        }
    }
    fn validate_implicit(seen: &HashMap<Variant, Ident>) -> Result<(), ParseError> {
        for (attr, ident) in seen {
            for implicit in attr.implied_attrs() {
                if let Some(implicit_ident) = seen.get(implicit) {
                    return Err(ParseError::ImplicitAttr {
                        ident: implicit_ident.clone(), // error on implied
                        other: ident.clone(),
                    });
                }
            }
        }
        Ok(())
    }
    fn validate_mutex(seen: &HashMap<Variant, Ident>) -> Result<(), ParseError> {
        for (attr, ident) in seen {
            for incompat in attr.incompatible_attrs() {
                if let Some(incompat_ident) = seen.get(incompat) {
                    return Err(ParseError::IncompatibleAttrs {
                        ident: incompat_ident.clone(), // error on implied
                        other: ident.clone(),
                    });
                }
            }
        }
        Ok(())
    }
    fn validate_deps(seen: &HashMap<Variant, Ident>) -> Result<(), ParseError> {
        for (attr, ident) in seen {
            let attr_deps = attr.dependencies();
            for dep in attr_deps {
                if !seen.contains_key(dep) {
                    return Err(ParseError::MissingAttrDep { present: ident.clone(), missing: (*dep).into() });
                }
            }
        }

        Ok(())
    }

    /// dedicated fn to properly resolve idents for error
    fn validate_proxied_compat(&self, seen: &HashMap<Variant, Ident>) -> Result<(), ParseError> {
        // manually handle mut ex validation on first pass for proxied attr
        // fixme, loop through all seen attrs, remove if let Some(attr_ident) = seen.get()
        for proxied in &self.proxied {
            if let Some(attr_ident) = seen.get(&proxied.variant) {
                for incompat in proxied.variant.incompatible_attrs() {
                    if let Some(incompat_ident) = seen.get(incompat) {
                        Err(ParseError::IncompatibleAttrs {
                            ident: attr_ident.clone(),
                            other: incompat_ident.clone(),
                        })?;
                    }
                }
            }
        }
        Ok(())
    }
    
    fn validate_required_present(seen: &HashMap<Variant, Ident>, span: Span) -> Result<(), ParseError> {
        for req_attr in Variant::required_set() {
            if !seen.contains_key(req_attr) {
                Err(ParseError::MissingRequiredAttr {
                    name: req_attr.as_str(),
                    span,
                })?;
            }
        }
        Ok(())
    }
}

// ! Standard

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct StandardAttr<Variant: AttrVariant> {
    /// The path as the attr family variant
    pub variant: Variant,

    /// The path ident of the meta list
    ///
    /// E.g, `some_attrs` in `#[config(some_attrs(allow(dead_code)))]`
    pub ident: Ident, // TODO: remove?

    /// Contents (includes variant)
    pub meta: Meta,
}

// ! Proxied

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ProxiedAttr<Variant: AttrVariant> {
    /// The path as the attr family variant
    /// ///
    /// E.g, `some_attrs` in `#[config(some_attrs(allow(dead_code)))]`
    pub variant: Variant,

    /// The inner attribute
    ///
    /// E.g, `allow(dead_code)` in `#[config(some_attrs(allow(dead_code)))]`
    ///
    /// **NOTE**: inner list must not be split/delimited
    pub inner: TokenStream,
}

// ! Passthrough

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct PassthroughAttrs {
    /// `#[doc = ".."]` attrs to (potentially) be incorporated
    ///
    /// Not currently used by origin struct, only by fields
    pub doc: Vec<Attribute>,

    /// `#[doc(cfg(..))]` attrs which will be passed through to all derived type
    ///
    /// For fields this also applies to getters and setters
    pub doc_cfg: Vec<Attribute>,

    /// `#[cfg]` attrs which will be passed through to all derived types
    ///
    /// For fields this also applies to getters, setters, and relevant statements where the field occurs
    pub cfg: Vec<Attribute>,
}
impl PassthroughAttrs {
    fn tokens(attrs: &Vec<Attribute>) -> TokenStream {
        quote! {
            #(#attrs)*
        }
    }
    pub(crate) fn doc(&self) -> &Vec<Attribute> {
        &self.doc
    }
    pub(crate) fn doc_cfg_tokens(&self) -> TokenStream {
        Self::tokens(&self.doc_cfg)
    }
    pub(crate) fn cfg_tokens(&self) -> TokenStream {
        Self::tokens(&self.cfg)
    }
}
impl From<Vec<Attribute>> for PassthroughAttrs {
    fn from(attrs: Vec<Attribute>) -> Self {
        let mut parsed = Self::default();
        for attr in attrs {
            assert!(
                !attr.path().is_ident(HELPER_ATTR_CONFIG),
                "Helper attrs should already be filtered out",
            );

            // #[doc]
            if let Some(doc_attr) = util::parse_doc_attrs(&attr) { // attr is doc
                // ensure doc attr is valid, then include it if not ignored
                match doc_attr {
                    util::ParsedDocAttr::Comment => parsed.doc.push(attr.clone()),
                    util::ParsedDocAttr::Cfg => parsed.doc_cfg.push(attr.clone()),
                    util::ParsedDocAttr::Other => {
                        #[cfg(feature = "syn-debug")]
                        { eprintln!("Ignoring doc attr: {}", attr.to_token_stream()); }
                    }
                }
            }
            // #[cfg]
            else if let Some(keep) = util::parse_cfg_attribute(&attr) {
                if keep {
                    parsed.cfg.push(attr.clone());
                } else {
                    #[cfg(feature = "syn-debug")]
                    { eprintln!("Ignoring cfg attr: {}", attr.to_token_stream());}
                }
            }
            else {
                // unknown attr that we were able to get a key of
                #[cfg(feature = "syn-debug")]
                {
                    if let Some(ident) = attr.path().get_ident() {
                        eprintln!("Ignoring unknown struct attribute: {ident}");
                    }
                }
            }
        }
        
        parsed
    }
}

// !- Attr family trait

pub(crate) trait AttrVariant:
    fmt::Debug + fmt::Display +
    Copy + Clone +
    PartialEq + Eq +
    Hash +
    TryFrom<Ident, Error: Into<syn::Error>> +
    for<'a> TryFrom<&'a Ident, Error: Into<syn::Error>> +
    Into<&'static str> +
    'static
{
    fn as_str(self) -> &'static str;
    
    /// Proxied attrs are attrs like `#[config(attr(allow(unused)))]` that get placed on derived itens
    fn is_proxied(self) -> bool {
        Self::proxied_set().contains(&self)
    }

    fn is_option_incompatible(self) -> bool {
        Self::opt_incompatible_set().contains(&self)
    }

    fn empty_set() -> &'static Vec<Self>;
    fn proxied_set() -> &'static Vec<Self>;
    fn opt_incompatible_set() -> &'static Vec<Self>;
    fn required_set() -> &'static Vec<Self>;
    fn dependency_map() -> &'static HashMap<Self, Vec<Self>>;
    fn incompatible_map() -> &'static HashMap<Self, Vec<Self>>;
    fn implied_map() -> &'static HashMap<Self, Vec<Self>>;
    fn shapes_map() -> &'static HashMap<Self, Vec<Shape>>;

    fn dependencies(&self) -> &'static Vec<Self> {
        Self::dependency_map().get(self).unwrap_or(Self::empty_set())
    }
    fn incompatible_attrs(&self) -> &'static Vec<Self> {
        Self::incompatible_map().get(self).unwrap_or(Self::empty_set())
    }
    fn implied_attrs(&self) -> &'static Vec<Self> {
        Self::implied_map().get(self).unwrap_or(Self::empty_set())
    }
    fn supports_shape(&self, shape: Shape) -> bool {
        Self::shapes_map().get(self).is_some_and(|x| x.contains(&shape))
    }
    fn validate_shape(self, shape: Shape, ident: &Ident) -> Result<(), ParseError> {
        if self.supports_shape(shape) {
            Ok(())
        } else {
            Err(ParseError::InvalidAttrShape { ident: ident.clone(), shape })
        }
    }
}

// !- Attr shape

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum Shape {
    Flag,
    KeyValue,
    List,
}
impl From<&Meta> for Shape {
    fn from(meta: &Meta) -> Shape {
        match meta {
            Meta::List(_) => Self::List,
            Meta::NameValue(_) => Self::KeyValue,
            Meta::Path(_) => Self::Flag,
        }
    }
}

// !- Attribute expression variants

pub(crate) trait ExprGroup {
    fn in_subset(expr: &Expr) -> bool;
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct ExprSubset<G: ExprGroup> {
    expr: Expr,
    _group: PhantomData<G>,
}
impl<G: ExprGroup> ExprSubset<G> {
    fn new(expr: Expr) -> Self {
        Self {
            expr,
            _group: PhantomData,
        }
    }
    pub(crate) fn to_pretty_string(&self) -> String {
        let tokens = self.to_token_stream();
        if let Some(pretty) = render_expr(&tokens) {
            pretty
        } else {
            tokens.to_string()
        }
    }
    fn try_from_default(expr: Expr) -> Result<Self, ParseError> {
        if G::in_subset(&expr) {
            Ok(Self::new(expr))
        } else {
            #[cfg(feature = "syn-debug")]
            {
                eprintln!("Unexpected Expr variant for AttrExpr: {:?}", &expr);
            }
            Err(ParseError::InvalidAttrExpr { span: expr.span() })
        }
    }
    
}
impl<G: ExprGroup> From<ExprSubset<G>> for Expr {
    fn from(subset: ExprSubset<G>) -> Self {
        subset.expr
    }
}
impl<G: ExprGroup> ToTokens for ExprSubset<G> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens)
    }
}

// ! Call expr

#[derive(Debug, Copy, Clone)]
pub(crate) struct CallExprGroup;

impl ExprGroup for CallExprGroup {
    fn in_subset(expr: &Expr) -> bool {
        match expr {
            Expr::Path(_) => true,
            //Expr::Call(_) |
            //Expr::MethodCall(_) => true,
            _ => false,
        }
    }
}

pub(crate) type CallExpr = ExprSubset<CallExprGroup>;

impl TryFrom<Expr> for CallExpr {
    type Error = ParseError;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        Self::try_from_default(expr)
    }
}

// ! Default expr

#[derive(Debug, Copy, Clone)]
pub(crate) struct DefaultExprGroup;

impl ExprGroup for DefaultExprGroup {
    fn in_subset(expr: &Expr) -> bool {
        match expr {
            Expr::Lit(_) |
            Expr::Path(_) |
            Expr::Call(_) |
            Expr::MethodCall(_) => true,
            _ => false,
        }
    }
}

pub(crate) type DefaultExpr = ExprSubset<DefaultExprGroup>;

impl DefaultExpr {
    pub(crate) fn none_expr() -> Self {
        Self::new(Expr::Path(parse_quote!(None)))
    }
}
impl TryFrom<Expr> for DefaultExpr {
    type Error = ParseError;
    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        Self::try_from_default(expr)
    }
}

// ! Str expr

#[derive(Debug, Copy, Clone)]
pub(crate) struct StrExprGroup;

impl ExprGroup for StrExprGroup {
    fn in_subset(expr: &Expr) -> bool {
        if let Expr::Lit(expr) = expr && matches!(&expr.lit, Lit::Str(_)) {
            true
        } else {
            false
        }
    }
}

pub(crate) type StrExpr = ExprSubset<StrExprGroup>;

impl TryFrom<Expr> for StrExpr {
    type Error = ParseError;

    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        if DefaultExprGroup::in_subset(&expr) {
            Ok(Self::new(expr))
        } else {
            #[cfg(feature = "syn-debug")]
            {
                eprintln!("Unexpected Expr variant for AttrExpr: {:?}", &expr);
            }
            Err(ParseError::ExpectedStrExpr { span: expr.span() })
        }
    }
}
impl fmt::Display for StrExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Expr::Lit(expr) = &self.expr
        && let Lit::Str(lit) = &expr.lit {
            let value = lit.value();
            write!(f, "{value}")?;
        }
        Ok(())
    }
}
