use crate::Result;
use app_config::AppConfig;
use std::{fs, path::PathBuf};

pub const DEFAULT_FAVICON_ICO: &[u8] = include_bytes!("./assets/modhost.ico");
pub const DEFAULT_FAVICON_PNG: &[u8] = include_bytes!("./assets/modhost.png");

#[cfg(not(debug_assertions))]
pub const UI_SOURCE: include_dir::Dir<'static> =
    include_dir::include_dir!("$CARGO_MANIFEST_DIR/../../ui");

#[cfg(debug_assertions)]
pub async fn build_ui(config: &AppConfig, dir: &PathBuf) -> Result<()> {
    if config.ui.favicon_ico == "default" {
        info!("Downloading favicon.ico...");

        fs::write(dir.join("static/favicon.ico"), DEFAULT_FAVICON_ICO)?;
    } else {
        info!("Downloading favicon.ico...");

        let data = if config.ui.favicon_ico.starts_with("http") {
            reqwest::get(&config.ui.favicon_ico).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_ico)?.into()
        };

        fs::write(dir.join("static/favicon.ico"), data)?;
    }

    if config.ui.favicon_png == "default" {
        info!("Downloading favicon.png...");

        fs::write(dir.join("static/favicon.png"), DEFAULT_FAVICON_PNG)?;
    } else {
        info!("Downloading favicon.png...");

        let data = if config.ui.favicon_png.starts_with("http") {
            reqwest::get(&config.ui.favicon_png).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_png)?.into()
        };

        fs::write(dir.join("static/favicon.png"), data)?;
    }

    Ok(())
}

#[cfg(not(debug_assertions))]
pub async fn build_ui(config: &AppConfig) -> Result<PathBuf> {
    use tempfile::TempDir;
    use tokio::process::Command;

    let bun_exe = crate::bun::get_bun_exe().await?;

    info!("Extracting UI...");

    let dir = TempDir::new()?.into_path();

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    info!("Extracting to: {:?}", dir);

    UI_SOURCE.extract(&dir)?;

    if config.ui.favicon_ico == "default" {
        info!("Downloading favicon.ico...");

        fs::write(dir.join("static/favicon.ico"), DEFAULT_FAVICON_ICO)?;
    } else {
        info!("Downloading favicon.ico...");

        let data = if config.ui.favicon_ico.starts_with("http") {
            reqwest::get(&config.ui.favicon_ico).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_ico)?.into()
        };

        fs::write(dir.join("static/favicon.ico"), data)?;
    }

    if config.ui.favicon_png == "default" {
        info!("Downloading favicon.png...");

        fs::write(dir.join("static/favicon.png"), DEFAULT_FAVICON_PNG)?;
    } else {
        info!("Downloading favicon.png...");

        let data = if config.ui.favicon_png.starts_with("http") {
            reqwest::get(&config.ui.favicon_png).await?.bytes().await?
        } else {
            fs::read(&config.ui.favicon_png)?.into()
        };

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
