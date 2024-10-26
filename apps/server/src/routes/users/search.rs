use crate::{db::user::search_users, state::AppState, HttpResult, User};
use axum::{
    body::Body,
    extract::{Query, State},
    response::Response,
};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct SearchQuery {
    q: String,
}

/// Search Users
///
/// Search for users by a case-insensitive string in their username.
#[utoipa::path(
    get,
    path = "/api/v1/users/search",
    tag = "Users",
    params(
        ("q" = String, Query, description = "The string to search for in the username."),
    ),
    responses(
        (status = 200, description = "Finished successfully!", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! The user may not exist!"),
    ),
)]
#[debug_handler]
pub async fn search_handler(
    State(state): State<AppState>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> HttpResult<Response> {
    Ok(
        Response::builder().body(Body::new(serde_json::to_string_pretty(
            &search_users(q, &mut state.pool.get().await?).await?,
        )?))?,
    )
}
