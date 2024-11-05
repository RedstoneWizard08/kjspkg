use anyhow::Result;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum)]
pub enum ListOutputFormat {
    #[clap(aliases = ["md"])]
    Markdown,
    HTML,
    #[clap(aliases = ["txt"])]
    Text,
}

pub async fn cmd_list(format: ListOutputFormat) -> Result<()> {
    Ok(())
}
