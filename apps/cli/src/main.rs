use anyhow::Result;
use clap::Parser;
use kjspkg::cli::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
