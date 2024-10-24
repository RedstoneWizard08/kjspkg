use std::env;

use anyhow::Result;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};

use crate::db::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub auth: BasicClient,
    pub supabase_url: String,
    pub supabase_key: String,
    pub packages_bucket: String,
}

impl AppState {
    pub fn new(
        pool: DbPool,
        client_id: Option<String>,
        client_secret: Option<String>,
        supabase_url: Option<String>,
        supabase_key: Option<String>,
        packages_bucket: Option<String>,
    ) -> Result<Self> {
        let client_id = client_id
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("GITHUB_CLIENT_ID"))?;

        let client_secret = client_secret
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("GITHUB_CLIENT_SECRET"))?;

        let supabase_url = supabase_url
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("SUPABASE_URL"))?;

        let supabase_key = supabase_key
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("SUPABASE_KEY"))?;

        let packages_bucket = packages_bucket
            .map(|v| Ok(v))
            .unwrap_or_else(|| env::var("SUPABASE_PACKAGES_BUCKET"))?;

        Ok(Self {
            pool,
            auth: Self::client(client_id, client_secret)?,
            supabase_url,
            supabase_key,
            packages_bucket,
        })
    }

    fn client(client_id: impl AsRef<str>, client_secret: impl AsRef<str>) -> Result<BasicClient> {
        Ok(BasicClient::new(
            ClientId::new(client_id.as_ref().into()),
            Some(ClientSecret::new(client_secret.as_ref().into())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string(),
            )?),
        ))
    }
}
