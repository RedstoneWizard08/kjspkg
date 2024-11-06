use super::install::cmd_install;
use crate::{ctx::CliContext, manifest::ProjectManifest};
use color_eyre::Section;
use eyre::{eyre, Result};
use itertools::Itertools;
use std::path::PathBuf;

pub async fn cmd_update_all(cx: &CliContext) -> Result<()> {
    if !PathBuf::from("project.json").exists() {
        return Err(
            eyre!("project.json does not exist!").suggestion("Maybe try running `kjspkg init`?")
        );
    }

    let data = ProjectManifest::read(None)?;

    cmd_install(cx, data.packages.keys().cloned().collect_vec(), false, true).await
}
