use lib_conf_derive::LibConfig;

#[derive(Debug, Clone, LibConfig)]
struct TestConfig {
    #[config(override_required, override_skip)]
    pub(crate) foo: bool,
}
