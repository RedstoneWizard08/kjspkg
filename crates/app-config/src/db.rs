#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: Option<String>,
    pub database: String,
}

impl PostgresConfig {
    pub fn user(&self) -> String {
        match &self.pass {
            Some(it) => format!("{}:{}", self.user, it),
            None => self.user.clone(),
        }
    }

    pub fn uri(&self) -> String {
        format!(
            "postgresql://{}@{}:{}/{}",
            self.user(),
            self.host,
            self.port,
            self.database
        )
    }
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 5432,
            user: "modhost".into(),
            pass: Some("changeme".into()),
            database: "modhost".into(),
        }
    }
}
