#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeilisearchConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub key: String,
    pub pkg_index: String,
}

impl MeilisearchConfig {
    pub fn url(&self) -> String {
        format!("{}://{}:{}", self.protocol, self.host, self.port)
    }
}

impl Default for MeilisearchConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 7700,
            protocol: "http".into(),
            key: "CHANGE_ME".into(),
            pkg_index: "packages".into(),
        }
    }
}
