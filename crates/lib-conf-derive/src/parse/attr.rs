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
use syn::{Attribute, Expr, Ident, Lit, Meta, parse_quote, spanned::Spanned};

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
impl<Variant: AttrVariant> GroupedAttrs<Variant> {
    pub(crate) fn try_from_attrs<Ctx>(attrs: Vec<Attribute>, validation_ctx: &mut Ctx) -> Result<Self, syn::Error>
    where
        UnvalidatedGroupedAttrs<Variant>: ValidateAttrs<Variant, Ctx>
    {
        let unvalidated = UnvalidatedGroupedAttrs::<Variant>::try_from(attrs)?;
        let grouped = unvalidated.validate(validation_ctx)?;
        Ok(grouped)
    }
}

// !- Validate attrs trait

pub(crate) trait ValidateAttrs<Variant: AttrVariant, Ctx>: Sized {
    /// Validation specific to the attr variant
    fn validate_scoped(&self, context: &mut Ctx) -> Result<(), ParseError>;

    /// Validation independent of attr variant
    fn validate_base(&self) -> Result<(), ParseError>;

    fn into_validated(self) -> GroupedAttrs<Variant>;

    fn validate(self, context: &mut Ctx) -> Result<GroupedAttrs<Variant>, ParseError> {
        self.validate_scoped(context)?;
        self.validate_base()?;

        Ok(self.into_validated())
    }
}

// !- Unvalidated grouped attrs

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct UnvalidatedGroupedAttrs<Variant: AttrVariant> {
    pub grouped: GroupedAttrs<Variant>,
    pub seen: HashMap<Variant, Ident>,
    pub first_helper_span: Option<Span>,
}
impl<Variant: AttrVariant> Default for UnvalidatedGroupedAttrs<Variant> {
    fn default() -> Self {
        Self {
            grouped: GroupedAttrs::default(),
            seen: HashMap::new(),
            first_helper_span: None,
        }
    }
}
impl<Variant: AttrVariant> TryFrom<Vec<Attribute>> for UnvalidatedGroupedAttrs<Variant> {
    type Error = syn::Error;
    fn try_from(attrs: Vec<Attribute>) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        let mut other_attrs = Vec::new(); // temp storage for non #[config] attrs before filter & set

        for attr in attrs {
            // handle `#[config(..)]` attrs
            if attr.path().is_ident(HELPER_ATTR_CONFIG) {

                if out.first_helper_span.is_none() {
                    out.first_helper_span = Some(attr.span());
                }
                for meta in attr.parse_args_with(util::MetaListInner::parse_terminated)? {
                    out.parse_helper_attr(None, meta)?;
                }
            }

            // handle
            // handle non-`#[config(..)]` (passthrough) attrs
            else {
                other_attrs.push(attr);
            }
        }

        // handles filtering passthrough attrs
        out.grouped.passthrough = other_attrs.into();

        Ok(out)
    }
}
impl<Variant: AttrVariant> UnvalidatedGroupedAttrs<Variant> {
    fn parse_helper_attr(
        &mut self,
        parent: Option<Variant>,
        meta: Meta,
    ) -> Result<(), syn::Error> {
        let ident = meta.path().require_ident()?.clone();
        let variant = Variant::try_from((parent, &ident)).map_err(Into::into)?;

        // handle proxied attr
        if variant.is_proxied() {
            assert!(parent.is_none());
            variant.validate_shape(Shape::List, &ident)?;
            self.grouped.proxied.push(ProxiedAttr {
                variant,
                inner: meta.require_list()?.tokens.clone(), // FIXME: use match for full extract
            });
            self.seen.insert(variant, ident);
        }

        // handle standard attr
        else {
            let shape = Shape::from(&meta);
            // handle shape & unique validation during attr scan (fail-fast)
            variant.validate_shape(shape, &ident)?;
            self.validate_unique(variant)?;

            // nested attrs, recurse
            if let Meta::List(inner) = meta {
                let nested_metas = util::parse_inner_meta_list(&inner)?;
                for nested in nested_metas {
                    self.parse_helper_attr(Some(variant), nested)?;
                }
                self.grouped.standard.push(StandardAttr {
                    variant,
                    ident: ident.clone(),
                    meta: Meta::Path(inner.path),
                });
            } else {
                self.grouped.standard.push(StandardAttr {
                    variant,
                    ident: ident.clone(),
                    meta,
                });
            }
            self.seen.insert(variant, ident);
        }
        Ok(())
    }
    pub(crate) fn validate_common(&self) -> Result<(), ParseError> {
        self.validate_proxied_compat()?;

        // error on missing required attr (and a #[config()] group exists)
        self.validate_required_present()?;

        // error on implicit attr specified
        self.validate_implicit()?;

        // error on multiple exclusive deps prior to normal mutex & deps checks
        //self.validate_exclusive_deps()?;

        // error on incompatible attrs
        self.validate_mutex()?;

        // validate deps
        self.validate_deps()?;

        Ok(())
    }
    fn validate_unique(&self, attr: Variant) -> Result<(), ParseError> {
        if let Some(ident) = self.seen.get(&attr) {
            Err(ParseError::DuplicateAttr { ident: ident.clone() })
        } else {
            Ok(())
        }
    }
    fn validate_implicit(&self) -> Result<(), ParseError> {
        for (attr, ident) in &self.seen {
            for implicit in attr.implied_attrs() {
                if let Some(implicit_ident) = self.seen.get(implicit) {
                    return Err(ParseError::ImplicitAttr {
                        ident: implicit_ident.clone(), // error on implied
                        other: ident.clone(),
                    });
                }
            }
        }
        Ok(())
    }
    fn validate_mutex(&self) -> Result<(), ParseError> {
        for (attr, ident) in &self.seen {
            for incompat in attr.incompatible_attrs() {
                if let Some(incompat_ident) = self.seen.get(incompat) {
                    return Err(ParseError::IncompatibleAttrs {
                        ident: incompat_ident.clone(), // error on implied
                        other: ident.clone(),
                    });
                }
            }
        }
        Ok(())
    }
    fn validate_deps(&self) -> Result<(), ParseError> {
        for (attr, ident) in &self.seen {
            let attr_deps = attr.dependencies();
            for dep in attr_deps {
                if !self.seen.contains_key(dep) {
                    return Err(ParseError::MissingAttrDep { present: ident.clone(), missing: (*dep).into() });
                }
            }
        }

        Ok(())
    }
    // fn validate_exclusive_deps(&self) -> Result<(), ParseError> {
    //     for (attr, ident) in &self.seen {
    //         let attr_deps = attr.exclusive_dependencies();
    //         if !attr_deps.is_empty() {
    //             let mut first_dep = None;
    //             for dep in attr_deps {
    //                 if let Some(second_ident) = self.#seen.get(dep) {
    //                     if let Some(first) = first_dep  {
    //                         return Err(ParseError::MultipleExclusiveDeps { first, second: second_ident.clone() });
    //                     } else {
    //                         first_dep = Some(second_ident.clone());
    //                     }
    //                 }
    //             }
    //             if first_dep.is_none() {
    //                 return Err(ParseError::MissingExclusiveDep {
    //                         present: ident.clone(),
    //                         supported: attr_deps.into_iter().map(|v| v.as_str()).collect(),
    //                     });
    //             }
    //         }
    //     }

    //     Ok(())
    // }
    /// dedicated fn to properly resolve idents for error
    fn validate_proxied_compat(&self) -> Result<(), ParseError> {
        // manually handle mut ex validation on first pass for proxied attr
        // fixme, loop through all seen attrs, remove if let Some(attr_ident) = seen.get()
        for proxied in &self.grouped.proxied {
            if let Some(attr_ident) = self.seen.get(&proxied.variant) {
                for incompat in proxied.variant.incompatible_attrs() {
                    if let Some(incompat_ident) = self.seen.get(incompat) {
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

    fn validate_required_present(&self) -> Result<(), ParseError> {
        if !Variant::required_set().is_empty() && let Some(span) = &self.first_helper_span {
            for req_attr in Variant::required_set() {
                if !self.seen.contains_key(req_attr) {
                    Err(ParseError::MissingRequiredAttr {
                        name: req_attr.as_str(),
                        span: *span,
                    })?;
                }
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
    ///
    /// E.g, `some_attrs` in `#[config(some_attrs(allow(dead_code)))]`
    pub variant: Variant,

    /// The inner attribute
    ///
    /// E.g, `allow(dead_code)` in `#[config(some_attrs(allow(dead_code)))]`
    ///
    /// **NOTE**: inner list must not be split/delimited, it's of an unknown (potentially non-compliant) format
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
    TryFrom<(Option<Self>, Ident), Error: Into<syn::Error>> +
    for<'a> TryFrom<(Option<Self>, &'a Ident), Error: Into<syn::Error>> +
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
    //fn exclusive_dependency_map() -> &'static HashMap<Self, Vec<Self>>;
    fn incompatible_map() -> &'static HashMap<Self, Vec<Self>>;
    fn implied_map() -> &'static HashMap<Self, Vec<Self>>;
    fn shapes_map() -> &'static HashMap<Self, Vec<Shape>>;

    fn dependencies(&self) -> &'static Vec<Self> {
        Self::dependency_map().get(self).unwrap_or(Self::empty_set())
    }
    // fn exclusive_dependencies(&self) -> &'static Vec<Self> {
    //     Self::exclusive_dependency_map().get(self).unwrap_or(Self::empty_set())
    // }
    fn incompatible_attrs(&self) -> &'static Vec<Self> {
        Self::incompatible_map().get(self).unwrap_or(Self::empty_set())
    }
    fn implied_attrs(&self) -> &'static Vec<Self> {
        Self::implied_map().get(self).unwrap_or(Self::empty_set())
    }
    fn shape_names(&self) -> Vec<String> {
        let shapes = Self::shapes_map().get(self).cloned().unwrap_or_default();
        shapes.into_iter().map(|s| format!("{s:#?}")).collect()
    }
    fn supports_shape(&self, shape: Shape) -> bool {
        Self::shapes_map().get(self).is_some_and(|x| x.contains(&shape))
    }
    fn validate_shape(self, shape: Shape, ident: &Ident) -> Result<(), ParseError> {
        if self.supports_shape(shape) {
            Ok(())
        } else {
            Err(ParseError::InvalidAttrShape { ident: ident.clone(), shape, valid: self.shape_names() })
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
                eprintln!("Unexpected Expr variant for AttrExpr: {expr:?}");
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
        self.expr.to_tokens(tokens);
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

// !- FIXME: path fn call possibly broken
impl ExprGroup for DefaultExprGroup {
    fn in_subset(expr: &Expr) -> bool {
        matches!(expr,
            Expr::Lit(_) |
            Expr::Path(_) |
            Expr::Call(_) |
            Expr::MethodCall(_)
        )
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

impl StrExpr {
    pub(crate) fn try_to_string(expr: Expr) -> Result<String, ParseError> {
        Ok(Self::try_from(expr)?.to_string())
    }
}
impl TryFrom<Expr> for StrExpr {
    type Error = ParseError;

    fn try_from(expr: Expr) -> Result<Self, Self::Error> {
        if DefaultExprGroup::in_subset(&expr) {
            Ok(Self::new(expr))
        } else {
            #[cfg(feature = "syn-debug")]
            {
                eprintln!("Unexpected Expr variant for AttrExpr: {expr:?}");
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
