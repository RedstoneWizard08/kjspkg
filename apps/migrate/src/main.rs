use anyhow::Result;
use migrate::run;

#[tokio::main]
pub async fn main() -> Result<()> {
    run().await
}
