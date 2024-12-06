use crate::{state::AppState, Result};
use db::{GalleryImage, PublicGalleryImage};

pub async fn get_image(id: impl AsRef<str>, state: &AppState) -> Result<Vec<u8>> {
    Ok(state
        .buckets
        .gallery
        .get_object(format!("/{}", id.as_ref()))
        .await?
        .to_vec())
}

pub async fn transform_gallery_image(img: GalleryImage) -> Result<PublicGalleryImage> {
    let url = format!("/api/v1/packages/s3/gallery/{}", img.s3_id);

    Ok(PublicGalleryImage {
        id: img.id,
        name: img.name,
        package: img.package,
        created_at: img.created_at,
        updated_at: img.updated_at,
        description: img.description,
        ordering: img.ordering,
        url,
    })
}

pub async fn transform_gallery(images: Vec<GalleryImage>) -> Result<Vec<PublicGalleryImage>> {
    let mut output = Vec::new();

    for img in images {
        output.push(transform_gallery_image(img).await?);
    }

    Ok(output)
}
