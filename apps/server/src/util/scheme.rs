use axum::{
    extract::FromRequestParts,
    http::{self, request::Parts},
};
use axum_core::__composite_rejection as composite_rejection;
use axum_core::__define_rejection as define_rejection;
use url::Url;

define_rejection! {
    #[status = BAD_REQUEST]
    #[body = "No scheme found in request"]
    pub struct FailedToResolveScheme;
}

composite_rejection! {
    pub enum SchemeRejection {
        FailedToResolveScheme,
    }
}

#[derive(Debug, Clone)]
pub struct Scheme(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Scheme
where
    S: Send + Sync,
{
    type Rejection = SchemeRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        if let Some(referer) = parts
            .headers
            .get(http::header::REFERER)
            .and_then(|referer| referer.to_str().ok()?.parse::<Url>().ok())
        {
            return Ok(Scheme(referer.scheme().into()));
        }

        if let Some(scheme) = parts.uri.scheme() {
            return Ok(Scheme(scheme.to_string()));
        }

        Ok(Scheme("http".into()))
    }
}
