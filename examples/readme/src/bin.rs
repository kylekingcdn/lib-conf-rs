#![allow(unused, clippy::pedantic)]
pub(crate) use lib_conf_example_readme::crab_log; // emulate dep

use crab_log::CrabLogConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read app config from .env
    let runtime_config = conf::AppRuntimeConfig::try_load()?;
    println!("Runtime config: {runtime_config:#?}");

    let _crab_config = CrabLogConfig::builder()
        .log_file_path(Some("/tmp/test.log".to_string()))
        .with_override(runtime_config.crab)
        .build();

    Ok(())
}

mod conf {
    use super::crab_log; // emulate dep

    use config::Config;
    use crab_log::{CrabLogConfig, CrabLogOverrideConfig};
    use serde::Deserialize;

    /// Database config for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct DbConfig {
        pub url: String,
        pub max_connections: u32,
    }

    /// Runtime configuration for my app
    #[derive(Debug, Clone, Deserialize)]
    pub struct AppRuntimeConfig {
        pub db: DbConfig,

        #[serde(default)]
        pub crab: CrabLogOverrideConfig,
    }
    impl AppRuntimeConfig {
        /// loads runtime config from .env file
        pub fn try_load() -> Result<Self, Box<dyn std::error::Error>> {
            dotenvy::dotenv()?;

            let parsed = Config::builder()
                .add_source(config::Environment::with_prefix("APP").separator("__"))
                .build()?
                .try_deserialize()?;

            Ok(parsed)
        }
    }
}
