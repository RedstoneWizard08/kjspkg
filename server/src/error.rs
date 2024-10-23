use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type HttpResult<T, E = HttpError> = std::result::Result<T, E>;

pub struct HttpError(anyhow::Error);

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("An internal error occured: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for HttpError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
