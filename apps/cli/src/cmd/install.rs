use crate::{
    ctx::CliContext,
    manifest::ProjectManifest,
    pkg::install::install_package,
    util::{get_spinner_style, parse_pkg_input},
};
use color_eyre::Section;
use eyre::{eyre, Result};
use indicatif::{MultiProgress, ProgressBar};
use itertools::Itertools;
use parking_lot::RwLock;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::runtime::Handle;
use std::{path::PathBuf, sync::Arc, time::Duration};

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

    let real_data = ProjectManifest::read(None)?;
    let existing = real_data.packages.clone();
    let data = RwLock::new(real_data);
    let master = Arc::new(MultiProgress::new());
    let par_master = Arc::clone(&master);
    let rt = Handle::current();

    let results = packages
        .par_iter()
        .map(|pkg| {
            let (pkg, ver) = parse_pkg_input(pkg);
            let api = cx.api.package(&pkg);
            let pb = par_master.add(ProgressBar::new_spinner().with_style(get_spinner_style()));

            pb.enable_steady_tick(Duration::from_millis(100));
            pb.set_message(format!("Fetching package info: {}", pkg));

            match api.get_sync(&rt) {
                Ok(info) => {
                    if !update && existing.contains_key(&info.slug) {
                        pb.finish_and_clear();

                        warn!(
                            "Package \"{}\" is already installed! Skipping...",
                            info.slug
                        );

                        return Ok(());
                    }

                    pb.set_message(format!("Fetching version info: {}", info.slug));

                    match ver {
                        Some(ver) => match api.version(&ver).get_sync(&rt) {
                            Ok(version) => {
                                pb.set_message(format!(
                                    "Installing package: {}@{}",
                                    info.slug, version.version_number
                                ));

                                let file = api.version(version.id.to_string()).download_sync(&rt)?;

                                install_package(
                                    info.slug,
                                    version.version_number,
                                    file,
                                    &mut *data.write(),
                                )?;
                                pb.finish_and_clear();

                                Ok(())
                            }

                            Err(_) => {
                                pb.finish_and_clear();

                                Err(eyre!(
                                    "Could not find version \"{}\" for package \"{}\"!",
                                    ver,
                                    info.slug
                                ))
                            }
                        },

                        None => match api.latest_version_sync(&rt) {
                            Ok(version) => {
                                pb.set_message(format!(
                                    "Installing package: {}@{}",
                                    info.slug, version.version_number
                                ));

                                let file = api.version(version.id.to_string()).download_sync(&rt)?;

                                install_package(
                                    info.slug,
                                    version.version_number,
                                    file,
                                    &mut *data.write(),
                                )?;
                                pb.finish_and_clear();

                                Ok(())
                            }

                            Err(_) => {
                                pb.finish_and_clear();

                                Err(eyre!(
                                    "Could not find a version for package \"{}\"!",
                                    info.slug
                                ))
                            }
                        },
                    }
                }

                Err(err) => {
                    pb.finish_and_clear();

                    let err = format!("{}", err)
                        .lines()
                        .map(|v| format!(" > {}", v))
                        .join("\n");

                    if skip_missing {
                        warn!("Could not find package \"{}\"!\nDetails:\n{}", pkg, err);
                        Ok(())
                    } else {
                        Err(eyre!(
                            "Could not find package \"{}\"!\nDetails:\n{}",
                            pkg,
                            err
                        ))
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    for item in results {
        item?; // This is SO dumb LMFAO
    }

    master.clear()?;
    data.read().save()?;

    Ok(())
}
