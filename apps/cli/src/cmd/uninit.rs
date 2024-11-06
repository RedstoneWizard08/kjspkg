use crate::{ctx::CliContext, manifest::ProjectManifest};
use color_eyre::Section;
use eyre::{eyre, Result};
use inquire::Confirm;
use std::{env::current_dir, fs};

pub async fn cmd_uninit(_cx: &CliContext, confirm: bool) -> Result<()> {
    let path = current_dir()?.join("project.json");

    if !path.exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    if !confirm
        && !Confirm::new("Are you sure you want to delete your project?")
            .with_default(false)
            .prompt()?
    {
        return Ok(());
    }

    let data = ProjectManifest::read(current_dir().ok())?;

    for (_, pkg) in data.packages {
        for item in pkg.files {
            let real = data.root.join(item);

            if real.exists() {
                if fs::metadata(&real)?.is_dir() {
                    fs::remove_dir_all(real)?;
                } else {
                    fs::remove_file(real)?;
                }
            }
        }
    }

    let temp_root = data.root.join(".kjspkg");

    if temp_root.exists() {
        fs::remove_dir_all(temp_root)?;
    }

    fs::remove_file(path)?;

    Ok(())
}
