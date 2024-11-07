use std::{env::current_dir, fs};

use clap::ValueEnum;
use eyre::{eyre, Result};
use inquire::{validator::Validation, Confirm, MultiSelect, Text};
use itertools::Itertools;
use semver::Version;

use crate::{
    ctx::CliContext,
    manifest::{ModLoader, PackageManifest},
    util::{create_slug, mc::get_minecraft_versions},
};

fn validate_semver(
    data: &str,
) -> Result<Validation, Box<dyn std::error::Error + Send + Sync + 'static>> {
    Version::parse(data)
        .map(|_| Validation::Valid)
        .map_err(|v| Box::new(v).into())
}

pub async fn cmd_init(
    _cx: &CliContext,
    minecraft: Option<Vec<String>>,
    loaders: Option<Vec<ModLoader>>,
    name: Option<String>,
    version: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    force: bool,
) -> Result<()> {
    let name = if let Some(name) = name {
        name
    } else {
        Text::new("What should this package be named?")
            .with_default("my-package")
            .prompt()?
    };

    let version = if let Some(version) = version {
        version
    } else {
        Text::new("What version should this package be at?")
            .with_default("0.1.0")
            .with_validator(validate_semver)
            .prompt()?
    };

    let description = if let Some(desc) = description {
        desc
    } else {
        Text::new("What is a short description of this package?")
            .prompt_skippable()?
            .unwrap_or_default()
    };

    let authors = if let Some(authors) = authors {
        authors
    } else {
        Text::new("Who created this package? (This is a comma-separated list)")
            .prompt()?
            .split(",")
            .map(|v| v.trim().into())
            .collect_vec()
    };

    let minecraft = if let Some(mc) = minecraft {
        mc
    } else {
        MultiSelect::new(
            "Select the Minecraft versions this package should support.",
            get_minecraft_versions().await?,
        )
        .prompt()?
    };

    let loaders = if let Some(loaders) = loaders {
        loaders
    } else {
        MultiSelect::new(
            "Select the mod loaders this package should support.",
            ModLoader::value_variants().to_vec(),
        )
        .prompt()?
    };

    let dir = current_dir()?.join(create_slug(&name));

    if dir.exists() {
        if !fs::read_dir(&dir)?.collect_vec().is_empty() {
            if !force
                && !Confirm::new(
                    "The target directory is not empty! Are you sure you want to continue?",
                )
                .prompt()?
            {
                return Err(eyre!("The target directory was not empty!"));
            }
        }
    } else {
        fs::create_dir_all(&dir)?;
    }

    let manifest = PackageManifest {
        authors,
        dependencies: Vec::new(),
        description,
        incompatibilities: Vec::new(),
        kubejs: Vec::new(),
        minecraft,
        loaders,
        name,
        version,
    };

    fs::write(
        dir.join("kjspkg.json"),
        serde_json::to_string_pretty(&manifest)?,
    )?;

    Ok(())
}
