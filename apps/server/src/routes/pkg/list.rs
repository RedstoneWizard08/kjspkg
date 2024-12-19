use crate::{
    auth::get_user_from_req, routes::users::pkg::clear_user_cache, state::AppState, Result,
};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use db::{
    get_full_package, package_authors, packages, NewPackage, Package, PackageAuthor, PackageData,
};
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

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

    tokio::spawn(clear_user_cache(user.id));
    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder().body(Body::new(serde_json::to_string(
        &get_full_package(pkg.id.to_string(), &mut conn).await?,
    )?))?)
}
