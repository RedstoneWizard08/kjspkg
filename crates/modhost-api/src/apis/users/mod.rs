use super::{ApiClient, ApiHelper};
use crate::models::{PackageWithData, User};
use eyre::Result;

impl ApiClient {
    pub async fn current_user(&self) -> Result<User> {
        Ok(self
            .client
            .get(self.url("users/me")?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn get_user(&self, user: String) -> Result<User> {
        Ok(self
            .client
            .get(self.url(format!("users/{}", user))?)
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn search_users(&self, query: String) -> Result<Vec<User>> {
        Ok(self
            .client
            .get(self.url("users/search")?)
            .query(&[("q", query)])
            .send()
            .await?
            .json()
            .await?)
    }

    pub async fn user_packages(&self, user: String) -> Result<Vec<PackageWithData>> {
        Ok(self
            .client
            .get(self.url(format!("users/{}/packages", user))?)
            .send()
            .await?
            .json()
            .await?)
    }
}
