use anyhow::Result;
use clap::Parser;
use kjspkg::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
