use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub connection: ConnectionConfig,
    pub defaults: DefaultsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub base_url: String,
    pub websocket_url: String,
    pub timeout: u64,
    pub retry_attempts: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub output_format: String,
    pub log_level: String,
    pub auto_connect: bool,
    pub follow_logs: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig {
                base_url: "https://api.max.dev".to_string(),
                websocket_url: "wss://api.max.dev/socket/websocket".to_string(),
                timeout: 30,
                retry_attempts: 3,
            },
            defaults: DefaultsConfig {
                output_format: "table".to_string(),
                log_level: "info".to_string(),
                auto_connect: true,
                follow_logs: false,
            },
        }
    }
}

impl Config {
    pub fn load(path: Option<PathBuf>) -> Result<Self> {
        match path {
            Some(path) => {
                let settings = config::Config::builder()
                    .add_source(config::File::from(path))
                    .build()?;
                Ok(settings.try_deserialize()?)
            }
            None => Ok(Self::default()),
        }
    }

    pub fn config_dir() -> Result<PathBuf> {
        dirs::config_dir()
            .map(|dir| dir.join("rig"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))
    }

    pub fn default_config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }
}
