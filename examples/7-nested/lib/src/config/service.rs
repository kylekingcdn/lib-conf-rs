use lib_conf::LibConfig;
use url::Url;

#[derive(Debug, Clone, LibConfig)]
#[config(example_field_name = "some_service")]
pub struct ServiceConfig {
    pub(crate) endpoint: Url,
}
