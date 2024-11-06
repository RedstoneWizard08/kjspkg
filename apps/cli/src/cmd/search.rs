use crate::{ctx::CliContext, util::print_package};
use eyre::Result;

pub async fn cmd_search(cx: &CliContext, query: String) -> Result<()> {
    let pkgs = cx.api.search_packages(query).await?;

    if pkgs.is_empty() {
        info!("No packages found!");
        return Ok(());
    }

    for pkg in pkgs {
        print_package(pkg, cx).await?;
    }

    Ok(())
}
