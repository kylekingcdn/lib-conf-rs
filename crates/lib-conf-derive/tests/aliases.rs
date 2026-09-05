mod common;

#[derive(Debug, Clone, lib_conf_derive::LibConfig)]
pub struct TestSettings {
    #[config(copy, default = 1)]
    pub foo: u16,
    #[config(copy, default = 1)]
    pub bar: u16,
    #[config(copy, default = 1)]
    pub baz: u16,
    #[config(copy, default = 1)]
    pub baz2: u16,
    #[config(copy, default = 1)]
    pub baz3: u16,
    #[config(copy, default = 1)]
    pub baz4: u16,
}

#[test]
fn main() {
    let override_settings = common::load_env_and_config_default(file!());
    println!("override settings: {override_settings:#?}");

    let settings = TestSettings::builder()
        .with_override(override_settings)
        .foo(5)
        .bar(10)
        .baz(10)
        .baz2(10)
        .baz3(10)
        .baz4(10)
        .build();
    println!("settings: {settings:#?}");

    assert_eq!(settings.foo(), 5);
    assert_eq!(settings.bar(), 2000);
    assert_eq!(settings.baz(), 1);
    assert_eq!(settings.baz2(), 1);
    assert_eq!(settings.baz3(), 1);
    assert_eq!(settings.baz4(), 1);
}
