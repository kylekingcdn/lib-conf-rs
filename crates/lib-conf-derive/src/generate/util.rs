#![allow(dead_code)]

use crate::parse::attr::DOC_ATTR;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Display;
use std::ops::{Add, AddAssign};
use syn::{Attribute, Generics, Ident, Type, parse_quote};

// !- Token manipulation

pub(crate) fn build_type(ident: &Ident, generics: &Generics) -> Type {
    let ty_gen = generics.split_for_impl().1;
    parse_quote!(#ident #ty_gen)
}

// !- Common trait impls

pub(crate) fn bare_default_impl_tokens(
    ident: &Ident,
    generics: &Generics,
    cfg_attrs: &TokenStream,
    doc_cfg_attrs: &TokenStream,
) -> TokenStream {
    let (
        impl_generics,
        ty_generics,
        where_clause,
    ) = generics.split_for_impl();
    quote! {
        #cfg_attrs
        #doc_cfg_attrs
        #[automatically_derived]
        impl #impl_generics Default for #ident #ty_generics #where_clause {
            fn default() -> Self {
                Self::new()
            }
        }
    }
}

// !- Doc attr addition helper

/// Handles trim + prefix of leading space
pub(crate) fn doc_line(text: impl Display, trim: bool) -> TokenStream {
    let line = text.to_string();
    doc_lines(vec![&line], trim)
}

/// Handles trim + prefix of leading space
pub(crate) fn doc_lines(lines: Vec<&str>, trim: bool) -> TokenStream {
    let lines: Vec<_> = lines.into_iter().map(|s| {
        let text = s.to_string();
        let trimmed = if trim {
            text.trim()
        } else {
            &text
        };
        if trimmed.is_empty() {
            String::new()
        } else {
            format!(" {trimmed}")
        }
    }).collect();

    quote! {
        #(#[doc = #lines])*
    }
}
/// Handles line split + trim + prefix of leading space
pub(crate) fn doc_lines_split(lines: impl Display, trim: bool) -> TokenStream {
    doc_lines(lines.to_string().lines().collect(), trim)
}

// !- Document trait

/// Docs helper trait
pub(crate) trait Document: Sized {
    /// Intended for use by trait implementors only
    #[must_use]
    fn insert(self, line: impl Display) -> Self;

    #[must_use]
    #[doc(hidden)]
    fn insert_text(mut self, text: impl Display) -> Self {
        for line in text.to_string().lines() {
            // let trimmed = if self.preserve_whitespace() {
            //     eprintln!("preserving: {line:#?}");
            //     line
            // } else {
            //     eprintln!("trimming: {line:#?}");
            //     //line.trim()
            //     line
            // };
            self = self.insert(line);
        }
        self
    }
    // #[must_use]
    // #[doc(hidden)]
    // fn insert_transparent(mut self, text: impl Display, trim: bool) -> Self {
    //     self.insert_fmt(text, false)
    // }
    /// Returns true if any docs are currently present
    fn is_empty(&self) -> bool;

    // fn preserve_whitespace(&self) -> bool;

    /// iterates over provided lines and adds each as a new element
    #[must_use]
    fn add(mut self, doc: impl Into<Doc>) -> Self {
        let doc = doc.into();
        for line in doc.lines() {
            self = self.insert(line);
        }
        self
    }

    #[must_use]
    fn newl(self) -> Self {
        self.insert(String::new())
    }

    // !- TODO: change Vec<String> to Vec<DocItem>
    // DocItem variants:
    // - Text(String)
    // - H(Level, String)
    // - Separators
    // - Tables
    /// Adds a separator w/ newlines if there are currently docs
    ///
    /// Responsibility on caller to ensure text will be added after
    #[must_use]
    fn separator(self) -> Self {
        self.newl()
            .insert("---")
            .newl()
    }
    #[must_use]
    fn section(self, docs: impl Into<Doc>) -> Self {
        let docs = docs.into();
        if docs.is_empty() {
            self
        } else {
            self.separator()
                .add(docs)
        }
    }
    #[must_use]
    fn sections(mut self, sections: Vec<Doc>) -> Self {
        for section in sections {
            self = self.section(section);
        }
        self
    }

    #[must_use]
    #[doc(hidden)]
    fn header(self, level: u8, text: impl Display) -> Self {
        let text = text.to_string();
        let trimmed = text.trim();
        let prefix = "#".repeat(level.into());
        self.insert(format!("{prefix} {trimmed}"))
    }
    #[must_use]
    fn h1(self, text: impl Display) -> Self {
        self.header(1, text)
    }
    #[must_use]
    fn h2(self, text: impl Display) -> Self {
        self.header(2, text)
    }
    #[must_use]
    fn h3(self, text: impl Display) -> Self {
        self.header(3, text)
    }
    #[must_use]
    fn h4(self, text: impl Display) -> Self {
        self.header(4, text)
    }
    #[must_use]
    fn h5(self, text: impl Display) -> Self {
        self.header(5, text)
    }
    #[must_use]
    fn begin_code(self, flags: &str) -> Self {
        self.insert(format!("```{flags}"))
    }
    #[must_use]
    fn end_code(self) -> Self {
        self.begin_code("")
    }
    // TODO: handle escapes
    #[must_use]
    fn begin_collapsed(self, summary: impl Display) -> Self {
        self.insert("<details>")
            .insert(format!("<summary><b>{summary}</b></summary"))
            .newl()
    }
    // TODO: handle escapes
    #[must_use]
    fn end_collapsed(self) -> Self {
        self.newl()
            .insert("</details>")
    }
}

// !- Doc struct

/// Docs generated locally
#[derive(Debug, Clone, Default)]
pub struct Doc {
    lines: Vec<String>,
}
impl Doc {
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
    fn merge(&mut self, doc: Doc) {
        self.lines.extend(doc.lines);
    }
    pub fn as_text(&self) -> String {
        self.lines.join("\n")
    }
}
impl Document for Doc {
    #[doc(hidden)]
    fn insert(mut self, line: impl Display) -> Self {
        self.lines.push(line.to_string());
        self
    }
    fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
impl Display for Doc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.lines {
            writeln!(f, "{line}")?;
        }
        Ok(())
    }
}
impl From<Doc> for Vec<String> {
    fn from(doc: Doc) -> Vec<String> {
        doc.lines
    }
}
impl From<&str> for Doc {
    fn from(text: &str) -> Self {
        Self::default().insert_text(text)
    }
}
impl From<String> for Doc {
    fn from(text: String) -> Self {
        Self::default().insert_text(text)
    }
}
impl From<CodeBlock> for Doc {
    fn from(block: CodeBlock) -> Self {
        Self {
            lines: block.build_lines()
        }
    }
}
impl<R: TableRow> From<Table<R>> for Doc {
    fn from(table: Table<R>) -> Self {
        Self::default().insert_text(table)
    }
}
impl From<Option<&str>> for Doc {
    fn from(text: Option<&str>) -> Self {
        if let Some(t) = text {
            Self::default().insert_text(t)
        } else {
            Self::default()
        }
    }
}
impl From<Option<String>> for Doc {
    fn from(text: Option<String>) -> Self {
        if let Some(t) = text {
            Self::default().insert_text(t)
        } else {
            Self::default()
        }
    }
}
impl From<Option<Doc>> for Doc {
    fn from(doc: Option<Doc>) -> Self {
        doc.unwrap_or_default()
    }
}
impl From<Vec<String>> for Doc {
    fn from(text: Vec<String>) -> Self {
        let mut out = Self::default();
        for line in text {
            out = out.insert_text(line);
        }
        out
    }
}
impl From<Vec<Doc>> for Doc {
    fn from(docs: Vec<Doc>) -> Self {
        let mut out = Self::default();
        for doc in docs {
            out.merge(doc);
        }
        out
    }
}
impl<T: Into<Doc>> Add<T> for Doc {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}
impl<T: Into<Doc>> AddAssign<T> for Doc {
    fn add_assign(&mut self, rhs: T) {
        self.merge(rhs.into());
    }
}
impl Add<()> for Doc {
    type Output = Self;

    fn add(mut self, _rhs: ()) -> Self::Output {
        self.lines.push(String::new());
        self
    }
}
impl AddAssign<()> for Doc {
    fn add_assign(&mut self, _rhs: ()) {
        self.lines.push(String::new());
    }
}
impl ToTokens for Doc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lines = doc_lines(self.lines.iter().map(String::as_str).collect(), false);
        tokens.extend(lines);
    }
}

// !- AppendDoc struct

/// Merges docs from source with locally generated docs
#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone, Default)]
pub(crate) struct AppendDoc {
    source: Vec<Attribute>,
    append: Doc,
}
#[allow(dead_code)]
impl AppendDoc {
    pub(crate) fn new(source: Vec<Attribute>) -> Self {
        for attr in &source {
            assert!(attr.path().is_ident(DOC_ATTR));
        }
        Self {
            source,
            append: Doc::default(),
        }
    }
    pub(crate) fn source(&self) -> &Vec<Attribute> {
        &self.source
    }
    pub(crate) fn appended(&self) -> &Doc {
        &self.append
    }
}
impl Document for AppendDoc {
    #[doc(hidden)]
    fn insert(mut self, line: impl Display) -> Self {
        self.append = self.append.insert(line);
        self

    }
    fn is_empty(&self) -> bool {
        self.source.is_empty() && self.append.is_empty()
    }
}
impl ToTokens for AppendDoc {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let source = &self.source;
        let append = &self.append;
        tokens.extend(quote! {
            #(#source)*
            #append
        });
    }
}
impl From<Vec<Attribute>> for AppendDoc {
    fn from(value: Vec<Attribute>) -> Self {
        Self::new(value)
    }
}
impl From<Doc> for AppendDoc {
    fn from(append: Doc) -> Self {
        Self {
            source: Vec::new(),
            append,
        }
    }
}
impl<T: Into<Doc>> Add<T> for AppendDoc {
    type Output = Self;

    fn add(mut self, rhs: T) -> Self::Output {
        self.append += rhs;
        self
    }
}
impl<T: Into<Doc>> AddAssign<T> for AppendDoc {
    fn add_assign(&mut self, rhs: T) {
        self.append += rhs;
    }
}
impl Add<()> for AppendDoc {
    type Output = Self;

    fn add(mut self, rhs: ()) -> Self::Output {
        self.append += rhs;
        self
    }
}
impl AddAssign<()> for AppendDoc {
    fn add_assign(&mut self, rhs: ()) {
        self.append += rhs;
    }
}

// !- Code block struct

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CodeBlock {
    opts: String,
    lines: Vec<String>
}
impl CodeBlock {
    pub fn new(opts: &str) -> Self {
        Self {
            opts: opts.to_string(),
            ..Default::default()
        }
    }
    pub fn lines(&self) -> &Vec<String> {
        &self.lines
    }
    pub fn build_lines(self) -> Vec<String> {
        let opts = self.opts;
        let mut out = Vec::new();
        out.push(format!("```{opts}"));
        out.extend(self.lines);
        out.push("```".to_string());
        out
    }
}
impl AddAssign<&str> for CodeBlock {
    fn add_assign(&mut self, rhs: &str) {
        for line in rhs.lines() {
            self.lines.push(line.to_string());
        }
    }
}
impl AddAssign<String> for CodeBlock {
    fn add_assign(&mut self, rhs: String) {
        for line in rhs.lines() {
            self.lines.push(line.to_string());
        }
    }
}
impl AddAssign<Option<&str>> for CodeBlock {
    fn add_assign(&mut self, rhs: Option<&str>) {
        if let Some(rhs) = rhs {
            *self += rhs;
        }
    }
}
impl AddAssign<Option<String>> for CodeBlock {
    fn add_assign(&mut self, rhs: Option<String>) {
        if let Some(rhs) = rhs {
            *self += rhs;
        }
    }
}
impl<T> Add<T> for CodeBlock
where
    Self: AddAssign<T>
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}
// !- Doc link utils

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub(crate) enum LinkScope {
    Implicit,
    Module,
    Struct,
    Enum,
    Trait,
    Function,
    Method,
    Field,
    Variant,
}
impl LinkScope {
    pub(crate) fn scope_name(self) -> Option<&'static str> {
        match self {
            Self::Implicit => None,
            Self::Module => Some("module"),
            Self::Struct => Some("struct"),
            Self::Enum => Some("enum"),
            Self::Trait => Some("trait"),
            Self::Function => Some("function"),
            Self::Method => Some("method"),
            Self::Field => Some("field"),
            Self::Variant => Some("variant"),
        }
    }
    pub(crate) fn ref_prefix(self) -> Option<String> {
        self.scope_name().map(|s| format!("{s}@"))
    }
    pub(crate) fn name_suffix(self) -> Option<&'static str> {
        match self {
            Self::Method |
            Self::Function => Some("()"),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Link {
    name_path: String,
    ref_path: String,
    scope: LinkScope,
    fragment: Option<String>,
    mono: bool,
}
impl Link {
    pub(crate) fn new(name_path: impl Display, ref_path: impl Display) -> Self {
        Self{
            name_path: name_path.to_string(),
            ref_path: ref_path.to_string(),
            scope: LinkScope::Implicit,
            fragment: None,
            mono: false,
        }
    }

    #[must_use]
    pub(crate) fn name_path(mut self, name: impl Display) -> Self {
        self.name_path = name.to_string();
        self
    }
    #[must_use]
    pub(crate) fn ref_path(mut self, ref_path: impl Display) -> Self {
        self.ref_path = ref_path.to_string();
        self
    }
    #[must_use]
    pub(crate) fn scope(mut self, scope: LinkScope) -> Self {
        self.scope = scope;
        self
    }
    #[must_use]
    pub(crate) fn fragment(mut self, s: impl Display) -> Self {
        let s = s.to_string();
        self.fragment = (!s.is_empty()).then_some(s);
        self
    }
    #[must_use]
    pub(crate) fn mono(mut self) -> Self {
        self.mono = true;
        self
    }

    #[must_use]
    pub(crate) fn build_name(&self) -> String {
        let name = self.name_path.replace('[', "\\[");
        let name = name.replace(']', "\\]");
        // hacky method to fix names that were pre-escaped
        let name = name.replace("\\\\[", "\\[");
        let name = name.replace("\\\\]", "\\]");

        let suffix = self.scope.name_suffix().unwrap_or("");

        let mut out = format!("{name}{suffix}");
        if self.mono {
            out = format!("`{out}`");
        }
        out
    }
    #[must_use]
    pub(crate) fn build_ref(&self) -> String {
        let prefix = self.scope.ref_prefix().unwrap_or_default();
        let fragment = self.build_fragment();

        format!("{prefix}{}{fragment}", self.ref_path)
    }
    #[must_use]
    pub(crate) fn build_fragment(&self) -> String {
        if let Some(fragment) = &self.fragment && !fragment.is_empty() {
            format!("#{fragment}")
        } else {
            String::new()
        }
    }
    #[must_use]
    pub(crate) fn build(&self) -> String {
        let link_name = self.build_name();
        let link_ref = self.build_ref();

        format!("[{link_name}]({link_ref})")
    }
}
impl Display for Link {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

/// Wrapper around `Link` for simpler linking to derived structs/fns/etc
///
/// Since derived items will all be in scope, only the type's ident is required
pub(crate) struct DerivedLink {
    struct_ident: Ident,
}
impl DerivedLink {
    pub(crate) fn new(struct_ident: &Ident) -> Self {
        Self {
            struct_ident: struct_ident.clone(),
        }
    }
    #[must_use]
    fn build(self, member: Option<&impl Display>, include_struct: bool) -> Link {
        let name = if let Some(member) = &member {
            if include_struct {
                format!("{}::{member}", self.struct_ident)
            } else {
                format!("{member}")
            }
        } else {
            format!("{}", self.struct_ident)
        };

        let ref_path = if let Some(member) = &member {
            format!("{}::{member}", self.struct_ident)
        } else {
            format!("{}", self.struct_ident)
        };

        Link::new(name, ref_path).mono()
    }

    #[must_use]
    pub(crate) fn struct_only(self) -> Link {
        self.build(Option::<&String>::None, false).scope(LinkScope::Struct)
    }
    #[must_use]
    pub(crate) fn enum_only(self) -> Link {
        self.build(Option::<&String>::None, false).scope(LinkScope::Enum)
    }
    #[must_use]
    pub(crate) fn trait_only(self) -> Link {
        self.build(Option::<&String>::None, false).scope(LinkScope::Trait)
    }

    /// Includes only the function in the link name
    #[must_use]
    pub(crate) fn function(self, ident: impl Display) -> Link {
        self.build(Some(&ident), false).scope(LinkScope::Function)
    }
    /// Includes the containing type in the link name as well.
    ///
    /// Helps make links more intuitive when linking to members of another type
    #[must_use]
    pub(crate) fn function_explicit(self, ident: impl Display) -> Link {
        self.build(Some(&ident), true).scope(LinkScope::Function)
    }

    /// Includes only the method in the link name
    #[must_use]
    pub(crate) fn method(self, ident: impl Display) -> Link {
        self.build(Some(&ident), false).scope(LinkScope::Method)
    }
    /// Includes the containing type in the link name as well.
    ///
    /// Helps make links more intuitive when linking to members of another type
    #[must_use]
    pub(crate) fn method_explicit(self, ident: impl Display) -> Link {
        self.build(Some(&ident), true).scope(LinkScope::Method)
    }

    /// Includes only the field in the link name
    #[must_use]
    pub(crate) fn field(self, ident: impl Display) -> Link {
        self.build(Some(&ident), false).scope(LinkScope::Field)
    }
    /// Includes the containing type in the link name as well.
    ///
    /// Helps make links more intuitive when linking to members of another type
    #[must_use]
    pub(crate) fn field_explicit(self, ident: impl Display) -> Link {
        self.build(Some(&ident), true).scope(LinkScope::Field)
    }

    /// Includes only the variant in the link name
    #[must_use]
    pub(crate) fn variant(self, ident: impl Display) -> Link {
        self.build(Some(&ident), false).scope(LinkScope::Variant)
    }
    /// Includes the containing type in the link name as well.
    ///
    /// Helps make links more intuitive when linking to members of another type
    #[must_use]
    pub(crate) fn variant_explicit(self, ident: impl Display) -> Link {
        self.build(Some(&ident), true).scope(LinkScope::Variant)
    }
}

// !- Doc table

// TODO: store cols as strings, add add_row/new impls to blanket blocks
// with impl Table<(String, String)>

#[derive(Debug, Clone)]
pub struct Table<R: TableRow> {
    header: Row<R>,
    rows: Vec<Row<R>>,
}
impl<R: TableRow> Table<R> {
    pub fn new(header: R) -> Self {
        Self {
            header: Row(header),
            rows: Vec::new(),
        }
    }
    pub fn add_row(&mut self, row: R) {
        self.rows.push(Row(row));
    }
    pub fn cols()  -> usize {
        R::cols()
    }
    fn build_row(cols: &[String]) -> String {
        format!("| {} |", cols.join(" | "))
    }
    fn build_separator() -> String {
        let mut sep_cols = Vec::new();
        for _ in 0..Self::cols() {
            sep_cols.push("-".to_string());
        }
        Self::build_row(&sep_cols)
    }
}
impl<R: TableRow> Display for Table<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.header)?;
        writeln!(f, "{}", Self::build_separator())?;
        for row in &self.rows {
            writeln!(f, "{row}")?;
        }
        Ok(())
    }
}
impl Table<String> {
    pub fn new_1(col1: impl Display) -> Self {
        Self::new(col1.to_string())
    }
}
impl Table<(String, String)> {
    pub fn new_2(col1: impl Display, col2: impl Display) -> Self {
        Self::new((col1.to_string(), col2.to_string()))
    }
}
impl Table<(String, String, String)> {
    pub fn new_3(col1: impl Display, col2: impl Display, col3: impl Display) -> Self {
        Self::new((col1.to_string(), col2.to_string(), col3.to_string()))
    }
}
impl Table<(String, String, String, String)> {
    pub fn new_4(col1: impl Display, col2: impl Display, col3: impl Display, col4: impl Display) -> Self {
        Self::new((col1.to_string(), col2.to_string(), col3.to_string(), col4.to_string()))
    }
}
impl<C1: Display> AddAssign<C1> for Table<String> {
    fn add_assign(&mut self, c1: C1) {
        self.add_row(c1.to_string());
    }
}
impl<R, T> Add<T> for Table<R>
where
    R: TableRow,
    Self: AddAssign<T>
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        self += rhs;
        self
    }
}
impl<
    C1: Display,
    C2: Display,
> AddAssign<(C1,C2)> for Table<(String, String)> {
    fn add_assign(&mut self, (c1,c2): (C1,C2)) {
        self.add_row((c1.to_string(), c2.to_string()));
    }
}
impl<
    C1: Display,
    C2: Display,
    C3: Display,
> AddAssign<(C1,C2,C3)> for Table<(String, String, String)> {
    fn add_assign(&mut self, (c1,c2,c3): (C1,C2,C3)) {
        self.add_row((c1.to_string(), c2.to_string(), c3.to_string()));
    }
}
impl<
    C1: Display,
    C2: Display,
    C3: Display,
    C4: Display,
> AddAssign<(C1,C2,C3,C4)> for Table<(String, String, String, String)> {
    fn add_assign(&mut self, (c1,c2,c3,c4): (C1,C2,C3,C4)) {
        self.add_row((c1.to_string(), c2.to_string(), c3.to_string(), c4.to_string()));
    }
}

// ! Row internal newtype

#[derive(Debug, Clone)]
struct Row<R: TableRow>(R);

impl<R: TableRow> Display for Row<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cols = self.0.as_vec();
        write!(f, "{}", Table::<R>::build_row(&cols))
    }
}

// ! Table row trait

pub trait TableRow {
    fn cols() -> usize;
    fn get(&self, index: usize) -> String;

    fn as_vec(&self) -> Vec<String> {
        let mut cols = Vec::new();
        for col in 0..Self::cols()  {
            cols.push(self.get(col));
        }
        cols
    }
}

// ! TableRow impls

impl TableRow for String {
    fn cols() -> usize { 1 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.clone(),
            _ => unreachable!("type safety guards against invalid index"),
        }
    }
}
impl TableRow for &str {
    fn cols() -> usize { 1 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.to_string(),
            _ => unreachable!("type safety guards against invalid index"),
        }
    }
}
impl TableRow for Doc {
    fn cols() -> usize { 1 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.as_text(),
            _ => unreachable!("type safety guards against invalid index"),
        }
    }
}
impl<
    C1: Display,
    C2: Display,
> TableRow for (C1, C2) {
    fn cols() -> usize { 2 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.0.to_string(),
            1 => self.1.to_string(),
            _ => unreachable!("type safety guards against invalid index"),
        }
    }
}
impl<
    C1: Display,
    C2: Display,
    C3: Display,
> TableRow for (C1, C2, C3) {
    fn cols() -> usize { 3 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.0.to_string(),
            1 => self.1.to_string(),
            2 => self.2.to_string(),
            _ => unreachable!("type safety guards against invalid index"),
        }
    }
}
impl<
    C1: Display,
    C2: Display,
    C3: Display,
    C4: Display,
> TableRow for (C1, C2, C3, C4) {
    fn cols() -> usize { 4 }
    fn get(&self, index: usize) -> String {
        match index {
            0 => self.0.to_string(),
            1 => self.1.to_string(),
            2 => self.2.to_string(),
            3 => self.3.to_string(),
            _ => unreachable!("type safety guards against invalid index"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn link_simple() {
        // base
        let link = Link::new("MyConfig", "conf::MyConfig");
        assert_eq!(link.to_string().as_str(), "[MyConfig](conf::MyConfig)");

        // mononspaced
        let link = Link::new("MyConfig", "conf::MyConfig").mono();
        assert_eq!(link.to_string().as_str(), "[`MyConfig`](conf::MyConfig)");

        // setter overrides
        let link = Link::new("MyConfig", "conf::MyConfig")
            .name_path("Config")
            .ref_path("Config");
        assert_eq!(link.to_string().as_str(), "[Config](Config)");
    }
    #[test]
    fn link_scope_works() {
        let link = Link::new("MyConfig::new", "conf::MyConfig::new")
            .scope(LinkScope::Method)
            .mono();
        assert_eq!(link.to_string().as_str(), "[`MyConfig::new()`](method@conf::MyConfig::new)");

        let link = Link::new("db_url", "MyConfig::db_url")
            .scope(LinkScope::Field)
            .mono();
        assert_eq!(link.to_string().as_str(), "[`db_url`](field@MyConfig::db_url)");
    }
    #[test]
    fn link_fragment_works() {
        let link = Link::new("MyConfig::new", "conf::MyConfig::new")
            .scope(LinkScope::Method)
            .mono()
            .fragment("examples");
        assert_eq!(link.to_string().as_str(), "[`MyConfig::new()`](method@conf::MyConfig::new#examples)");
    }
    #[test]
    fn url_and_escapes_work() {
        let name = "Linking to items by name - Valid Links [The rustdoc book]";
        let url = "https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html";
        let link = Link::new(name, url).fragment("valid-links");
        assert_eq!(link.to_string().as_str(), "[Linking to items by name - Valid Links \\[The rustdoc book\\]](https://doc.rust-lang.org/rustdoc/write-documentation/linking-to-items-by-name.html#valid-links)");
    }

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
}
