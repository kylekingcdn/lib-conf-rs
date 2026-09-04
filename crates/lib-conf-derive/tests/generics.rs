mod common;

#[derive(Debug, Clone, lib_conf_derive::LibConfig)]
pub struct TestConfig<T: Default + Clone + std::fmt::Debug, S>
where
    S: Copy,
{
    pub req_t: T,

    #[config(default = T::default())]
    pub def_t: T,

    pub opt_t: Option<T>,
    
    #[config(override_skip, copy)]
    pub opt_s: Option<S>,
    
    #[config(skip_all, default = true)]
    pub all_skipped: bool,
}

fn load_override<'a, T, S>() -> TestOverrideConfig<T, S>
where
    T: Default + Clone + std::fmt::Debug + serde::Deserialize<'a>,
    S: Copy
{
    common::load_env_and_config_default(file!())
}

fn load_override_string() -> TestOverrideConfig<String, bool> {
    load_override()
}

static REQ_DEF_VAL: &str = "required_default";
static COMPILE_VAL: &str = "compile_val";
static RUNTIME_VAL: &str = "runtime_val";

fn main() {
    default_works();
    compile_time_works();
    override_works();
}

fn builder_default() -> TestConfigBuilder<String, bool> {
    TestConfig::builder(String::from(REQ_DEF_VAL))
}

#[test]
fn default_works() {
    let config = builder_default().build();
    assert_eq!(config.req_t(), &String::from(REQ_DEF_VAL));
    assert_eq!(config.def_t(), &String::default());
    assert_eq!(config.opt_t(), &None);
    assert_eq!(config.opt_s(), None);
}

#[test]
fn compile_time_works() {
    let config = builder_default()
        .req_t(String::from(COMPILE_VAL))
        .def_t(String::from(COMPILE_VAL))
        .opt_t(Some(String::from(COMPILE_VAL)))
        .opt_s(Some(true))
        .build();
    assert_eq!(config.req_t(), &String::from(COMPILE_VAL));
    assert_eq!(config.def_t(), &String::from(COMPILE_VAL));
    assert_eq!(config.opt_t(), &Some(String::from(COMPILE_VAL)));
    assert_eq!(config.opt_s(), Some(true));
}

#[test]
fn override_works() {
    let override_config = load_override_string();
    println!("override config: {override_config:#?}");
    assert_eq!(override_config.req_t(), &Some(String::from(RUNTIME_VAL)));
    assert_eq!(override_config.def_t(), &Some(String::from(RUNTIME_VAL)));
    assert_eq!(override_config.opt_t(), &Some(String::from(RUNTIME_VAL)));

    // TODO: add unset getters for consistency
    
    let config = builder_default()
        .req_t(String::from(COMPILE_VAL))
        .def_t(String::from(COMPILE_VAL))
        .opt_t(Some(String::from(COMPILE_VAL)))
        .opt_s(Some(true))
        .with_override(override_config)
        .build();
    assert_eq!(config.req_t(), &String::from(RUNTIME_VAL));
    assert_eq!(config.def_t(), &String::default()); // has unset
    assert_eq!(config.opt_t(), &Some(String::from(RUNTIME_VAL)));
    assert_eq!(config.opt_s(), Some(true)); // has skip override
    
    println!("output config: {config:#?}");
}