mod read;
mod write;

use super::ApiHelper;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct VersionApi {
    pub(crate) api_base: String,
    pub(crate) package: String,
    pub(crate) version: String,
    pub(crate) client: Client,
}

impl ApiHelper for VersionApi {
    fn base(&self) -> &String {
        &self.api_base
    }
}
