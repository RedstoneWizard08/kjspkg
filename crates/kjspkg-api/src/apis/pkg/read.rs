use super::{ApiHelper, PackageApi};
use crate::models::{PackageVersion, PackageWithData, User};
use eyre::Result;

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
}
