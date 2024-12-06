pub mod gallery;
pub mod sanitize;
pub mod scheme;

use octocrab::Octocrab;

pub fn create_github_client(token: impl AsRef<str>) -> octocrab::Result<Octocrab> {
    Ok(Octocrab::builder().personal_token(token.as_ref()).build()?)
}
