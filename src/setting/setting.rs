use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub port: u16,
    pub env: String,
}

impl Configs {
    pub fn init() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name("config/config").required(false))
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;

        config.try_deserialize()
    }
}
