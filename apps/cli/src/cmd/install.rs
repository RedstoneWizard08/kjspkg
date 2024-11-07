use crate::{
    ctx::CliContext, manifest::ProjectManifest, pkg::install::install_package,
    util::parse_pkg_input,
};
use color_eyre::Section;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::path::PathBuf;

pub async fn cmd_install(
    cx: &CliContext,
    packages: Vec<String>,
    skip_missing: bool,
    update: bool,
) -> Result<()> {
    if !PathBuf::from("project.json").exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    let mut data = ProjectManifest::read(None)?;

    for pkg in packages {
        let (pkg, ver) = parse_pkg_input(pkg);
        let api = cx.api.package(&pkg);

        match api.get().await {
            Ok(info) => {
                if !update && data.packages.contains_key(&info.slug) {
                    warn!(
                        "Package \"{}\" is already installed! Skipping...",
                        info.slug
                    );

                    continue;
                }

                match ver {
                    Some(ver) => match api.version(&ver).get().await {
                        Ok(version) => {
                            info!(
                                "Installing package \"{}\" at version {}...",
                                info.slug, version.version_number
                            );

                            let file = api.version(version.id.to_string()).download().await?;

                            install_package(info.slug, version.version_number, file, &mut data)?;
                        }

                        Err(_) => {
                            return Err(eyre!(
                                "Could not find version \"{}\" for package \"{}\"!",
                                ver,
                                info.slug
                            ));
                        }
                    },

                    None => match api.latest_version().await {
                        Ok(version) => {
                            info!(
                                "Installing package \"{}\" at version {}...",
                                info.slug, version.version_number
                            );

                            let file = api.version(version.id.to_string()).download().await?;

                            install_package(info.slug, version.version_number, file, &mut data)?;
                        }

                        Err(_) => {
                            return Err(eyre!(
                                "Could not find a version for package \"{}\"!",
                                info.slug
                            ));
                        }
                    },
                }
            }

            Err(err) => {
                let err = format!("{}", err)
                    .lines()
                    .map(|v| format!(" > {}", v))
                    .join("\n");

                if skip_missing {
                    warn!("Could not find package \"{}\"!\nDetails:\n{}", pkg, err);
                    continue;
                } else {
                    return Err(eyre!(
                        "Could not find package \"{}\"!\nDetails:\n{}",
                        pkg,
                        err
                    ));
                }
            }
        }
    }

    data.save()?;

    Ok(())
}
