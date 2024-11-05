use anyhow::Result;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
pub enum ModLoader {
    Forge,
    Fabric,
    Quilt,
    NeoForge,
}

pub async fn cmd_init(
    minecraft: Option<String>,
    loader: Option<ModLoader>,
    force: bool,
) -> Result<()> {
    Ok(())
}
