pub mod pkg;
pub mod scheme;

use anyhow::Result;
use octocrab::Octocrab;

pub fn create_github_client(token: impl AsRef<str>) -> Result<Octocrab> {
    Ok(Octocrab::builder().personal_token(token.as_ref()).build()?)
}
