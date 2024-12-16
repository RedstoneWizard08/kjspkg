use crate::{
    package_authors,
    packages::{self, BoxedQuery},
    users, DbConn, Package, PackageData, PackageVisibility, User,
};
use app_core::Result;
use diesel::{
    dsl::count, pg::Pg, BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods,
    QueryDsl, SelectableHelper,
};
use diesel_async::RunQueryDsl;
use itertools::Itertools;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct Pagination {
    pub page: i64,
    pub per_page: i64,
    pub results: i64,
    pub total: i64,
    pub pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct SearchResults {
    pub pagination: Pagination,
    pub results: Vec<PackageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse, Default)]
pub enum SortMode {
    #[default]
    #[serde(rename = "none")]
    None,

    #[serde(rename = "name")]
    DisplayName,

    #[serde(rename = "downloads")]
    Downloads,

    #[serde(rename = "published")]
    Published,

    #[serde(rename = "updated")]
    LastUpdated,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse, Default)]
pub enum SortDirection {
    #[default]
    #[serde(rename = "asc")]
    Ascending,

    #[serde(rename = "desc")]
    Descending,
}

pub fn generate_query(
    query: BoxedQuery<'static, Pg>,
    mode: SortMode,
    dir: SortDirection,
) -> BoxedQuery<'static, Pg> {
    match mode {
        SortMode::DisplayName => match dir {
            SortDirection::Ascending => query.order_by(packages::name.asc()),
            SortDirection::Descending => query.order_by(packages::name.desc()),
        },

        SortMode::Downloads => match dir {
            SortDirection::Ascending => query.order_by(packages::downloads.asc()),
            SortDirection::Descending => query.order_by(packages::downloads.desc()),
        },

        SortMode::Published => match dir {
            SortDirection::Ascending => query.order_by(packages::created_at.asc()),
            SortDirection::Descending => query.order_by(packages::created_at.desc()),
        },

        SortMode::LastUpdated => match dir {
            SortDirection::Ascending => query.order_by(packages::updated_at.asc()),
            SortDirection::Descending => query.order_by(packages::updated_at.desc()),
        },

        _ => query,
    }
}

pub async fn search_packages(
    query: Option<String>,
    page: i64,
    per_page: i64,
    mode: SortMode,
    dir: SortDirection,
    conn: &mut DbConn,
) -> Result<SearchResults> {
    match query {
        Some(query) => {
            let count = packages::table
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query)))
                        .and(packages::visibility.eq(PackageVisibility::Public)),
                )
                .count()
                .get_result::<i64>(conn)
                .await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query)))
                        .and(packages::visibility.eq(PackageVisibility::Public)),
                )
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }

        None => {
            let count = packages::table
                .filter(packages::visibility.eq(PackageVisibility::Public))
                .count()
                .get_result::<i64>(conn)
                .await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .filter(packages::visibility.eq(PackageVisibility::Public))
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }
    }
}

pub async fn search_packages_authed(
    query: Option<String>,
    page: i64,
    per_page: i64,
    mode: SortMode,
    dir: SortDirection,
    user: User,
    conn: &mut DbConn,
) -> Result<SearchResults> {
    match query {
        Some(query) => {
            let count = packages::table
                .inner_join(package_authors::table.inner_join(users::table))
                .select(count(packages::id))
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query)))
                        .and(
                            packages::visibility
                                .eq(PackageVisibility::Public)
                                .or(users::id.eq(user.id)),
                        ),
                )
                .first::<i64>(conn)
                .await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query)))
                        .and(
                            packages::visibility
                                .eq(PackageVisibility::Public)
                                .or(users::id.eq(user.id)),
                        ),
                )
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }

        None => {
            let count = packages::table
                .inner_join(package_authors::table.inner_join(users::table))
                .select(count(packages::id))
                .filter(
                    packages::visibility
                        .eq(PackageVisibility::Public)
                        .or(users::id.eq(user.id)),
                )
                .first::<i64>(conn)
                .await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .filter(
                    packages::visibility
                        .eq(PackageVisibility::Public)
                        .or(users::id.eq(user.id)),
                )
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }
    }
}

pub async fn search_packages_admin(
    query: Option<String>,
    page: i64,
    per_page: i64,
    mode: SortMode,
    dir: SortDirection,
    conn: &mut DbConn,
) -> Result<SearchResults> {
    match query {
        Some(query) => {
            let count = packages::table
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query))),
                )
                .count()
                .get_result::<i64>(conn)
                .await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .filter(
                    packages::name
                        .ilike(format!("%{}%", query))
                        .or(packages::slug.ilike(format!("%{}%", query))),
                )
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }

        None => {
            let count = packages::table.count().get_result::<i64>(conn).await?;

            let results = generate_query(packages::table.into_boxed(), mode, dir)
                .inner_join(package_authors::table.inner_join(users::table))
                .select((Package::as_select(), User::as_select()))
                .limit(if page == 1 { per_page + 1 } else { per_page })
                .offset((page - 1) * per_page)
                .load::<(Package, User)>(conn)
                .await?
                .into_iter()
                .into_group_map()
                .into_iter()
                .map(|v| v.0.with_authors(v.1))
                .collect_vec();

            Ok(SearchResults {
                pagination: Pagination {
                    page,
                    results: results.len() as i64,
                    per_page,
                    total: count,
                    pages: (count as f64 / per_page as f64).ceil() as i64,
                },
                results,
            })
        }
    }
}
