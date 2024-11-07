use std::{fs, path::PathBuf};

use color_eyre::Section;
use eyre::{eyre, Result};
use itertools::Itertools;
use kjspkg_api::models::{NewPackage, NewPackageVersion};

use crate::{
    ctx::CliContext,
    manifest::{ModLoader, PackageManifest},
    util::create_slug,
};

use super::pack::cmd_pack;

pub async fn cmd_publish(cx: &CliContext, force: bool) -> Result<()> {
    if !PathBuf::from("kjspkg.json").exists() {
        return Err(
            eyre!("kjspkg.json does not exist!").suggestion("Maybe try running `kjspkg pkg init`?")
        );
    }

    if cx.api.token.is_none() {
        return Err(eyre!("You aren't logged in!").suggestion("Maybe try running `kjspkg login`?"));
    }

    let path = cmd_pack(cx, force).await?;
    let data = fs::read(path)?;
    let manifest = PackageManifest::read(None)?;
    let mut readme = String::new();
    let readme_path = PathBuf::from("README.md");

    if readme_path.exists() {
        readme = fs::read_to_string(readme_path)?;
    }

    let version = NewPackageVersion {
        name: manifest.version.clone(),
        changelog: None,
        kubejs: manifest.kubejs,
        version_number: manifest.version,
        loaders: manifest
            .loaders
            .iter()
            .map(|v| <ModLoader as Into<String>>::into(*v))
            .collect_vec(),
        minecraft: manifest.minecraft,
    };

    let new_pkg = NewPackage {
        description: manifest.description,
        issues: None,
        name: manifest.name.clone(),
        readme,
        slug: create_slug(&manifest.name),
        source: None,
        wiki: None,
    };

    info!("Trying to upload...");

    // TODO: There's a way to simplify this but I'm lazy...
    if let Ok(pkg) = cx.api.package(manifest.name).get().await {
        info!("Package already exists, trying to upload a new version...");

        let api = cx.api.package(pkg.name);

        api.upload_version(version, data).await?;

        info!("Done!");
    } else {
        info!("Creating a new package...");

        let pkg = cx.api.create_package(new_pkg).await?;
        let api = cx.api.package(pkg.name);

        info!("Uploading a new version...");

        api.upload_version(version, data).await?;

        info!("Done!");
    }

    Ok(())
}
