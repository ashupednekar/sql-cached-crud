use config::{Config, ConfigError, Environment};
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings{
    pub database_url: String,
    pub database_max_connections: i32
}

impl Settings{
    fn new() -> Result<Self, ConfigError>{
        Config::builder()
            .add_source(Environment::default())
            .build()?
            .try_deserialize()
    }
}

lazy_static!{
    pub static ref settings: Settings = Settings::new().expect("improperly configured");
}
