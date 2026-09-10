#![allow(unused, clippy::pedantic)]

use config::Config;
use example1_lib::{Example1Config, Example1Lib, Example1OverrideConfig};
use secrecy::SecretString;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let example1_config = Example1Config::builder()
        .log_file_path(Some("/tmp/test.log".to_string()))
        .log_rotate_interval(Duration::from_secs(1))
        .with_override(runtime_config.ex1)
        .build();
    println!("Library config: {example1_config:#?}");

    let _example1_lib = Example1Lib::new(example1_config);

    Ok(())
}

mod conf {
    use super::*;

    /// Database config for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct DbConfig {
        pub url: SecretString,
        pub max_connections: u32,
    }

    /// Runtime configuration for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        #[serde(default)]
        pub ex1: Example1OverrideConfig,

        // not related to `lib-conf`, this is just a common pattern for handling bin config
        pub db: DbConfig
    }
    impl AppRuntimeConfig {
        /// loads runtime config from .env file
        pub fn try_load() -> Result<Self, Box<dyn Error>> {
            dotenvy::dotenv()?;
            let parsed = Config::builder()
                .add_source(config::Environment::with_prefix("APP").separator("__"))
                .build()?
                .try_deserialize()?;

            Ok(parsed)
        }
    }
}
