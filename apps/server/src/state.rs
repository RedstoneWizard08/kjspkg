use crate::{
    routes::meta::{loaders::ModLoader, vers::GameVersion},
    Result,
};
use app_config::AppConfig;
use axum::body::Bytes;
use db::DbPool;
use oauth2::basic::BasicClient;
use s3::Bucket;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub auth: BasicClient,
    pub bucket: Box<Bucket>,
    pub config: AppConfig,
    pub loaders: Vec<ModLoader>,
    pub game_versions: Vec<GameVersion>,
    pub verifier: Arc<Box<dyn Fn(Bytes) -> bool + Send + Sync>>,
}

impl AppState {
    pub fn new(
        pool: DbPool,
        config: &AppConfig,
        verifier: Box<dyn Fn(Bytes) -> bool + Send + Sync>,
    ) -> Result<Self> {
        Ok(Self {
            pool,
            auth: config.auth.github()?,
            bucket: config.storage.packages()?,
            config: config.clone(),
            loaders: vec![],
            game_versions: vec![],
            verifier: Arc::new(verifier),
        })
    }
}
