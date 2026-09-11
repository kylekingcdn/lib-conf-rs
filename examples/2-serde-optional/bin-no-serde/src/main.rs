// prevent workspace build failures with --all-features
#![cfg(not(feature = "serde"))]
#![allow(unused, clippy::pedantic)]

use example2_lib::{Example2Config, Example2Lib, Example2OverrideConfig};
use secrecy::SecretString;
use std::error::Error;
use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime_config = conf::AppRuntimeConfig::try_load().unwrap();
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
    #[derive(Debug, Clone)]
    pub struct DbConfig {
        pub url: SecretString,
        pub max_connections: Option<u32>,
    }
    impl DbConfig {
        /// loads db config from .env file
        pub fn try_load() -> Result<Self, Box<dyn Error>> {
            
            let max_connections = if let Ok(conn) = env::var("APP__DB__MAX_CONNECTIONS") {
                Some(conn.parse::<u32>()?)
            } else {
                None
            };
            Ok(Self {
                url: env::var("APP__DB__URL")?.parse::<String>()?.into(),
                max_connections: env::var("APP__DB__MAX_CONNECTIONS")
                    .ok().map(|x| x.parse()).transpose()?,
            })
        }
    }

    /// Runtime configuration for my app
    #[derive(Debug, Clone)]
    pub struct AppRuntimeConfig {
        pub ex2: Example2OverrideConfig,

        // not related to `lib-conf`, this is just a common pattern for handling bin config
        pub db: DbConfig
    }
    impl AppRuntimeConfig {
        /// loads runtime config from .env file
        pub fn try_load() -> Result<Self, Box<dyn Error>> {
            // inject .env file content into env
            dotenvy::dotenv()?;

            let ex2 = Example2OverrideConfig {
                verbose: env::var("APP__EX2__VERBOSE")
                    .ok().map(|x| x.parse()).transpose()?,
                verbose_unset: env::var("APP__EX2__VERBOSE_UNSET")
                    .unwrap_or(false.to_string()).parse()?,

                log_file_path: env::var("APP__EX2__LOG_FILE_PATH")
                    .ok().map(|x| x.parse()).transpose()?,
                log_file_path_unset: env::var("APP__EX2__LOG_FILE_PATH_UNSET")
                    .unwrap_or(false.to_string()).parse()?,
            };

            Ok(Self {
                ex2,
                db: DbConfig::try_load()?,
            })
        }
    }
}
