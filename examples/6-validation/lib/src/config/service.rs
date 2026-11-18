use lib_conf::LibConfig;

#[derive(Debug, Copy, Clone, LibConfig)]
#[config(example_field_name = "some_service")]
pub struct ServiceConfig {
    #[config(copy, default = false)]
    pub(crate) test: bool,
}
