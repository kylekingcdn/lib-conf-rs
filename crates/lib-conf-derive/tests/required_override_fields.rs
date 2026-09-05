mod common;

#[derive(Debug, Clone, lib_conf_derive::LibConfig)]
pub struct TestConfig {
    #[config(override_required)]
    pub runtime_only: String,
    
    #[config(copy, default = 1)]
    pub foo: u16,

    #[config(copy)]
    pub init_me: bool,
}

#[test]
fn main() {
    let override_config = common::load_env_and_config_default(file!());

    let config = TestConfig::builder(override_config, true)
        .foo(5)
        .build();
    assert_eq!(config.runtime_only(), &String::from("hunter2"));
    assert_eq!(config.foo(), 5);
    assert_eq!(config.init_me(), true);
}
