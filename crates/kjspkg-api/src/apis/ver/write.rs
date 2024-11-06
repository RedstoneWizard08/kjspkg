use super::{ApiHelper, VersionApi};
use crate::models::{PackageVersion, PackageVersionUpdate};
use eyre::Result;

impl VersionApi {
    pub async fn delete(&self) -> Result<()> {
        self.client
            .delete(self.url(format!(
                "packages/{}/versions/{}",
                self.package, self.version
            ))?)
            .send()
            .await?;

        Ok(())
    }

    pub async fn update(&self, data: PackageVersionUpdate) -> Result<PackageVersion> {
        Ok(self
            .client
            .patch(self.url(format!(
                "packages/{}/versions/{}",
                self.package, self.version
            ))?)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }
}
