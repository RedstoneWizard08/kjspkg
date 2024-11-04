use crate::Result;
use db::{create_connection, DbPool, SyncDbPool};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub sync_pool: SyncDbPool,
    pub auth: BasicClient,
    pub supabase_url: String,
    pub supabase_key: String,
    pub packages_bucket: String,
    pub db_url: Option<String>,
}

impl AppState {
    pub fn new(
        pool: DbPool,
        sync_pool: SyncDbPool,
        client_id: Option<String>,
        client_secret: Option<String>,
        supabase_url: Option<String>,
        supabase_key: Option<String>,
        packages_bucket: Option<String>,
        db_url: Option<String>,
    ) -> Result<Self> {
        let embedded_client_id = option_env!("GH_CLIENT_ID").map(|v| v.to_string());
        let embedded_client_secret = option_env!("GH_CLIENT_SECRET").map(|v| v.to_string());
        let embedded_supabase_url = option_env!("SUPABASE_URL").map(|v| v.to_string());
        let embedded_supabase_key = option_env!("SUPABASE_KEY").map(|v| v.to_string());

        let embedded_packages_bucket =
            option_env!("SUPABASE_PACKAGES_BUCKET").map(|v| v.to_string());

        let client_id = client_id.map(|v| Ok(v)).unwrap_or_else(|| {
            embedded_client_id
                .map(|v| Ok(v))
                .unwrap_or_else(|| env::var("GH_CLIENT_ID"))
        })?;

        let client_secret = client_secret.map(|v| Ok(v)).unwrap_or_else(|| {
            embedded_client_secret
                .map(|v| Ok(v))
                .unwrap_or_else(|| env::var("GH_CLIENT_SECRET"))
        })?;

        let supabase_url = supabase_url.map(|v| Ok(v)).unwrap_or_else(|| {
            embedded_supabase_url
                .map(|v| Ok(v))
                .unwrap_or_else(|| env::var("SUPABASE_URL"))
        })?;

        let supabase_key = supabase_key.map(|v| Ok(v)).unwrap_or_else(|| {
            embedded_supabase_key
                .map(|v| Ok(v))
                .unwrap_or_else(|| env::var("SUPABASE_KEY"))
        })?;

        let packages_bucket = packages_bucket.map(|v| Ok(v)).unwrap_or_else(|| {
            embedded_packages_bucket
                .map(|v| Ok(v))
                .unwrap_or_else(|| env::var("SUPABASE_PACKAGES_BUCKET"))
        })?;

        Ok(Self {
            pool,
            sync_pool,
            auth: Self::client(client_id, client_secret)?,
            supabase_url,
            supabase_key,
            packages_bucket,
            db_url,
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

    #[allow(unused)]
    pub async fn new_pool(&self) -> Result<DbPool> {
        create_connection(self.db_url.clone()).await
    }
}
