use crate::ctx::CliContext;
use anyhow::Result;

pub async fn cmd_uninstall(
    cx: &CliContext,
    packages: Vec<String>,
    skip_missing: bool,
) -> Result<()> {
    Ok(())
}
