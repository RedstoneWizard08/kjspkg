use std::fs;

use anyhow::{anyhow, Result};
use kjspkg_api::ApiClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct CliContext {
    pub api: ApiClient,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RawCliContext {
    #[serde(default)]
    pub api_base: Option<String>,

    #[serde(default)]
    pub token: Option<String>,
}

impl CliContext {
    pub fn of(raw: RawCliContext) -> Result<Self> {
        Ok(Self {
            api: ApiClient::new(raw.api_base, raw.token)?,
        })
    }

    pub fn save(&self) -> Result<()> {
        let path = dirs::config_dir()
            .ok_or(anyhow!("Could not find config directory!"))?
            .join("kjspkg-cli.json");

        let data = RawCliContext {
            api_base: Some(self.api.api_base.clone()),
            token: self.api.token.clone(),
        };

        fs::write(path, serde_json::to_string(&data)?)?;

        Ok(())
    }
}

pub fn get_ctx() -> Result<CliContext> {
    let path = dirs::config_dir()
        .ok_or(anyhow!("Could not find config directory!"))?
        .join("kjspkg-cli.json");

    if !path.exists() {
        return Ok(CliContext::of(Default::default())?);
    }

    let raw = serde_json::from_str::<RawCliContext>(&fs::read_to_string(path)?)?;

    Ok(CliContext::of(raw)?)
}
