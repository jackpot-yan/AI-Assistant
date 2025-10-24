use serde::Deserialize;
use std::env;
use std::fs;
use std::error::Error;

#[derive(Deserialize, Debug)]
pub struct ModelConfig {
    pub deploy: String,
    pub host: String,
    pub model_name: String,
}

#[derive(Deserialize, Debug)]
pub struct DataBaseConfig {
    pub database_type: String,
    pub database_url: String,
    pub database_name: String,
}

#[derive(Deserialize, Debug)]
pub struct LogConfig {
    pub level: String,
    pub file_path: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub model: ModelConfig,
    pub database: DataBaseConfig,
    pub logging: LogConfig,
}

impl Config {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let cwd = env::current_dir()?;
        let config_path = cwd.join("config.toml");
        let config_str = fs::read_to_string(&config_path)?;
        let cfg:Config = toml::from_str(&config_str)?;
        Ok(cfg)
    }
}
