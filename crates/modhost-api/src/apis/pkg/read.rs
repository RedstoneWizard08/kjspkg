use super::{ApiHelper, PackageApi};
use crate::models::{PackageVersion, PackageWithData, User};
use eyre::Result;
use tokio::runtime::Handle;

impl PackageApi {
    pub async fn get(&self) -> Result<PackageWithData> {
        Ok(self
            .client
            .get(self.url(format!("packages/{}", self.package))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub fn get_sync(&self, rt: &Handle) -> Result<PackageWithData> {
        Ok(rt.block_on(
            rt.block_on(
                self.client
                    .get(self.url(format!("packages/{}", self.package))?)
                    .send(),
            )?
            .json(),
        )?)
    }

    pub async fn authors(&self) -> Result<Vec<User>> {
        Ok(self
            .client
            .get(self.url(format!("packages/{}/authors", self.package))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn versions(&self) -> Result<Vec<PackageVersion>> {
        Ok(self
            .client
            .get(self.url(format!("packages/{}/versions", self.package))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn latest_version(&self) -> Result<PackageVersion> {
        Ok(self
            .client
            .get(self.url(format!("packages/{}/versions/latest", self.package))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub fn latest_version_sync(&self, rt: &Handle) -> Result<PackageVersion> {
        Ok(rt.block_on(
            rt.block_on(
                self.client
                    .get(self.url(format!("packages/{}/versions/latest", self.package))?)
                    .send(),
            )?
            .json(),
        )?)
    }
}
