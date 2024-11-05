use crate::{state::AppState, Result};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use db::{get_user, package_authors, packages, users, Package, PackageAuthor, PackageData, User};
use diesel::{BelongingToDsl, ExpressionMethods, GroupedBy, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;
use parking_lot::Mutex;
use std::{collections::HashMap, sync::Arc};

const CACHE_EXPIRY_MS: i64 = 15 * 60 * 1000; // 15 minutes = 15m * 60s * 1000ms

lazy_static! {
    static ref USER_PACKAGES_CACHE: Arc<Mutex<HashMap<i32, (i64, Vec<PackageData>)>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

pub fn clear_user_cache(id: i32) {
    USER_PACKAGES_CACHE.lock().remove(&id);
}

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
) -> Result<Json<Vec<PackageData>>> {
    let mut conn = state.pool.get().await?;
    let user = get_user(id, &mut conn).await?;

    if let Some((expires, data)) = USER_PACKAGES_CACHE.lock().get(&user.id) {
        let now = Utc::now().timestamp_millis();

        if *expires > now {
            return Ok(Json(data.clone()));
        }
    }

    let pkgs = package_authors::table
        .filter(package_authors::user_id.eq(user.id))
        .inner_join(packages::table)
        .select((PackageAuthor::as_select(), Package::as_select()))
        .load(&mut conn)
        .await?;

    let pkgs = pkgs.iter().map(|(_, pkg)| pkg).collect::<Vec<_>>();

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
        .map(|(users, pkg)| {
            pkg.clone()
                .with_authors(users.iter().map(|(_, user)| user.clone()).collect())
        })
        .collect::<Vec<_>>();

    USER_PACKAGES_CACHE.lock().insert(
        user.id,
        (Utc::now().timestamp_millis() + CACHE_EXPIRY_MS, res.clone()),
    );

    Ok(Json(res))
}
