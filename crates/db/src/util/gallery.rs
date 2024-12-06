use crate::{gallery_images, DbConn, GalleryImage, Result};
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::RunQueryDsl;

pub async fn get_gallery_image(id: impl AsRef<str>, conn: &mut DbConn) -> Result<GalleryImage> {
    let id = id.as_ref().parse::<i32>()?;

    Ok(gallery_images::table
        .filter(gallery_images::id.eq(id))
        .select(GalleryImage::as_select())
        .first(conn)
        .await?)
}
