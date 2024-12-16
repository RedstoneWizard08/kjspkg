use crate::{auth::get_user_from_req, state::AppState, Result};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use db::{
    search_packages, search_packages_admin, search_packages_authed, SearchResults, SortDirection,
    SortMode,
};

pub const MAX_PER_PAGE: i64 = 100;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct SearchQuery {
    /// The query string.
    pub q: Option<String>,

    /// The current page. Defaults to 1.
    pub page: Option<i64>,

    /// How many items per page. Defaults to 25.
    pub per_page: Option<i64>,

    /// The sort mode. Defaults to None.
    pub sort: Option<SortMode>,

    /// The sort direction. Defaults to Descending.
    pub direction: Option<SortDirection>,
}

/// Search Packages
///
/// Search packages by a query string
#[utoipa::path(
    get,
    path = "/api/v1/packages/search",
    tag = "Packages",
    params(
        ("q" = Option<String>, Query, description = "The query string"),
        ("page" = Option<i64>, Query, description = "The current page (zero-based indexed) - defaults to 0"),
        ("per_page" = Option<i64>, Query, description = "How many items per page - defaults to 25"),
    ),
    responses(
        (status = 200, description = "Method returned ok", body = SearchResults),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn search_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(SearchQuery {
        q,
        page,
        per_page,
        sort,
        direction,
    }): Query<SearchQuery>,
) -> Result<Json<SearchResults>> {
    let mut conn = state.pool.get().await?;
    let page = page.unwrap_or(1).max(1);
    let per_page = per_page.unwrap_or(25).min(MAX_PER_PAGE).max(1);
    let sort = sort.unwrap_or_default();
    let dir = direction.unwrap_or_default();

    Ok(Json(
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if user.admin {
                    search_packages_admin(q, page, per_page, sort, dir, &mut conn).await?
                } else {
                    search_packages_authed(q, page, per_page, sort, dir, user, &mut conn).await?
                }
            }

            Err(_) => search_packages(q, page, per_page, sort, dir, &mut conn).await?,
        },
    ))
}
