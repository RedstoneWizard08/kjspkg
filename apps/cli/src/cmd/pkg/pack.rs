use crate::{
    ctx::CliContext,
    manifest::PackageManifest,
    util::{get_spinner_style, tar::MaybeAppend},
};
use color_eyre::Section;
use eyre::{eyre, Result};
use flate2::{write::GzEncoder, Compression};
use indicatif::ProgressBar;
use inquire::Confirm;
use std::{env::current_dir, fs::File, path::PathBuf, time::Duration};

pub async fn cmd_pack(_cx: &CliContext, overwrite: bool) -> Result<PathBuf> {
    if !PathBuf::from("kjspkg.json").exists() {
        return Err(
            eyre!("kjspkg.json does not exist!").suggestion("Maybe try running `kjspkg pkg init`?")
        );
    }

    let pb = ProgressBar::new_spinner().with_style(get_spinner_style());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Creating package tarball");

    let root = current_dir()?;
    let data = PackageManifest::read(None)?;
    let path = PathBuf::from(format!("{}_v{}.tgz", data.name, data.version));

    if path.exists() && !overwrite {
        if !Confirm::new("The output file already exists! Are you sure you want to continue?")
            .prompt()?
        {
            return Err(eyre!("The output file already exists!"));
        }
    }

    let file = File::create(&path)?;
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar = tar::Builder::new(enc);

    tar.maybe_append_dir_all("client_scripts", root.join("client_scripts"))?;
    tar.maybe_append_dir_all("setup_scripts", root.join("setup_scripts"))?;
    tar.maybe_append_dir_all("startup_scripts", root.join("startup_scripts"))?;
    tar.maybe_append_dir_all("server_scripts", root.join("server_scripts"))?;
    tar.maybe_append_dir_all("public", root.join("public"))?;
    tar.maybe_append_dir_all("data", root.join("data"))?;
    tar.maybe_append_dir_all("assets", root.join("assets"))?;
    tar.maybe_append_dir_all("config", root.join("config"))?;
    tar.maybe_append_dir_all("src", root.join("src"))?;
    // TODO: Add custom directories in kjspkg.json

    tar.maybe_append_named(root.join("README.md"), "README.md")?;
    tar.maybe_append_named(root.join("README.txt"), "README.txt")?;
    tar.maybe_append_named(root.join("README"), "README")?;
    tar.maybe_append_named(root.join("LICENSE"), "LICENSE")?;

    // TODO: Maybe we should just use walkdir and ignore .gitignore and .git?

    tar.maybe_append_named(root.join("kjspkg.json"), "kjspkg.json")?;

    pb.finish_and_clear();

    Ok(path)
}
