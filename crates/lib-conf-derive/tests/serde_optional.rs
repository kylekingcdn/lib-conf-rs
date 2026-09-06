#![cfg(not(feature = "serde"))]

#[derive(Debug, Clone, lib_conf_derive::LibConfig)]
pub struct TestConfig {
    #[config(override_required)]
    pub runtime_only: String,
    
    #[config(copy, default = 1)]
    pub foo: u16,
    
    #[config(copy)]
    pub bar: Option<u16>,

    #[config(copy)]
    pub init_me: bool,
}

#[test]
fn main() {
    let override_config = TestOverrideConfig {
        runtime_only: "hunter2".to_string(),
        foo: Some(50),
        bar: Some(10),
        init_me: None,

        foo_unset: true,
        bar_unset: false,
    };
    let config = TestConfig::builder(override_config, true)
        .foo(90)
        .bar(None)
        .build();
    assert_eq!(config.runtime_only(), "hunter2");
    assert_eq!(config.foo(), 1);
    assert_eq!(config.bar(), Some(10));
    assert_eq!(config.init_me(), true);
}
