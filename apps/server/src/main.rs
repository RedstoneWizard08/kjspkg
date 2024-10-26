#[cfg(not(feature = "shuttle"))]
use anyhow::Result;

#[cfg(not(feature = "shuttle"))]
#[tokio::main]
pub async fn main() -> Result<()> {
    use clap::Parser;

    kjspkg::Cli::parse().run().await
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
pub async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    let _ = dotenvy::dotenv();

    if let Some(database_url) = secrets.get("DATABASE_URL") {
        std::env::set_var("DATABASE_URL", database_url);
    }

    if let Some(gh_client_id) = secrets.get("GH_CLIENT_ID") {
        std::env::set_var("GH_CLIENT_ID", gh_client_id);
    }

    if let Some(gh_client_secret) = secrets.get("GH_CLIENT_SECRET") {
        std::env::set_var("GH_CLIENT_SECRET", gh_client_secret);
    }

    if let Some(supabase_url) = secrets.get("SUPABASE_URL") {
        std::env::set_var("SUPABASE_URL", supabase_url);
    }

    if let Some(supabase_key) = secrets.get("SUPABASE_KEY") {
        std::env::set_var("SUPABASE_KEY", supabase_key);
    }

    if let Some(supabase_packages_bucket) = secrets.get("SUPABASE_PACKAGES_BUCKET") {
        std::env::set_var("SUPABASE_PACKAGES_BUCKET", supabase_packages_bucket);
    }

    kjspkg::create_shuttle_axum().await
}
