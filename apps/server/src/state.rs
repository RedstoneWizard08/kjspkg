use crate::{routes::meta::vers::GameVersion, Result};
use app_config::AppConfig;
use db::DbPool;
use oauth2::basic::BasicClient;
use s3::Bucket;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub auth: BasicClient,
    pub bucket: Box<Bucket>,
    pub config: AppConfig,
    pub game_versions: Vec<GameVersion>,
}

impl AppState {
    pub fn new(pool: DbPool, config: &AppConfig) -> Result<Self> {
        Ok(Self {
            pool,
            auth: config.auth.github()?,
            bucket: config.storage.packages()?,
            config: config.clone(),
            game_versions: vec![],
        })
    }
}
