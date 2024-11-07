use std::time::Duration;

use crate::{
    ctx::CliContext,
    util::{get_spinner_style, pkg::print_package},
};
use eyre::Result;
use indexmap::IndexMap;
use indicatif::ProgressBar;

pub async fn cmd_search(cx: &CliContext, query: String) -> Result<()> {
    let pb = ProgressBar::new_spinner().with_style(get_spinner_style());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Searching packages");

    let pkgs = cx.api.search_packages(query).await?;

    if pkgs.is_empty() {
        pb.finish_and_clear();
        info!("No packages found!");
        return Ok(());
    }

    let mut data = IndexMap::new();

    for pkg in pkgs {
        pb.set_message(format!("Fetching latest version: {}", pkg.name));

        let ver = cx.api.package(&pkg.slug).latest_version().await.ok();

        data.insert(pkg, ver);
    }

    pb.finish_and_clear();

    for (pkg, ver) in data {
        print_package(pkg, ver, cx)?;
    }

    Ok(())
}
