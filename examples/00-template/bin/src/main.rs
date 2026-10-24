#![allow(unused, clippy::pedantic)]

use config::Config;
use example00_lib::{Example00Config, Example00OverrideConfig};
use serde::Deserialize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let example00_config = Example00Config::builder()
        .verbose(true)
        .with_override(runtime_config.ex00)
        .build();

    println!("Library config: {example00_config:#?}");

    Ok(())
}

mod conf {
    use super::*;

    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        #[serde(default)]
        pub ex00: Example00OverrideConfig,
    }
    impl AppRuntimeConfig {
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
