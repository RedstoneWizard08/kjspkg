#![cfg(not(debug_assertions))]

use crate::{bun::get_bun_exe, Result};
use app_config::AppConfig;
use include_dir::{include_dir, Dir};
use std::{fs, path::PathBuf};
use tempfile::TempDir;
use tokio::process::Command;

pub const UI_SOURCE: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../../ui");

pub async fn build_ui(config: &AppConfig) -> Result<PathBuf> {
    let bun_exe = get_bun_exe().await?;

    info!("Extracting UI...");

    let dir = TempDir::new()?.into_path();

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    info!("Extracting to: {:?}", dir);

    UI_SOURCE.extract(&dir)?;

    if config.ui.favicon_ico != "default" {
        info!("Downloading favicon.ico...");

        let data = reqwest::get(&config.ui.favicon_ico).await?.bytes().await?;

        fs::write(dir.join("static/favicon.ico"), data)?;
    }

    if config.ui.favicon_png != "default" {
        info!("Downloading favicon.png...");

        let data = reqwest::get(&config.ui.favicon_png).await?.bytes().await?;

        fs::write(dir.join("static/favicon.png"), data)?;
    }

    info!("Running `bun install`...");

    Command::new(&bun_exe)
        .arg("install")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Running `bun run sync`...");

    Command::new(&bun_exe)
        .arg("--bun")
        .arg("run")
        .arg("sync")
        .env("NODE_ENV", "production")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Running `bun run build`...");

    Command::new(&bun_exe)
        .arg("--bun")
        .arg("run")
        .arg("dist")
        .env("NODE_ENV", "production")
        .envs(config.ui.env())
        .current_dir(&dir)
        .spawn()?
        .wait()
        .await?;

    info!("Successfully built the UI!");

    fs::remove_file(bun_exe)?;

    let dir_clone = dir.as_os_str().to_os_string();

    ctrlc::set_handler(move || {
        info!("Caught exit! Cleaning up...");

        fs::remove_dir_all(&dir_clone).unwrap();
        jsglue::abort::on_exit();
    })
    .unwrap();

    Ok(dir.join("build"))
}
