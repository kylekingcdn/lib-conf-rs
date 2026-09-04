mod common;

#[derive(Debug, Clone, lib_conf::LibConfig)]
pub struct TestConfig {
    #[config(default = true)]
    pub foo: bool,

    pub init_me: bool,
}

#[test]
fn main() {
    let override_config = common::load_env_and_config_default(file!());

    let config = TestConfig::builder(true)
        .with_override(override_config)
        .foo(false)
        .build();
    assert_eq!(config.foo(), true);
    assert_eq!(config.init_me(), true);
}
