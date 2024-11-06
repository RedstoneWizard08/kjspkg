use crate::{ctx::CliContext, manifest::ProjectManifest};
use color_eyre::Section;
use eyre::{eyre, Result};
use std::{fs, path::PathBuf};

pub async fn cmd_uninstall(
    _cx: &CliContext,
    packages: Vec<String>,
    skip_missing: bool,
) -> Result<()> {
    if !PathBuf::from("project.json").exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    let mut data = ProjectManifest::read(None)?;

    for pkg in packages {
        match data.packages.get(&pkg) {
            Some(info) => {
                info!("Uninstalling package \"{}\"...", info.name);

                for item in &info.files {
                    let real = data.root.join(item);

                    if real.exists() {
                        if fs::metadata(&real)?.is_dir() {
                            fs::remove_dir_all(real)?;
                        } else {
                            fs::remove_file(real)?;
                        }
                    }
                }

                data.packages.remove(&pkg);
            }

            None => {
                if skip_missing {
                    warn!("Cannot find package \"{}\"! Skipping...", pkg);
                    continue;
                } else {
                    return Err(eyre!("Cannot find package \"{}\"!", pkg));
                }
            }
        }
    }

    data.save()?;

    Ok(())
}
