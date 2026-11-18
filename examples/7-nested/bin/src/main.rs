#![allow(unused, clippy::pedantic, clippy::allow_attributes, clippy::allow_attributes_without_reason)]

use config::Config;
use example7_lib::{Example7Config, Example7OverrideConfig};
use secrecy::SecretString;
use serde::Deserialize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let pkg_name = env!("CARGO_CRATE_NAME");
    let pkg_vers = env!("CARGO_PKG_VERSION");
    let client_name = format!("{pkg_name} v{pkg_vers}");
    let telemetry = true;

    // // required parameters get added to the builder fn params
    // // parameters are added in the order they appear in the struct
    // let example7_config = Example7Config::builder(runtime_config.ex7).build();
    // println!("Library config: {example7_config:#?}");

    Ok(())
}

mod conf {
    use super::*;

    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        #[serde(default)]
        pub ex7: Example7OverrideConfig,
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
