use anyhow::Result;
use astro::cli::Cli;
use clap::Parser;

#[tokio::main]
pub async fn main() -> Result<()> {
    Cli::parse().run().await
}
