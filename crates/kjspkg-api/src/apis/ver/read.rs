use super::{ApiHelper, VersionApi};
use crate::models::PackageVersion;
use eyre::Result;

impl VersionApi {
    pub async fn get(&self) -> Result<PackageVersion> {
        Ok(self
            .client
            .get(self.url(format!(
                "packages/{}/versions/{}",
                self.package, self.version
            ))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn download(&self) -> Result<Vec<u8>> {
        Ok(self
            .client
            .get(self.url(format!(
                "packages/{}/versions/{}/download",
                self.package, self.version
            ))?)
            .send()
            .await?
            .bytes()
            .await?
            .to_vec())
    }
}
