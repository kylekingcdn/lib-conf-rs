use lib_conf::LibConfig;

/// Settings for `example00` of `lib-conf`
#[derive(Debug, Copy, Clone, LibConfig)]
#[config(example_field_name = "example00")]
pub struct Example00Config {
    // - Example Notes
    //
    // ..
    //

    /// Enables verbose logging
    #[config(copy, default = false)]
    pub(crate) verbose: bool,

    // ..
}
