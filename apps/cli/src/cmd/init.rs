use crate::{
    ctx::CliContext,
    manifest::{ModLoader, ProjectManifest},
    util::versions::VersionManifestV2,
};
use clap::ValueEnum;
use eyre::{eyre, Result};
use inquire::{Confirm, Select};
use itertools::Itertools;
use std::{env::current_dir, fs};

pub const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn cmd_init(
    _cx: &CliContext,
    minecraft: Option<String>,
    loader: Option<ModLoader>,
    force: bool,
) -> Result<()> {
    let dir = current_dir()?;
    let items = fs::read_dir(&dir)?.collect_vec();

    if !items.is_empty() {
        if !force && !Confirm::new("The current directory is not empty! Are you sure you want to create a project here?").with_default(false).prompt()? {
            return Err(eyre!("Directory was not empty!"));
        }
    }

    let minecraft = if let Some(mc) = minecraft {
        mc
    } else {
        let data = reqwest::get(VERSION_MANIFEST_URL)
            .await?
            .json::<VersionManifestV2>()
            .await?;

        let versions = data.versions.iter().map(|v| v.id.clone()).collect_vec();

        Select::new("Select the Minecraft version for this project.", versions).prompt()?
    };

    let loader = if let Some(loader) = loader {
        loader
    } else {
        Select::new(
            "Select the mod loader for this project.",
            ModLoader::value_variants().to_vec(),
        )
        .prompt()?
    };

    ProjectManifest::new(dir.join("kubejs"), minecraft, loader).save()?;

    Ok(())
}
