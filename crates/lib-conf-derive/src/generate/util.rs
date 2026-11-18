use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::rc::Rc;
use syn::{ExprPath, Generics, Ident, Path, Type, TypePath, parse_quote};

// !- General item structure

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct MaybeFallibleFn(FnProperties);

impl MaybeFallibleFn {
    pub(crate) fn new(name: Ident, ty: impl Into<ReturnSignature>) -> Self {
        Self(FnProperties::new(name, ty))
    }
    pub(crate) fn into_fallible(self, error_ty: impl IntoTypePath) -> FnProperties {
        let name = self.0.name;
        let inner_ty = self.0.ret_sig.ty_explicit();
        let error_ty = error_ty.into_type_path();
        let result_path = CanonPath::Result;
        let ty: TypePath = parse_quote!(#result_path<#inner_ty, #error_ty>);

        FnProperties::new(format_ident!("try_{name}"), ty)
    }
    pub(crate) fn to_fallible(&self, error_ty: impl IntoTypePath) -> FnProperties {
        self.clone().into_fallible(error_ty)
    }
    pub(crate) fn into_infallible(self) -> FnProperties {
        self.0
    }
    pub(crate) fn into_fallible_when(
        self,
        fallible: bool,
        error_ty: impl IntoTypePath,
    ) -> FnProperties {
        if fallible {
            self.to_fallible(error_ty)
        } else {
            self.into_infallible()
        }
    }
    pub(crate) fn into_fallible_if_err(self, error_ty: Option<impl IntoTypePath>) -> FnProperties {
        if let Some(error) = error_ty {
            self.to_fallible(error)
        } else {
            self.into_infallible()
        }
    }
}

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct FnProperties {
    name: Ident,
    ret_sig: ReturnSignature,
}
impl FnProperties {
    pub(crate) fn new(name: Ident, ty: impl Into<ReturnSignature>) -> Self {
        Self {
            name,
            ret_sig: ty.into(),
        }
    }
    pub(crate) fn name(&self) -> &Ident {
        &self.name
    }
    // pub(crate) fn ret_sig(&self) -> &ReturnSignature {
    //     &self.ret_sig
    // }
    // pub(crate) fn ty(&self) -> Option<&TypePath> {
    //     self.ret_sig.ty()
    // }
    pub(crate) fn split_for_impl(&self) -> (&Ident, &ReturnSignature) {
        (&self.name, &self.ret_sig)
    }
}

/// Function return signature
///
/// Wrapper around optional return type. Tokenizes fn return arrow if type is set
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct ReturnSignature(Option<TypePath>);

impl ReturnSignature {
    // pub(crate) fn new(ty: impl Into<Self>) -> Self {
    //     ty.into()
    // }
    // pub(crate) fn empty() -> Self {
    //     Self::default()
    // }
    pub(crate) fn ty(&self) -> Option<&TypePath> {
        self.0.as_ref()
    }
    /// returns tokens for the type if some, otherwise returns tokens for `()`
    pub(crate) fn ty_explicit(&self) -> TokenStream {
        self.0.as_ref().map(ToTokens::to_token_stream).unwrap_or(quote!(()))
    }
    // pub(crate) fn has_return(&self) -> bool {
    //     self.0.is_some()
    // }
}
impl ToTokens for ReturnSignature {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(ty) = self.ty() {
            tokens.extend(quote!( -> #ty));
        }
    }
}
impl<T: IntoTypePath> From<T> for ReturnSignature {
    fn from(t: T) -> Self {
        Self(Some(t.into_type_path()))
    }
}
impl From<()> for ReturnSignature {
    fn from((): ()) -> Self {
        Self(None)
    }
}

// !- Path/fn call generation

/// Builds a `static_assertions` macro call
pub(crate) fn static_assert(pkg: &Rc<TypePath>, macro_name: impl ToTokens, args: impl ToTokens) -> TokenStream {
    quote!(#pkg::#macro_name(#args);)
}

pub(crate) fn assert_impl_all(pkg: &Rc<TypePath>, ty: &impl ToTokens, traits: &[impl ToTokens]) -> TokenStream {
    static_assert(pkg, quote!(assert_impl_all!), quote!(#ty: #(#traits), *))
}

pub(crate) fn build_type(ident: &Ident, generics: &Generics) -> Type {
    let ty_gen = generics.split_for_impl().1;
    parse_quote!(#ident #ty_gen)
}

// !- Path/type enums

/// Canonical item path tokenizer
#[expect(dead_code, reason = "unused for now")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum CanonPath {
    Result,
    Ok,
    Err,
    Error,
    Option,
    Some,
    None,
    Default,
    String,
    Vec,
    PhantomData,
    Copy,
    Clone,
    Debug,
    Display,
    From,
    TryFrom,
    Into,
    TryInto,
    Add,
    AddAssign,
    BuildConfig,
    TryBuildConfig,
    MergeOverride,
    TryMergeOverride,
    ConfigError,
}
impl CanonPath {
    pub(crate) fn path(self) -> Path {
        match self {
            Self::Result => parse_quote!(::std::result::Result),
            Self::Ok => parse_quote!(::std::result::Result::Ok),
            Self::Err => parse_quote!(::std::result::Result::Err),
            Self::Error => parse_quote!(::std::error::Error),
            Self::Option => parse_quote!(::std::option::Option),
            Self::Some => parse_quote!(::std::option::Some),
            Self::None => parse_quote!(::std::option::None),
            Self::Default => parse_quote!(::std::default::Default),
            Self::Vec => parse_quote!(::std::vec::Vec),
            Self::String => parse_quote!(::std::string::String),
            Self::PhantomData => parse_quote!(::std::marker::PhantomData),
            Self::Copy => parse_quote!(::std::marker::Copy),
            Self::Clone => parse_quote!(::std::clone::Clone),
            Self::Debug => parse_quote!(::std::fmt::Debug),
            Self::Display => parse_quote!(::std::fmt::Display),
            Self::From => parse_quote!(::std::convert::From),
            Self::TryFrom => parse_quote!(::std::convert::TryFrom),
            Self::Into => parse_quote!(::std::convert::Into),
            Self::TryInto => parse_quote!(::std::convert::TryInto),
            Self::Add => parse_quote!(::std::ops::Add),
            Self::AddAssign => parse_quote!(::std::ops::AddAssign),
            Self::BuildConfig => parse_quote!(build::BuildConfig),
            Self::TryBuildConfig => parse_quote!(build::TryBuildConfig),
            Self::MergeOverride => parse_quote!(merge::MergeOverride),
            Self::TryMergeOverride => parse_quote!(merge::TryMergeOverride),
            Self::ConfigError => parse_quote!(error::ConfigError),
        }
    }
    pub(crate) fn dep_path(self, pkg: Rc<TypePath>) -> CanonDepPath {
        CanonDepPath::new(pkg, self)
    }

}
impl ToTokens for CanonPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path().to_tokens(tokens);
    }
}
impl From<CanonPath> for Path {
    fn from(path: CanonPath) -> Self {
        path.path()
    }
}
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct CanonDepPath {
    pub pkg: Rc<TypePath>,
    pub item: CanonPath,
}
impl CanonDepPath {
    pub(crate) fn new(pkg: Rc<TypePath>, item: CanonPath) -> Self {
        Self {
            pkg,
            item,
        }
    }
    pub(crate) fn path(&self) -> Path {
        let pkg = &self.pkg;
        let item = &self.item;
        parse_quote!(#pkg::#item)
    }
}
impl ToTokens for CanonDepPath {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path().to_tokens(tokens);
    }
}
// !- Path traits

/// Absolute path helper for generic traits
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) enum CanonTrait {
    From(TokenStream),
    TryFrom { from: TokenStream, error: Option<TokenStream> },
    BuildConfig { pkg: Rc<TypePath>, config: Option<TokenStream> },
    TryBuildConfig { pkg: Rc<TypePath>, config: Option<TokenStream>, error: Option<TokenStream> },
    MergeOverride { pkg: Rc<TypePath>, override_: Option<TokenStream> },
    TryMergeOverride { pkg: Rc<TypePath>, override_: Option<TokenStream>, error: Option<TokenStream> },
}
impl CanonTrait {
    pub(crate) fn path(&self) -> Path {
        match self {
            Self::From(_) => CanonPath::From.into(),
            Self::TryFrom { .. } => CanonPath::TryFrom.into(),
            Self::BuildConfig { pkg, .. } => CanonPath::BuildConfig.dep_path(pkg.clone()).path(),
            Self::TryBuildConfig { pkg, .. } => CanonPath::TryBuildConfig.dep_path(pkg.clone()).path(),
            Self::MergeOverride { pkg, .. } => CanonPath::MergeOverride.dep_path(pkg.clone()).path(),
            Self::TryMergeOverride { pkg, .. } => CanonPath::TryMergeOverride.dep_path(pkg.clone()).path(),
        }
    }
    fn params(&self) -> Vec<TokenStream> {
        match self.clone() {
            Self::From(from) => vec![from],
            Self::TryFrom { from, error } => vec![
                from,
                error.map(|e| quote!(Error = #e)).unwrap_or_default(),
            ],
            Self::BuildConfig { config, .. } => vec![
                config.map(|c| quote!(Config = #c)).unwrap_or_default(),
            ],
            Self::TryBuildConfig { config, error, .. } => vec![
                config.map(|c| quote!(Config = #c)).unwrap_or_default(),
                error.map(|e| quote!(Error = #e)).unwrap_or_default(),
            ],
            Self::MergeOverride { override_, .. } => vec![
                override_.map(|o| quote!(Override = #o)).unwrap_or_default(),
            ],
            Self::TryMergeOverride { override_, error, .. } => vec![
                override_.map(|o| quote!(Override = #o)).unwrap_or_default(),
                error.map(|e| quote!(Error = #e)).unwrap_or_default(),
            ],
        }
        .into_iter()
        .filter(|p| !p.is_empty())
        .collect()
    }
}
impl ToTokens for CanonTrait {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let path = self.path();
        let params = self.params();

        tokens.extend(quote! {
            #path<#(#params),*>
        });
    }
}

/// Extended syn conversion, improving parameter diversity via `impl IntoTypePath`
pub(crate) trait IntoTypePath: Clone {
    fn into_type_path(self) -> TypePath;
}
impl IntoTypePath for TypePath {
    fn into_type_path(self) -> TypePath {
        self
    }
}
impl IntoTypePath for Rc<TypePath> {
    fn into_type_path(self) -> TypePath {
        (*self).clone()
    }
}
impl IntoTypePath for Path {
    fn into_type_path(self) -> TypePath {
        TypePath {
            path: self,
            qself: None,
            attrs: Vec::new(),
        }
    }
}
impl IntoTypePath for ExprPath {
    fn into_type_path(self) -> TypePath {
        TypePath {
            path: self.path,
            qself: self.qself,
            attrs: self.attrs,
        }
    }
}
impl IntoTypePath for Ident {
    fn into_type_path(self) -> TypePath {
        Path::from(self).into_type_path()
    }
}
impl IntoTypePath for Rc<Ident> {
    fn into_type_path(self) -> TypePath {
        Path::from((*self).clone()).into_type_path()
    }
}
impl IntoTypePath for &Rc<Ident> {
    fn into_type_path(self) -> TypePath {
        Path::from((**self).clone()).into_type_path()
    }
}

// !- Common trait impls

pub(crate) fn bare_default_impl_tokens(
    ident: &Ident,
    generics: &Generics,
    cfg_attrs: &TokenStream,
    doc_cfg_attrs: &TokenStream,
) -> TokenStream {
    let default_path = CanonPath::Default;
    let (
        impl_generics,
        ty_generics,
        where_clause,
    ) = generics.split_for_impl();
    quote! {
        #cfg_attrs
        #doc_cfg_attrs
        #[automatically_derived]
        impl #impl_generics #default_path for #ident #ty_generics #where_clause {
            fn default() -> Self {
                Self::new()
            }
        }
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
pub(crate) fn render_expr(tokens: &TokenStream) -> Option<String> {
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
