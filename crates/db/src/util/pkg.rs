use crate::{
    schema::{packages, users},
    DbConn, Package, PackageAuthor, PackageData, Result, User,
};
use diesel::{BelongingToDsl, ExpressionMethods, OptionalExtension, QueryDsl, SelectableHelper};
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

    let authors = PackageAuthor::belonging_to(&pkg)
        .inner_join(users::table)
        .select(User::as_select())
        .load(conn)
        .await?;

    Ok(pkg.with_authors(authors))
}
