use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use modhost::{from_log_level, init_logger, GameVersion, ModHost, Result};
use serde::{Deserialize, Serialize};

pub const PISTON_META_ENDPOINT: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        dotenvy::dotenv()?;
        init_logger(from_log_level(self.verbose.log_level_filter()));

        ModHost::new()
            .await?
            .versions(get_minecraft_versions().await?)
            .router()
            .run()
            .await?;

        Ok(())
    }
}

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftVersionInfo {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinecraftLatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PistonManifest {
    pub latest: MinecraftLatestVersions,
    pub versions: Vec<MinecraftVersionInfo>,
}

pub async fn get_minecraft_versions() -> Result<Vec<GameVersion>> {
    let manifest: PistonManifest = reqwest::get(PISTON_META_ENDPOINT).await?.json().await?;

    Ok(manifest
        .versions
        .iter()
        .map(|v| GameVersion {
            id: v.id.clone(),
            beta: v.kind != "release",
        })
        .collect())
}
