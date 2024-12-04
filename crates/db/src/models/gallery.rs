use chrono::NaiveDateTime;
use diesel::pg::Pg;

use crate::{gallery_images, Package};

/// A gallery image.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = gallery_images)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct GalleryImage {
    /// The gallery image ID.
    pub id: i32,

    /// The package ID.
    pub package: i32,

    /// The display name of the version.
    pub name: String,

    /// This image's ID in S3.
    pub s3_id: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,

    /// The date this version was created.
    pub created_at: NaiveDateTime,

    /// The date this version was last updated.
    pub updated_at: NaiveDateTime,
}

/// A gallery image for insertion.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Associations,
    Insertable,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = gallery_images)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct NewGalleryImage {
    /// The package ID.
    pub package: i32,

    /// The display name of the version.
    pub name: String,

    /// This image's ID in S3.
    pub s3_id: String,

    /// An optional markdown-formatted description.
    pub description: Option<String>,

    /// The order of this image.
    pub ordering: i32,
}
