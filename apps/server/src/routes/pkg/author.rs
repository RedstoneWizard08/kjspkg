use crate::{
    auth::get_user_from_req,
    db::{
        pkg::{get_full_package, get_package},
        user::get_user,
    },
    schema::{package_authors, users},
    state::AppState,
    PackageAuthor, PackageData, Result, User,
};
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
};
use axum_extra::extract::CookieJar;
use diesel::{insert_into, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

/// Get Package Authors
///
/// Get a package's authors by its ID or slug.
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/authors",
    tag = "Packages",
    responses(
        (status = 200, description = "A list of package authors", body = Vec<User>),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package ID or slug"),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    let mut res = Vec::new();

    for item in authors {
        res.push(
            users::table
                .find(item.user_id)
                .select(User::as_select())
                .get_result(&mut conn)
                .await?,
        );
    }

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&res)?))?)
}

/// Add Package Author
///
/// Add an author to a package.
#[utoipa::path(
    put,
    path = "/api/v1/packages/{id}/authors",
    tag = "Packages",
    responses(
        (status = 200, description = "Package updated successfully!", body = PackageData),
        (status = UNAUTHORIZED, description = "You do not have access to modify this package!"),
        (status = BAD_REQUEST, description = "The user is already a member of the project!"),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    request_body(content = String, description = "The ID/username of the author to add."),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn add_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    body: String,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let to_add = get_user(body, &mut conn).await?;

    if authors.iter().find(|v| v.user_id == to_add.id).is_some() {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Author is already a member of the project!".to_string(),
            ))?);
    }

    insert_into(package_authors::table)
        .values(&PackageAuthor {
            package: pkg.id,
            user_id: to_add.id,
        })
        .execute(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_package(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}
