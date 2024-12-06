use app_core::Result;
use s3::{creds::Credentials, Bucket, Region};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub s3_region: String,
    pub s3_endpoint: String,

    pub s3_access_key: String,
    pub s3_secret_key: String,

    pub packages_bucket: String,
    pub gallery_bucket: String,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            s3_region: String::new(),
            s3_endpoint: String::new(),
            s3_access_key: String::new(),
            s3_secret_key: String::new(),
            packages_bucket: "packages".into(),
            gallery_bucket: "gallery".into(),
        }
    }
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

    pub fn gallery(&self) -> Result<Box<Bucket>> {
        Ok(
            Bucket::new(&self.gallery_bucket, self.region(), self.credentials()?)?
                .with_path_style(),
        )
    }
}
