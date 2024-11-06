use clap::Parser;
use eyre::Result;
use kjspkg::cli::Cli;

#[tokio::main]
pub async fn main() -> Result<()> {
    color_eyre::install()?;
    dotenvy::dotenv()?;
    Cli::parse().run().await?;

    Ok(())
}
