use app_config::AppConfig;
use app_core::Result;
use meilisearch_sdk::client::Client;
use meilisearch_sdk::indexes::Index;

#[derive(Debug, Clone)]
pub struct MeilisearchService {
    pub(crate) client: Client,
    pub(crate) packages: String,
}

impl MeilisearchService {
    pub fn new(cfg: &AppConfig) -> Result<Self> {
        Ok(Self {
            client: Client::new(cfg.meilisearch.url(), Some(&cfg.meilisearch.key))?,
            packages: cfg.meilisearch.pkg_index.clone(),
        })
    }

    pub fn packages(&self) -> Index {
        self.client.index(&self.packages)
    }
}
