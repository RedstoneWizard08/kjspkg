use std::sync::Arc;

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
use chrono::Utc;
use db::{
    get_full_package, get_full_package_sync, package_authors, packages, DbPool, NewPackage,
    Package, PackageAuthor, PackageData, SyncDbPool,
};
use diesel::{insert_into, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use parking_lot::Mutex;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const CACHE_EXPIRY_MS: i64 = 15 * 60 * 1000; // 15 minutes = 15m * 60s * 1000ms

lazy_static! {
    static ref PACKAGE_LIST_CACHE: Arc<Mutex<Option<(i64, Vec<PackageData>)>>> =
        Arc::new(Mutex::new(None));
}

pub async fn refresh_list_cache(pool: DbPool, sync_pool: SyncDbPool) {
    let data = packages::table
        .select(Package::as_select())
        .load(&mut pool.get().await.unwrap())
        .await
        .unwrap_or_default();

    let res = data
        .par_iter()
        .filter_map(move |v| {
            let mut local_conn = sync_pool.get().unwrap();
            get_full_package_sync(v.id.to_string(), &mut local_conn).ok()
        })
        .collect::<Vec<_>>();

    *PACKAGE_LIST_CACHE.lock() =
        Some((Utc::now().timestamp_millis() + CACHE_EXPIRY_MS, res.clone()));
}

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
    if let Some((expires, data)) = PACKAGE_LIST_CACHE.lock().clone() {
        let now = Utc::now().timestamp_millis();

        if expires > now {
            return Ok(Json(data));
        }
    }

    let mut conn = state.pool.get().await?;

    let data = packages::table
        .select(Package::as_select())
        .load(&mut conn)
        .await?;

    let local_pool = state.sync_pool.clone();

    let res = data
        .par_iter()
        .filter_map(move |v| {
            let mut local_conn = local_pool.get().unwrap();
            get_full_package_sync(v.id.to_string(), &mut local_conn).ok()
        })
        .collect::<Vec<_>>();

    *PACKAGE_LIST_CACHE.lock() =
        Some((Utc::now().timestamp_millis() + CACHE_EXPIRY_MS, res.clone()));

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

    tokio::spawn(refresh_list_cache(state.pool, state.sync_pool));
    clear_user_cache(user.id);

    Ok(Response::builder().body(Body::new(serde_json::to_string(
        &get_full_package(pkg.id.to_string(), &mut conn).await?,
    )?))?)
}
