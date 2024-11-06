use super::install::cmd_install;
use crate::ctx::CliContext;
use eyre::Result;

pub async fn cmd_update(cx: &CliContext, packages: Vec<String>, skip_missing: bool) -> Result<()> {
    cmd_install(cx, packages, skip_missing, true).await
}
