use config::{Environment, File};
use serde::Deserialize;

pub mod database;

#[derive(Debug, Deserialize, Clone)]
#[allow(unused)]
pub struct Config {
    pub list: Vec<String>,
    pub database: database::Database,
}

impl Config {
    pub fn new() -> Result<Self, config::ConfigError> {
        let config = config::Config::builder()
            // Add in the default configuration file
            .add_source(vec![
                File::with_name("config.example"),
                File::with_name("config").required(false),
            ])
            // Add in settings from the environment
            .add_source(
                Environment::default()
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(" ")
            )
            .build()?;


        let config = config.try_deserialize::<Self>()?;

        Ok(config)
    }
}