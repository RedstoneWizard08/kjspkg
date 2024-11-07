use std::time::Duration;

use crate::{
    ctx::CliContext,
    util::{get_bar_style, get_spinner_style, pkg::print_package},
};
use eyre::Result;
use indexmap::IndexMap;
use indicatif::ProgressBar;

pub async fn cmd_list_remote(cx: &CliContext) -> Result<()> {
    let pb = ProgressBar::new_spinner().with_style(get_spinner_style());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Fetching packages");

    let pkgs = cx.api.packages().await?;
    let mut data = IndexMap::new();

    pb.finish_and_clear();
    let pb = ProgressBar::new((pkgs.len() - 1) as u64).with_style(get_bar_style());

    for pkg in pkgs {
        pb.set_message(format!("Fetching latest version: {}", pkg.name));

        let ver = cx.api.package(&pkg.slug).latest_version().await.ok();

        data.insert(pkg, ver);
        pb.inc(1);
    }

    pb.finish_and_clear();

    for (pkg, ver) in data {
        print_package(pkg, ver, cx)?;
    }

    Ok(())
}
