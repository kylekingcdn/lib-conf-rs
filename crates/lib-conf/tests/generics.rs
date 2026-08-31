mod common;

#[derive(Debug, Clone, lib_conf::LibConfig)]
pub struct TestConfig<T: Default + Clone + std::fmt::Debug> {
    // TODO: test where clause

    #[config(default = true)]
    pub dog: bool,

    #[config(override_skip)]
    pub cat: Option<T>,
}

fn load_override<'a, T>() -> TestOverrideConfig<T>
where
    T: Default + Clone + std::fmt::Debug + serde::Deserialize<'a>
{
    common::load_env_and_config_default(file!())
}

fn load_override_string() -> TestOverrideConfig<String> {
    load_override()
}

#[test]
fn main() {
    let override_config = load_override_string();
    println!("override config: {override_config:#?}");

    let mut builder = TestConfig::builder();
    println!("builder default config: {builder:#?}");
    builder = builder.with_override(override_config);
    println!("builder w/ override config: {builder:#?}");
    builder = builder.dog(false);
    println!("builder w/ runtime config: {builder:#?}");

    let config = builder.build();
    println!("output config: {config:#?}");
}
