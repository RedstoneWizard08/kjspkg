use anyhow::Result;
use clap::Parser;

use crate::{init_logger, start_app};

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'H', long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(short = 'p', long, default_value_t = 4000)]
    pub port: u16,

    #[arg(short = 'U', long)]
    pub db_url: Option<String>,

    #[arg(short = 'I', long)]
    pub github_client_id: Option<String>,

    #[arg(short = 'S', long)]
    pub github_client_secret: Option<String>,

    #[arg(long)]
    pub supabase_url: Option<String>,

    #[arg(long)]
    pub supabase_key: Option<String>,

    #[arg(long)]
    pub packages_bucket: Option<String>,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        dotenvy::dotenv()?;
        init_logger();
        start_app(self).await?;

        Ok(())
    }
}
