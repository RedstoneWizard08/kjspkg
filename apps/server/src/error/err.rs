use super::{AxumError, HasCode};
use axum::response::{IntoResponse, Response};
use diesel_async::pooled_connection::deadpool::{BuildError, PoolError};
use jsglue::config::GlueConfigBuilderError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Pool(#[from] PoolError),

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

    #[error("Missing required token header or cookie!")]
    MissingToken,

    #[error("Unknown user!")]
    UnknownUser,
}

impl HasCode for AppError {
    fn code(&self) -> u16 {
        match self {
            Self::Multipart(_) => 400,
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

impl Into<shuttle_runtime::Error> for AppError {
    fn into(self) -> shuttle_runtime::Error {
        anyhow::anyhow!(self).into()
    }
}
