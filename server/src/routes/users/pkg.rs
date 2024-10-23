use crate::{
    db::{pkg::get_full_package, user::get_user},
    schema::package_authors,
    state::AppState,
    HttpResult, PackageAuthor, PackageData,
};
use axum::{
    body::Body,
    extract::{Path, State},
    response::Response,
};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

/// Get User Packages
///
/// Get a user's packages.
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}/packages",
    tag = "Users",
    params(
        ("id" = i32, description = "The user ID."),
    ),
    responses(
        (status = 200, description = "Found packages!", body = Vec<PackageData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured! The user may not exist!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> HttpResult<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user(id, &mut conn).await?;

    let pkg_refs = package_authors::table
        .filter(package_authors::user_id.eq(user.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    let mut pkgs = Vec::new();

    for item in pkg_refs {
        pkgs.push(get_full_package(item.package.to_string(), &mut conn).await?);
    }

    Ok(Response::builder().body(Body::new(serde_json::to_string_pretty(&pkgs)?))?)
}
