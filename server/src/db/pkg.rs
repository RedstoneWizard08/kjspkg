use super::DbConn;
use crate::{
    schema::{package_authors, package_versions, packages, users},
    Package, PackageAuthor, PackageData, PackageVersion, User,
};
use anyhow::Result;
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

pub async fn get_package(id: impl AsRef<str>, conn: &mut DbConn) -> Result<Package> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let pkg = packages::table
            .find(id)
            .select(Package::as_select())
            .first(conn)
            .await
            .optional()?;

        if let Some(pkg) = pkg {
            return Ok(pkg);
        }
    }

    Ok(packages::table
        .filter(packages::slug.eq(id))
        .select(Package::as_select())
        .first(conn)
        .await?)
}

pub async fn get_full_package(id: impl AsRef<str>, conn: &mut DbConn) -> Result<PackageData> {
    let pkg = get_package(id, conn).await?;
    let mut pkg_authors = Vec::new();

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(conn)
        .await?;

    for item in authors {
        pkg_authors.push(
            users::table
                .find(item.user_id)
                .select(User::as_select())
                .get_result(conn)
                .await?,
        );
    }

    let downloads = package_versions::table
        .filter(package_versions::package.eq(pkg.id))
        .select(PackageVersion::as_select())
        .load(conn)
        .await?
        .iter()
        .map(|v| v.downloads)
        .sum();

    Ok(PackageData {
        id: pkg.id,
        name: pkg.name,
        slug: pkg.slug,
        readme: pkg.readme,
        description: pkg.description,
        supports_forge: pkg.supports_forge,
        supports_fabric: pkg.supports_fabric,
        supports_quilt: pkg.supports_quilt,
        supports_neoforge: pkg.supports_neoforge,
        created_at: pkg.created_at,
        updated_at: pkg.updated_at,
        views: pkg.views,
        downloads,
        authors: pkg_authors,
    })
}
