use crate::{AuthConfigs, PostgresConfig, StorageConfig, UIConfig};
use app_core::Result;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub postgres: PostgresConfig,
    pub auth: AuthConfigs,
    pub storage: StorageConfig,
    pub ui: UIConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 4000,
        }
    }
}

impl AppConfig {
    pub fn save(&self) -> Result<()> {
        fs::write("ModHost.toml", toml::to_string_pretty(self)?)?;

        Ok(())
    }
}
