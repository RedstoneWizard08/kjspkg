use crate::{ctx::CliContext, util::print_package};
use eyre::Result;

pub async fn cmd_list_remote(cx: &CliContext) -> Result<()> {
    let pkgs = cx.api.packages().await?;

    for pkg in pkgs {
        print_package(pkg, cx).await?;
    }

    Ok(())
}
