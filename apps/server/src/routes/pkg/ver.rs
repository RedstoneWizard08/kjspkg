use crate::{auth::get_user_from_req, state::AppState, util::pkg::verify_package, Result};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{HeaderMap, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::CookieJar;
use db::{
    get_package, get_version, package_authors, package_versions, NewPackageVersion, PackageAuthor,
    PackageVersion, PackageVersionInit,
};
use diesel::{delete, insert_into, update, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use reqwest::Client;
use uuid::Uuid;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PartialPackageVersion {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(default)]
    pub version_number: Option<String>,

    #[serde(default)]
    pub changelog: Option<String>,

    #[serde(default)]
    pub kubejs: Option<Vec<String>>,

    #[serde(default)]
    pub loaders: Option<Vec<String>>,

    #[serde(default)]
    pub minecraft: Option<Vec<String>>,
}

/// List Package Versions
///
/// List available versions for a specific package.
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Found package versions!", body = Vec<PackageVersion>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package ID whose versions we are looking for."),
    ),
)]
#[debug_handler]
pub async fn list_handler(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_package(id, &mut conn).await?;

    let versions = package_versions::table
        .filter(package_versions::package.eq(pkg.id))
        .select(PackageVersion::as_select())
        .load(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&versions)?))?)
}

/// Get Package Version
///
/// Get information about a specific package version
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Found package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
)]
#[debug_handler]
pub async fn info_handler(
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Download Package Version
///
/// Download a specific package version
#[utoipa::path(
    get,
    path = "/api/v1/packages/{id}/versions/{version}/download",
    tag = "Versions",
    responses(
        (status = 307, description = "Redirecting to download"),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
)]
#[debug_handler]
pub async fn download_handler(
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

    let ver = update(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .set(package_versions::downloads.eq(ver.downloads + 1))
        .returning(PackageVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    let url = format!(
        "{}/storage/v1/object/{}/{}.tgz",
        state.supabase_url, state.packages_bucket, ver.file_id
    );

    Ok(Response::builder()
        .header("Location", url)
        .status(StatusCode::TEMPORARY_REDIRECT)
        .body(Body::empty())?)
}

/// Upload Package Version
///
/// Upload a new package version
#[utoipa::path(
    put,
    path = "/api/v1/packages/{id}/versions",
    tag = "Versions",
    responses(
        (status = 200, description = "Created package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
    ),
    request_body(content = PackageVersionInit, description = "The version data"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn create_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path(id): Path<String>,
    State(state): State<AppState>,
    mut data: Multipart,
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

    let mut name = None;
    let mut version_number = None;
    let mut changelog = None;
    let mut kubejs = None;
    let mut loaders = None;
    let mut minecraft = None;
    let mut file = None;

    while let Ok(Some(field)) = data.next_field().await {
        match field
            .name()
            .ok_or(anyhow!("Could not find a name for a field!"))?
        {
            "name" => name = Some(field.text().await?),
            "version_number" => version_number = Some(field.text().await?),
            "changelog" => changelog = Some(field.text().await?),
            "kubejs" => {
                kubejs = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "loaders" => {
                loaders = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "minecraft" => {
                minecraft = Some(
                    field
                        .text()
                        .await?
                        .split(",")
                        .map(|v| Some(v.to_string()))
                        .collect::<Vec<_>>(),
                )
            }
            "file" => file = Some(field.bytes().await?),
            _ => {}
        }
    }

    if name.is_none() {
        Err(anyhow!("Missing field: 'name'"))?;
    }

    if version_number.is_none() {
        Err(anyhow!("Missing field: 'version_number'"))?;
    }

    if kubejs.is_none() {
        Err(anyhow!("Missing field: 'kubejs'"))?;
    }

    if loaders.is_none() {
        Err(anyhow!("Missing field: 'loaders'"))?;
    }

    if minecraft.is_none() {
        Err(anyhow!("Missing field: 'minecraft'"))?;
    }

    if file.is_none() {
        Err(anyhow!("Missing field: 'file'"))?;
    }

    let name = name.unwrap();
    let version_number = version_number.unwrap();
    let kubejs = kubejs.unwrap();
    let loaders = loaders.unwrap();
    let minecraft = minecraft.unwrap();
    let file = file.unwrap();

    verify_package(&file)?;

    let file_id = Uuid::new_v4().to_string();
    let file_name = format!("{}.tgz", file_id);

    let url = format!(
        "{}/storage/v1/object/{}/{}",
        state.supabase_url, state.packages_bucket, file_name
    );

    Client::new()
        .post(url)
        .header("Authorization", format!("Bearer {}", state.supabase_key))
        .body(file)
        .send()
        .await?;

    let data = NewPackageVersion {
        package: pkg.id,
        name,
        version_number,
        file_id,
        changelog,
        kubejs,
        loaders,
        minecraft,
        downloads: 0,
    };

    let ver = insert_into(package_versions::table)
        .values(&data)
        .returning(PackageVersion::as_returning())
        .get_result(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Update Package Version
///
/// Update information about package version
#[utoipa::path(
    patch,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Updated package version!", body = PackageVersion),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    request_body(content = PartialPackageVersion, description = "The information to update"),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn update_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
    Json(data): Json<PartialPackageVersion>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

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

    let ver = update(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .set((
            package_versions::name.eq(data.name.unwrap_or(ver.name)),
            package_versions::version_number.eq(data.version_number.unwrap_or(ver.version_number)),
            package_versions::changelog
                .eq(data.changelog.map(|v| Some(v)).unwrap_or(ver.changelog)),
            package_versions::kubejs.eq(data
                .kubejs
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.kubejs)),
            package_versions::loaders.eq(data
                .loaders
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.loaders)),
            package_versions::minecraft.eq(data
                .minecraft
                .map(|v| v.iter().map(|v| Some(v.clone())).collect::<Vec<_>>())
                .unwrap_or(ver.minecraft)),
        ))
        .returning(PackageVersion::as_select())
        .get_result(&mut conn)
        .await?;

    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .body(Body::new(serde_json::to_string(&ver)?))?)
}

/// Delete Package Version
///
/// Delete a package version
#[utoipa::path(
    delete,
    path = "/api/v1/packages/{id}/versions/{version}",
    tag = "Versions",
    responses(
        (status = 200, description = "Deleted package version!", body = String),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
    params(
        ("id" = String, Path, description = "The package that this version is for."),
        ("version" = String, Path, description = "The version ID/name/number."),
    ),
    security(
        ("api_auth_token" = []),
    ),
)]
#[debug_handler]
pub async fn delete_handler(
    jar: CookieJar,
    headers: HeaderMap,
    Path((package, version)): Path<(String, String)>,
    State(state): State<AppState>,
) -> Result<Response> {
    let mut conn = state.pool.get().await?;
    let user = get_user_from_req(&jar, &headers, &mut conn).await?;
    let pkg = get_package(package, &mut conn).await?;
    let ver = get_version(pkg.id, version, &mut conn).await?;

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

    delete(package_versions::table)
        .filter(package_versions::id.eq(ver.id))
        .execute(&mut conn)
        .await?;

    Ok(Response::builder().body(Body::new(
        "Deleted package version successfully!".to_string(),
    ))?)
}
