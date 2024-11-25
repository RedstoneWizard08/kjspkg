use app_core::Result;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfigs {
    pub github: AuthConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AuthConfig {
    pub client_id: String,
    pub client_secret: String,
}

impl AuthConfigs {
    pub fn github(&self) -> Result<BasicClient> {
        Ok(BasicClient::new(
            ClientId::new(self.github.client_id.clone()),
            Some(ClientSecret::new(self.github.client_secret.clone())),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string(),
            )?),
        ))
    }
}
