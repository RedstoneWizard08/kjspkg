use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use modhost::{from_log_level, init_logger, loaders, GameVersion, ModHost, Result};
use serde::{Deserialize, Serialize};
use std::io::stdout;

pub const PISTON_META_ENDPOINT: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    #[arg(short = 'C', long)]
    pub complete: Option<Shell>,
}

impl Cli {
    fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
        generate(gen, cmd, cmd.get_name().to_string(), &mut stdout());
    }

    pub async fn run(self) -> Result<()> {
        if let Some(shell) = self.complete {
            Self::print_completions(shell, &mut Cli::command());
            return Ok(());
        }

        let _ = dotenvy::dotenv();
        init_logger(from_log_level(self.verbose.log_level_filter()));

        ModHost::new()
            .await?
            .versions(get_minecraft_versions().await?)
            .loaders(loaders!["Forge", "Fabric", "Quilt", "NeoForge",])
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
