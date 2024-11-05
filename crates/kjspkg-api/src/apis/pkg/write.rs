use super::{ApiHelper, PackageApi};
use crate::models::{NewPackageVersion, PackageUpdate, PackageVersion, PackageWithData, User};
use anyhow::Result;
use reqwest::multipart::Form;
use std::io::Write;
use tempfile::NamedTempFile;

impl PackageApi {
    pub async fn delete(&self) -> Result<()> {
        self.client
            .delete(self.url(format!("packages/{}", self.package))?)
            .send()
            .await?;

        Ok(())
    }

    pub async fn update(&self, data: PackageUpdate) -> Result<PackageWithData> {
        Ok(self
            .client
            .patch(self.url(format!("packages/{}", self.package))?)
            .json(&data)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn add_author(&self, user: User) -> Result<PackageWithData> {
        Ok(self
            .client
            .put(self.url(format!("packages/{}/authors", self.package))?)
            .body(user.id.to_string())
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn add_author_from_id(&self, user_id: String) -> Result<PackageWithData> {
        Ok(self
            .client
            .put(self.url(format!("packages/{}/authors", self.package))?)
            .body(user_id)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn upload_version(
        &self,
        data: NewPackageVersion,
        file: Vec<u8>,
    ) -> Result<PackageVersion> {
        let mut temp_file = NamedTempFile::new()?;
        let mut form = Form::new();

        temp_file.write_all(file.as_slice())?;

        form = form.text("name", data.name);
        form = form.text("version_number", data.version_number);

        if let Some(changelog) = data.changelog {
            form = form.text("changelog", changelog);
        }

        form = form.text("kubejs", data.kubejs.join(","));
        form = form.text("loaders", data.loaders.join(","));
        form = form.text("minecraft", data.minecraft.join(","));
        form = form.file("file", temp_file.path()).await?;

        Ok(self
            .client
            .put(self.url(format!("packages/{}/versions", self.package))?)
            .multipart(form)
            .send()
            .await?
            .json()
            .await?)
    }
}
