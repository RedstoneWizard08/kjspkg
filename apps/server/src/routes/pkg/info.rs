use crate::{
    auth::get_user_from_req, routes::users::pkg::clear_user_cache, state::AppState, Result,
};
use app_core::AppError;
use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use db::{
    get_full_package, get_package, package_authors, packages, Package, PackageAuthor, PackageData,
    PackageVisibility,
};
use diesel::{delete, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PartialPackage {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub readme: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub source: Option<String>,

    #[serde(default)]
    pub issues: Option<String>,

    #[serde(default)]
    pub wiki: Option<String>,

    #[serde(default)]
    pub visibility: Option<PackageVisibility>,

    #[serde(default)]
    pub license: Option<String>,

    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

/// Get Package
///
/// Get a package by its ID or slug.
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}",
    tag = "Packages",
    responses(
        (status = 200, description = "Information about the package", body = PackageData),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package ID or slug"),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_full_package(id, &mut conn).await?;

    if pkg.visibility == PackageVisibility::Private {
        match get_user_from_req(&jar, &headers, &mut conn).await {
            Ok(user) => {
                if !pkg.authors.iter().any(|v| v.github_id == user.github_id) && !user.admin {
                    return Err(AppError::NotFound);
                }
            }

            Err(_) => return Err(AppError::NotFound),
        }
    }

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&pkg)?))?)
}

/// Update Package
///
/// Update a package's information.
#[utoipa::path(
    patch,
    path = "/api/v1/packages/{id}",
    tag = "Packages",
    responses(
        (status = 200, description = "Package updated successfully!", body = PackageData),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    request_body(content = PartialPackage, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(data): Json<PartialPackage>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    let pkg = update(packages::table)
        .filter(packages::id.eq(pkg.id))
        .set((
            packages::name.eq(data.name.unwrap_or(pkg.name)),
            packages::readme.eq(data.readme.unwrap_or(pkg.readme)),
            packages::description.eq(data.description.unwrap_or(pkg.description)),
            packages::source.eq(data.source.map(|v| Some(v)).unwrap_or(pkg.source)),
            packages::issues.eq(data.issues.map(|v| Some(v)).unwrap_or(pkg.issues)),
            packages::wiki.eq(data.wiki.map(|v| Some(v)).unwrap_or(pkg.wiki)),
            packages::visibility.eq(data.visibility.unwrap_or(pkg.visibility)),
            packages::license.eq(data.license.map(|v| Some(v)).unwrap_or(pkg.license)),
            packages::tags.eq(data
                .tags
                .map(|v| v.into_iter().map(|v| Some(v)).collect::<Vec<_>>())
                .unwrap_or(pkg.tags)),
        ))
        .returning(Package::as_select())
        .get_result(&mut conn)
        .await?;

    tokio::spawn(clear_user_cache(user.id));
    state.search.update_package(pkg.id, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(
            &get_full_package(pkg.id.to_string(), &mut conn).await?,
        )?))?)
}

/// Delete Package
///
/// Delete a package
#[utoipa::path(
    delete,
    path = "/api/v1/packages/{id}",
    tag = "Packages",
    responses(
        (status = 200, description = "Package deleted successfully!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "Error: package might not exist, or another error occured!"),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(id, &mut conn).await?;

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(&mut conn)
        .await?;

    if authors.iter().find(|v| v.user_id == user.id).is_none() && !user.admin {
        return Ok(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::empty())?);
    }

    delete(packages::table)
        .filter(packages::id.eq(pkg.id))
        .execute(&mut conn)
        .await?;

    tokio::spawn(clear_user_cache(user.id));
    state.search.delete_package(pkg.id).await?;

    Ok(Response::builder().body(Body::new("Deleted package successfully!".to_string()))?)
}
