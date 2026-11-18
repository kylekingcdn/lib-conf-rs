
#[doc = r""] #[doc = r" -------------------------------------"]
#[doc = r" LibConf input (config attr cleaned):"]
#[doc = r" -------------------------------------"] #[doc = r""]
#[doc = " Settings for `example3` of `lib-conf`"] pub struct Example3Config
{
    #[doc = " Name of the client app using the library"] #[doc = ""]
    #[doc = " This field is mandatory and cannot be changed at runtime"]
    pub(crate) client_name : String, #[doc = " Enables usage telemetry"]
    #[doc = ""] #[doc = " This field is mandatory"] pub(crate)
    enable_telemetry : bool, #[doc = " Enables verbose logging"] pub(crate)
    verbose : bool
} #[doc = r""] #[doc = r" ----------------"] #[doc = r" LibConf output:"]
#[doc = r" ----------------"] #[doc = r""] impl Example3Config
{
    #[doc(hidden)] #[allow(dead_code)] fn __assertions()
    {
        :: lib_conf :: __private :: static_assertions :: assert_impl_all!
        (String : :: std :: clone :: Clone, :: std :: fmt :: Debug); ::
        lib_conf :: __private :: static_assertions :: assert_impl_all!
        (bool : :: std :: clone :: Clone, :: std :: fmt :: Debug);
    } #[must_use] #[allow(clippy :: redundant_field_names)] pub(crate) fn
    new(client_name : String, enable_telemetry : bool) -> Self
    {
        Self
        {
            client_name : client_name, enable_telemetry : enable_telemetry,
            verbose : false
        }
    } #[doc = r" Creates a new config builder"] #[must_use] pub fn
    builder(client_name : String, enable_telemetry : bool) ->
    Example3ConfigBuilder
    { < Example3ConfigBuilder > :: new(client_name, enable_telemetry) }
    #[doc = " Name of the client app using the library"] #[doc = ""]
    #[doc = " This field is mandatory and cannot be changed at runtime"]
    #[doc = ""] #[doc = " ---"] #[doc = ""] #[must_use] pub fn
    client_name(& self) -> & str { self.client_name.as_str() }
    #[doc = " Enables usage telemetry"] #[doc = ""]
    #[doc = " This field is mandatory"] #[doc = ""] #[doc = " ---"]
    #[doc = ""] #[must_use] pub fn enable_telemetry(& self) -> & bool
    { & self.enable_telemetry } #[doc = " Enables verbose logging"]
    #[doc = ""] #[doc = " ---"] #[doc = ""]
    #[doc = " - Library default: **`false`**"] #[must_use] pub fn
    verbose(& self) -> bool { self.verbose }
    #[doc =
    r" Applies all values from override (which are `Some(_)`) to self"] pub fn
    merge_with_override(& mut self, override_conf : Example3OverrideConfig)
    {
        {
            let val = override_conf.enable_telemetry; if let Some(val) = val
            { self.enable_telemetry = val; }
        }
        {
            let val = override_conf.verbose; if override_conf.verbose_unset
            { self.verbose = false; } else if let Some(val) = val
            { self.verbose = val; }
        }
    }
} #[automatically_derived] impl :: std :: ops :: Add < Example3OverrideConfig
> for Example3Config
{
    type Output = Self; fn add(mut self, rhs : Example3OverrideConfig) -> Self
    { self += rhs; self }
} #[automatically_derived] impl :: std :: ops :: AddAssign <
Example3OverrideConfig > for Example3Config
{
    fn add_assign(& mut self, rhs : Example3OverrideConfig)
    { self.merge_with_override(rhs); }
}
#[doc =
" Possible errors encountered during merge of [`Example3OverrideConfig`](struct@Example3OverrideConfig)"]
#[derive(:: std :: fmt :: Debug)] pub enum MergeExample3ConfigError
{
    #[doc = r" Unable to initialize struct due to missing required fields"]
    MissingRequiredFields
    { #[doc = r" Names of missing fields"] field_names : Vec < String > , },
} impl MergeExample3ConfigError
{
    #[doc(hidden)] pub fn
    build_field_chain(& self, chain : & mut Vec < String > , self_field_name :
    :: std :: option :: Option < & str >)
    {
        if let Some(name) = self_field_name { chain.push(name.to_string()); }
        match self { _ => {}, };
    } #[doc = r" Returns the source error message"] #[doc = r""]
    #[doc = r" Used to generate the full error including the nested path"] pub
    fn origin_message(& self) -> String
    { match self { _ => self.to_string(), } }
    #[doc = r" List of field names leading to the source of a nested error."]
    #[doc = r""]
    #[doc =
    r" Does not include the offending field(s) themselves, just the sequence of fields leading to their containing `Struct`"]
    pub fn source_chain(& self) -> Vec < String >
    {
        let mut chain = Vec :: new();
        self.build_field_chain(& mut chain, None); chain
    }
    #[doc =
    r" Same as [`source_chain()`](Self::source_chain), but returns the chain as a single dot-delimited string"]
    pub fn source_path(& self) -> String { self.source_chain().join(".") }
    impl :: std :: fmt :: Display for MergeExample3ConfigError
    {
        fn fmt(& self, f : & mut :: std :: fmt :: Formatter < '_ >) -> :: std
        :: fmt :: Result
        {
            let text = match self
            {
                Self :: MissingRequiredFields { field_names } => format!
                ("Missing required `Example3Config` fields:\n{}", :: lib_conf
                :: __private :: error_field_list(field_names)),
            }; write! (f, text) ? ; Ok(())
        }
    }
}
#[doc =
" Provides a builder used to configure [`Example3Config`](struct@Example3Config) at compile-time."]
#[doc = ""]
#[doc =
" Can be constructed using either [`Example3Config::builder()`](method@Example3Config::builder) or [`Example3ConfigBuilder::new()`](method@Example3ConfigBuilder::new)."]
#[doc = ""]
#[doc =
" For run-time configuration of `Example3Config`, see [`Example3OverrideConfig`](struct@Example3OverrideConfig)"]
#[derive(Debug, Clone)] pub struct Example3ConfigBuilder
{
    pub(crate) inner : Example3Config, pub(crate) override_conf : :: std ::
    option :: Option < Example3OverrideConfig > ,
} impl Example3ConfigBuilder
{
    #[doc = r" Constructs a new builder instance"] #[must_use]
    #[allow(clippy :: redundant_field_names)] pub fn
    new(client_name : String, enable_telemetry : bool) -> Self
    {
        Self
        {
            override_conf : None, inner : < Example3Config > ::
            new(client_name, enable_telemetry),
        }
    } #[doc = " Name of the client app using the library"] #[doc = ""]
    #[doc = " This field is mandatory and cannot be changed at runtime"]
    #[doc = ""] #[doc = " ---"] #[doc = ""] #[must_use] pub fn
    client_name(mut self, val : String) -> Self
    { self.inner.client_name = val; self } #[doc = " Enables usage telemetry"]
    #[doc = ""] #[doc = " This field is mandatory"] #[doc = ""]
    #[doc = " ---"] #[doc = ""] #[must_use] pub fn
    enable_telemetry(mut self, val : bool) -> Self
    { self.inner.enable_telemetry = val; self }
    #[doc = " Enables verbose logging"] #[doc = " # Revert at run-time"]
    #[doc = ""]
    #[doc =
    " Has run-time support for reverting back to the provided default (**`false`**)"]
    #[doc = ""]
    #[doc =
    " See [`verbose_unset()`](method@Example3OverrideConfig::verbose_unset) for more information"]
    #[doc = ""] #[doc = " ---"] #[doc = ""]
    #[doc = " - Library default: **`false`**"] #[must_use] pub fn
    verbose(mut self, val : bool) -> Self { self.inner.verbose = val; self }
    #[doc =
    r" If supplied, will overwrite any values present in the provided"]
    #[doc = r" override config."] #[doc = r""]
    #[doc = r" This happens as the last step within the"]
    #[doc = r"[`build()`](Self::build) method."] #[doc = r""]
    #[doc =
    r" Any previous calls to [`with_override()`](Self::with_override)"]
    #[doc = r" in the builder chain will have no effect on the resulting"]
    #[doc = r" config."] #[doc = r""]
    #[doc = r" The order in which this is chained with the builder's setter"]
    #[doc = r" methods does not matter."] #[must_use] pub fn
    with_override(mut self, override_conf : Example3OverrideConfig,) -> Self
    { self.override_conf = Some(override_conf); self }
    #[doc = r" Clears the override config previously set via"]
    #[doc = r" [`with_override()`](Self::with_override)."] #[must_use] pub fn
    clear_override(mut self) -> Self { self.override_conf = None; self }
    #[doc = " Builds the [`Example3Config`]"] #[doc = r""]
    #[doc =
    r" Values present in an override config (if supplied) will replace"]
    #[doc = r" corresponding assignments made using the builder."] #[must_use]
    pub fn build(self) -> Example3Config
    {
        let merged = if let Some(override_conf) = self.override_conf
        { self.inner + override_conf } else { self.inner }; merged
    }
}
#[doc =
" Provides support for configuring [`Example3Config`](struct@Example3Config) at run-time."]
#[doc = ""]
#[doc =
" A common method of rust app configuration is deserializing a config file or env vars into a *'Config'* struct at run-time."]
#[doc = ""]
#[doc = " `Example3OverrideConfig` is provided for just this case."]
#[doc = ""] #[doc = " ---"] #[doc = ""]
#[doc = " `Example3OverrideConfig` provides run-time support for:"]
#[doc = " - *Setting* [`Example3Config`](struct@Example3Config) fields"]
#[doc =
" - *Overriding* compile-time assignments (made using [`Example3ConfigBuilder`](struct@Example3ConfigBuilder))"]
#[doc =
" - *Reverting* compile-time (or run-time) assignments back to provided defaults"]
#[doc = " # Example"] #[doc = ""] #[doc = " ```rust,ignore"]
#[doc = " #[derive(Debug, Clone, Deserialize)]"] #[doc = " pub struct Conf {"]
#[doc = "     api: ApiConf,"] #[doc = "     // .."]
#[doc = "     #[serde(default)]"]
#[doc = "     example3: Example3OverrideConfig,"] #[doc = " }"]
#[doc = " ```"] #[doc = ""] #[doc = " # Fields"] #[doc = ""]
#[doc = " | Name | Type |"] #[doc = " | - | - |"]
#[doc = " | `enable_telemetry` | `bool` |"] #[doc = " | `verbose` | `bool` |"]
#[doc = ""] #[doc = " ## Revert to default"] #[doc = ""]
#[doc =
" For each of [`Example3Config`](struct@Example3Config)'s optional fields, there is a corresponding **`bool`** with a name in the form of `*_unset`."]
#[doc = ""]
#[doc =
" If set to `true`, the associated field will have its value reverted to the provided default, e.g."]
#[doc = " - if `verbose_unset` deserializes to `true`:"]
#[doc =
"   - `verbose` will be reverted back to the provided default: `false`"]
#[doc = ""] #[doc = " ### Unset fields"] #[doc = ""]
#[doc = " The following fields are supported:"] #[doc = ""]
#[doc = " | Name |"] #[doc = " | - |"] #[doc = " | `verbose_unset` |"]
#[doc = ""] #[doc = " ### Unset aliases"] #[doc = ""]
#[doc = " Defaults can also be restored using the following aliases"]
#[doc = ""] #[doc = " | Alias |"] #[doc = " | - |"] #[doc = " | `reset` |"]
#[doc = " | `revert` |"] #[doc = " | `clear` |"] #[doc = " | `default` |"]
#[doc = ""]
#[doc =
" For example, `verbose` could be reset using either `verbose_unset`, or any of the following:"]
#[doc = " - `verbose_reset`"] #[doc = " - `verbose_revert`"]
#[doc = " - `verbose_clear`"] #[doc = " - `verbose_default`"]
#[derive(Debug, Clone)] #[derive(:: serde :: Deserialize)] pub struct
Example3OverrideConfig
{
    #[doc = " Enables usage telemetry"] #[doc = ""]
    #[doc = " This field is mandatory"] #[doc = ""] #[doc = " ---"]
    #[doc = ""] pub(crate) enable_telemetry : :: std :: option :: Option <
    bool > , #[doc = " Enables verbose logging"] #[doc = ""] #[doc = " ---"]
    #[doc = ""] #[doc = " - Library default: **`false`**"] pub(crate) verbose
    : :: std :: option :: Option < bool > ,
    #[serde(default, alias = "verbose_reset", alias = "verbose_revert", alias
    = "verbose_clear", alias = "verbose_default")]
    #[doc = r" flag allowing for reverting a builder-configured"]
    #[doc = r" setting at runtime"] pub(crate) verbose_unset : bool,
} impl Example3OverrideConfig
{
    #[doc = " Enables usage telemetry"] #[doc = ""]
    #[doc = " This field is mandatory"] #[doc = ""] #[doc = " ---"]
    #[doc = ""] #[must_use] pub fn enable_telemetry(& self) -> :: std ::
    option :: Option < & bool > { self.enable_telemetry.as_ref() }
    #[doc = " Enables verbose logging"] #[doc = ""] #[doc = " ---"]
    #[doc = ""] #[doc = " - Library default: **`false`**"] #[must_use] pub fn
    verbose(& self) -> :: std :: option :: Option < bool > { self.verbose }
    #[doc =
    " Whether the unset field for [`verbose`](Self::verbose) has explicitly been set to `true`."]
    #[doc = ""] #[doc = " # Aliases"] #[doc = ""]
    #[doc =
    " Various aliases are supported in addition to reverting via `verbose_unset`."]
    #[doc = ""]
    #[doc =
    " - **Note:** unset aliases are only supported when the **`serde` feature is enabled**."]
    #[doc = ""] #[doc = " <details>"]
    #[doc =
    " <summary><b>Expand to view supported unset field aliases</b></summary"]
    #[doc = ""] #[doc = " > - `verbose_reset`"]
    #[doc = " > - `verbose_revert`"] #[doc = " > - `verbose_clear`"]
    #[doc = " > - `verbose_default`"] #[doc = ""] #[doc = " </details>"]
    #[must_use] pub fn verbose_unset(& self) -> bool { self.verbose_unset }
} #[automatically_derived] #[allow(clippy :: redundant_field_names)] impl ::
std :: default :: Default for Example3OverrideConfig
{
    fn default() -> Self
    {
        Self
        { enable_telemetry : None, verbose : None, verbose_unset : false, }
    }
}
