use crate::{auth::get_user_from_req, state::AppState, Result};
use axum::{
    extract::{Query, State},
    http::HeaderMap,
    Json,
};
use axum_extra::extract::CookieJar;
use db::{packages, users, Package, PackageAuthor, PackageData, PackageVisibility, User};
use diesel::{
    BelongingToDsl, BoolExpressionMethods, GroupedBy, PgTextExpressionMethods, QueryDsl,
    SelectableHelper,
};
use diesel_async::RunQueryDsl;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct SearchQuery {
    q: String,
}

/// Search Packages
///
/// Search packages by a query string
#[utoipa::path(
    get,
    path = "/api/v1/packages/search",
    tag = "Packages",
    params(
        ("q" = String, Query, description = "The query string"),
    ),
    responses(
        (status = 200, description = "Method returned ok", body = Vec<PackageData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn search_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(SearchQuery { q }): Query<SearchQuery>,
) -> Result<Json<Vec<PackageData>>> {
    let mut conn = state.pool.get().await?;

    let pkgs = packages::table
        .filter(
            packages::name
                .ilike(format!("%{}%", q))
                .or(packages::slug.ilike(format!("%{}%", q))),
        )
        .select(Package::as_select())
        .load(&mut conn)
        .await?;

    let users: Vec<(PackageAuthor, User)> = PackageAuthor::belonging_to(&pkgs)
        .inner_join(users::table)
        .select((PackageAuthor::as_select(), User::as_select()))
        .load(&mut conn)
        .await
        .unwrap();

    let res = users
        .grouped_by(&pkgs)
        .into_iter()
        .zip(pkgs)
        .map(|(users, pkg)| pkg.with_authors(users.iter().map(|(_, user)| user.clone()).collect()))
        .collect::<Vec<_>>();

    match get_user_from_req(&jar, &headers, &mut conn).await {
        Ok(user) => Ok(Json(
            res.iter()
                .filter(|v| {
                    v.visibility == PackageVisibility::Public
                        || v.authors.iter().any(|v| v.github_id == user.github_id)
                        || user.admin
                })
                .cloned()
                .collect(),
        )),

        Err(_) => Ok(Json(
            res.iter()
                .filter(|v| v.visibility == PackageVisibility::Public)
                .cloned()
                .collect(),
        )),
    }
}
