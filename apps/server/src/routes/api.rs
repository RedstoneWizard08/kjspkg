use crate::{api::ApiDocs, state::AppState, HttpResult};
use axum::{extract::Query, routing::get, Router};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable as ServableRedoc};
use utoipa_scalar::{Scalar, Servable as ServableScalar};
use utoipa_swagger_ui::{Config, SwaggerUi};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct JsonQueryParams {
    pub pretty: Option<bool>,
}

/// OpenAPI YAML
///
/// Get the YAML version of the OpenAPI definition.
#[utoipa::path(
    get,
    path = "/api/v1/openapi/yaml",
    tag = "Misc",
    responses(
        (status = 200, description = "The OpenAPI spec.", body = String),
    ),
)]
#[debug_handler]
pub async fn yaml_api() -> HttpResult<String> {
    Ok(ApiDocs::openapi().to_yaml()?)
}

/// OpenAPI JSON
///
/// Get the JSON version of the OpenAPI definition.
#[utoipa::path(
    get,
    path = "/api/v1/openapi/json",
    tag = "Misc",
    params(
        ("pretty" = bool, Query, description = "Should the JSON be pretty-printed?"),
    ),
    responses(
        (status = 200, description = "The OpenAPI spec.", body = String),
    ),
)]
#[debug_handler]
pub async fn json_api(
    Query(JsonQueryParams { pretty }): Query<JsonQueryParams>,
) -> HttpResult<String> {
    if pretty.unwrap_or(false) {
        Ok(ApiDocs::openapi().to_pretty_json()?)
    } else {
        Ok(ApiDocs::openapi().to_json()?)
    }
}

pub fn register(router: Router<AppState>) -> Router<AppState> {
    router
        .merge(
            SwaggerUi::new("/api/v1/docs/swagger")
                .config(Config::default().try_it_out_enabled(true))
                .url("/api/v1/docs/_swagger", ApiDocs::openapi()),
        )
        .merge(
            Redoc::with_url("/api/v1/docs/redoc", ApiDocs::openapi())
                .custom_html(include_str!("../redoc.html")),
        )
        .merge(Scalar::with_url("/api/v1/docs/scalar", ApiDocs::openapi()))
        .merge(RapiDoc::new("/api/v1/openapi/json").path("/api/v1/docs/rapidoc"))
        .route("/api/v1/openapi/yaml", get(yaml_api))
        .route("/api/v1/openapi/json", get(json_api))
}
