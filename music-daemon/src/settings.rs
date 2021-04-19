use std::env;

use config::{Config, ConfigError, Environment, File};
use log::LevelFilter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(remote = "LevelFilter")]
enum LevelFilterDef {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub file: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiServer {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicDirectory {
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(with = "LevelFilterDef")]
    pub log_level: LevelFilter,
    pub database: Database,
    pub server: ApiServer,
    pub music: Vec<MusicDirectory>,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // Merge default config file
        s.merge(File::with_name("config/default"))?;

        // Merge env file
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Merge environment
        s.merge(Environment::with_prefix("app"))?;

        s.try_into()
    }
}
