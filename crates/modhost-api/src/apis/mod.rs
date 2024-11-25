mod common;
mod pkg;
mod users;
mod ver;

pub use pkg::*;
pub use ver::*;

pub(crate) use common::*;

use eyre::Result;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

pub const DEFAULT_API_BASE: &str = "https://kjspkg-uwqx.shuttle.app/api/v1";

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,

    pub api_base: String,
    pub token: Option<String>,
}

impl ApiHelper for ApiClient {
    fn base(&self) -> &String {
        &self.api_base
    }
}

impl ApiClient {
    pub fn new(api_base: Option<String>, key: Option<String>) -> Result<Self> {
        if let Some(key) = key {
            let mut headers = HeaderMap::new();

            headers.insert(
                "Authorization",
                HeaderValue::from_str(&format!("Bearer {}", key))?,
            );

            Ok(ApiClient {
                api_base: api_base.unwrap_or(DEFAULT_API_BASE.into()),
                token: Some(key),

                client: ClientBuilder::new()
                    .default_headers(headers.clone())
                    .build()?,
            })
        } else {
            Ok(ApiClient {
                api_base: api_base.unwrap_or(DEFAULT_API_BASE.into()),
                token: key,
                client: ClientBuilder::new().build()?,
            })
        }
    }

    pub fn instance_url(&self) -> String {
        self.api_base
            .trim_end_matches('/')
            .trim_end_matches("/api/v1")
            .into()
    }

    pub fn package(&self, pkg: impl AsRef<str>) -> PackageApi {
        PackageApi {
            api_base: self.api_base.clone(),
            package: pkg.as_ref().into(),
            client: self.client.clone(),
        }
    }
}
