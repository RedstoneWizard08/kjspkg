use crate::{
    auth::get_user_from_req,
    db::pkg::get_full_package,
    schema::{package_authors, packages},
    state::AppState,
    NewPackage, Package, PackageAuthor, PackageData, Result,
};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

/// List Packages
///
/// List all available package
#[utoipa::path(
    get,
    path = "/api/v1/packages",
    tag = "Packages",
    responses(
        (status = 200, description = "Method returned ok", body = Vec<PackageData>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn list_handler(State(state): State<AppState>) -> Result<Json<Vec<PackageData>>> {
    let mut conn = state.pool.get().await?;

    let data = packages::table
        .select(Package::as_select())
        .load(&mut conn)
        .await?;

    let mut res = Vec::new();

    for pkg in data {
        res.push(get_full_package(pkg.id.to_string(), &mut conn).await?);
    }

    Ok(Json(res))
}

/// Create Package
///
/// Create a package
#[utoipa::path(
    put,
    path = "/api/v1/packages",
    tag = "Packages",
    responses(
        (status = 200, description = "Package created successfully!", body = PackageData),
        (status = 401, description = "Package already exists!"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    request_body(content = NewPackage, description = "Information about the package to create"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_handler(
    jar: CookieJar,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(body): Json<NewPackage>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;

    if let Some(_) = packages::table
        .filter(packages::slug.eq(body.slug.clone()))
        .select(Package::as_select())
        .first(&mut conn)
        .await
        .optional()?
    {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::new(
                "Package with that slug already exists!".to_string(),
            ))?);
    }

    let pkg = insert_into(packages::table)
        .values(&body)
        .returning(Package::as_returning())
        .get_result(&mut conn)
        .await?;

    insert_into(package_authors::table)
        .values(&PackageAuthor {
            package: pkg.id,
            user_id: user.id,
        })
        .execute(&mut conn)
        .await?;

    Ok(Response::builder().body(Body::new(serde_json::to_string(
        &get_full_package(pkg.id.to_string(), &mut conn).await?,
    )?))?)
}
