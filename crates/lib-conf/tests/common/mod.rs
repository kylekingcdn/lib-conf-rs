use config::{Config, Environment};
use std::path::Path;

pub fn load_test_env_file(test_path: &str) {
    let mut env_path = std::env::current_dir().unwrap();
    // fixme: hacky path fixup
    env_path.pop();
    env_path.pop();
    println!("current dir: {}", env_path.display());
    let env_relative_path = test_path.replace(".rs", ".env");
    env_path.push(&env_relative_path);

    eprintln!("loading file: {}", env_path.display());
    dotenvy::from_path(Path::new(&env_path)).unwrap();
}

pub fn load_config<'a, T: serde::Deserialize<'a>>(prefix: &str, separator: &str) -> T {
    let env_src = Environment::with_prefix(prefix).separator(separator);
    let conf = Config::builder().add_source(env_src).build().unwrap();
    conf.try_deserialize().unwrap()
}

pub fn load_env_and_config<'a, T: serde::Deserialize<'a>>(
    test_path: &str,
    prefix: &str,
    separator: &str,
) -> T {
    load_test_env_file(test_path);
    load_config(prefix, separator)
}

pub fn load_env_and_config_default<'a, T: serde::Deserialize<'a>>(
    test_path: &str,
) -> T {
    load_env_and_config(test_path, "TEST", "__")
}
