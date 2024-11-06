mod read;
mod write;

use crate::models::{NewPackage, PackageWithData};

use super::{ApiClient, ApiHelper, VersionApi};
use eyre::Result;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct PackageApi {
    pub(crate) api_base: String,
    pub(crate) package: String,
    pub(crate) client: Client,
}

impl ApiHelper for PackageApi {
    fn base(&self) -> &String {
        &self.api_base
    }
}

impl PackageApi {
    pub fn version(&self, ver: String) -> VersionApi {
        VersionApi {
            api_base: self.api_base.clone(),
            package: self.package.clone(),
            version: ver,
            client: self.client.clone(),
        }
    }
}

impl ApiClient {
    pub async fn packages(&self) -> Result<Vec<PackageWithData>> {
        Ok(self
            .client
            .get(self.url("packages")?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn search_packages(&self, query: String) -> Result<Vec<PackageWithData>> {
        Ok(self
            .client
            .get(self.url("packages/search")?)
            .query(&[("q", query)])
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn create_package(&self, data: NewPackage) -> Result<PackageWithData> {
        Ok(self
            .client
            .put(self.url("packages")?)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }
}
