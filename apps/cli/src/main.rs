use anyhow::Result;
use kjspkg::{manifest::ProjectManifest, pkg::install::install_package};
use std::{env::current_dir, fs, path::PathBuf};

#[tokio::main]
pub async fn main() -> Result<()> {
    let dir_base = PathBuf::from("kubejs_test");
    let kjs_dir = dir_base.strip_prefix(current_dir()?)?;
    let mut data = ProjectManifest::new(kjs_dir);

    install_package(
        "bad-apple",
        "0.0.0",
        fs::read("../../packages/bad-apple_v0.0.0.tgz")?,
        &mut data,
    )?;

    data.save()?;

    Ok(())
}
