#![allow(unused, clippy::pedantic, clippy::allow_attributes, clippy::allow_attributes_without_reason)]

use config::Config;
use example6_lib::{Example6Config, Example6OverrideConfig, RuntimeBackend};
use serde::Deserialize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let example6_config = Example6Config::builder()
        .verbose(true)
        .backend(RuntimeBackend::Tokio) // switch the backend to Tokio
        .with_override(runtime_config.ex6)
        .try_build()
        .unwrap_or_else(|error| panic!("\n{error}\n"));
        //                     ^^^^^^^^ panic on validation failure
        //
        // it's probably more ideal to use `.try_build()?;`.
        // we're handling explicitly to log the error since anyhow/eyre isn't in use

    println!("Library config: {example6_config:#?}");

    Ok(())
}

mod conf {
    use super::*;

    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        #[serde(default)]
        pub ex6: Example6OverrideConfig,
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
