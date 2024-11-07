use super::{ApiHelper, VersionApi};
use crate::models::PackageVersion;
use eyre::Result;
use tokio::runtime::Handle;

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

    pub fn get_sync(&self, rt: &Handle) -> Result<PackageVersion> {
        Ok(rt.block_on(
            rt.block_on(
                self.client
                    .get(self.url(format!(
                        "packages/{}/versions/{}",
                        self.package, self.version
                    ))?)
                    .send(),
            )?
            .json(),
        )?)
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

    pub fn download_sync(&self, rt: &Handle) -> Result<Vec<u8>> {
        Ok(rt
            .block_on(
                rt.block_on(
                    self.client
                        .get(self.url(format!(
                            "packages/{}/versions/{}/download",
                            self.package, self.version
                        ))?)
                        .send(),
                )?
                .bytes(),
            )?
            .to_vec())
    }
}
