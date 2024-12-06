use std::num::ParseIntError;

use super::{AxumError, HasCode};
use axum::response::{IntoResponse, Response};
use diesel::r2d2::PoolError as SyncPoolError;
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};
use jsglue::config::GlueConfigBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Pool(#[from] PoolError),

    #[error(transparent)]
    SyncPool(#[from] SyncPoolError),

    #[error(transparent)]
    GitHub(#[from] octocrab::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Database(#[from] diesel::result::Error),

    #[error(transparent)]
    Axum(#[from] axum::Error),

    #[error(transparent)]
    AxumHttp(#[from] axum::http::Error),

    #[error(transparent)]
    Header(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    HeaderValue(#[from] axum::http::header::InvalidHeaderValue),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Yaml(#[from] serde_yaml::Error),

    #[error(transparent)]
    TomlSer(#[from] toml::ser::Error),

    #[error(transparent)]
    TomlDe(#[from] toml::de::Error),

    #[error(transparent)]
    Env(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenv(#[from] dotenvy::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    AddrParse(#[from] std::net::AddrParseError),

    #[error(transparent)]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error(transparent)]
    Join(#[from] tokio::task::JoinError),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Glue(#[from] GlueConfigBuilderError),

    #[error(transparent)]
    DbInit(#[from] BuildError),

    #[error(transparent)]
    SemVer(#[from] semver::Error),

    #[error(transparent)]
    Config(#[from] config::ConfigError),

    #[error(transparent)]
    S3(#[from] s3::error::S3Error),

    #[error(transparent)]
    S3Creds(#[from] s3::creds::error::CredentialsError),

    #[error(transparent)]
    Zip(#[from] zip::result::ZipError),

    #[error(transparent)]
    TempFile(#[from] tempfile::PersistError),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error("Missing required token header or cookie!")]
    MissingToken,

    #[error("Unknown user!")]
    UnknownUser,
}

impl HasCode for AppError {
    fn code(&self) -> u16 {
        match self {
            Self::Multipart(_) | Self::ParseInt(_) => 400,
            Self::MissingToken => 401,
            Self::UnknownUser => 404,
            _ => 500,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.as_response()
    }
}

pub trait FixError<T> {
    fn fix_err(self) -> Result<T, Response>;
}

impl<T, E: Into<AppError>> FixError<T> for Result<T, E> {
    fn fix_err(self) -> Result<T, Response> {
        self.map_err(|v| v.into().as_response())
    }
}
