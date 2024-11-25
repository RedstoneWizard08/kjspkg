use app_core::Result;
use s3::{creds::Credentials, Bucket, Region};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StorageConfig {
    pub s3_region: String,
    pub s3_endpoint: String,

    pub s3_access_key: String,
    pub s3_secret_key: String,

    pub packages_bucket: String,
}

impl StorageConfig {
    pub fn credentials(&self) -> Result<Credentials> {
        Ok(Credentials::new(
            Some(&self.s3_access_key),
            Some(&self.s3_secret_key),
            None,
            None,
            None,
        )?)
    }

    pub fn region(&self) -> Region {
        Region::Custom {
            region: self.s3_region.clone(),
            endpoint: self.s3_endpoint.clone(),
        }
    }

    pub fn packages(&self) -> Result<Box<Bucket>> {
        Ok(
            Bucket::new(&self.packages_bucket, self.region(), self.credentials()?)?
                .with_path_style(),
        )
    }
}
