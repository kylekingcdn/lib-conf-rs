#![allow(unused, clippy::pedantic)]

use config::Config;
use example2_lib::{Example2Config, Example2Lib, Example2OverrideConfig};
use secrecy::SecretString;
use serde::Deserialize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let example2_config = Example2Config::builder()
        .log_file_path(Some("/tmp/test.log".to_string()))
        .verbose(true)
        .with_override(runtime_config.ex2)
        .build();
    println!("Library config: {example2_config:#?}");

    let _example2_lib = Example2Lib::new(example2_config);

    Ok(())
}

mod conf {
    use super::*;

    /// Database config for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct DbConfig {
        pub url: SecretString,
        pub max_connections: Option<u32>,
    }

    /// Runtime configuration for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        #[serde(default)]
        pub ex2: Example2OverrideConfig,

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
