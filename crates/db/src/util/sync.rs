use crate::{
    package_authors, package_versions, packages, users, Package, PackageAuthor, PackageData,
    PackageVersion, Result, SyncDbConn, User,
};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn get_package_sync(id: impl AsRef<str>, conn: &mut SyncDbConn) -> Result<Package> {
    let id = id.as_ref();

    if let Ok(id) = id.parse::<i32>() {
        let pkg = packages::table
            .find(id)
            .select(Package::as_select())
            .first(conn)
            .optional()?;

        if let Some(pkg) = pkg {
            return Ok(pkg);
        }
    }

    Ok(packages::table
        .filter(packages::slug.eq(id))
        .select(Package::as_select())
        .first(conn)?)
}

pub fn get_full_package_sync(id: impl AsRef<str>, conn: &mut SyncDbConn) -> Result<PackageData> {
    let pkg = get_package_sync(id, conn)?;
    let mut pkg_authors = Vec::new();

    let authors = package_authors::table
        .filter(package_authors::package.eq(pkg.id))
        .select(PackageAuthor::as_select())
        .load(conn)?;

    for item in authors {
        pkg_authors.push(
            users::table
                .find(item.user_id)
                .select(User::as_select())
                .get_result(conn)?,
        );
    }

    let downloads = package_versions::table
        .filter(package_versions::package.eq(pkg.id))
        .select(PackageVersion::as_select())
        .load(conn)?
        .iter()
        .map(|v| v.downloads)
        .sum();

    Ok(PackageData {
        id: pkg.id,
        name: pkg.name,
        slug: pkg.slug,
        readme: pkg.readme,
        description: pkg.description,
        source: pkg.source,
        issues: pkg.issues,
        wiki: pkg.wiki,
        created_at: pkg.created_at,
        updated_at: pkg.updated_at,
        views: pkg.views,
        downloads,
        authors: pkg_authors,
    })
}
