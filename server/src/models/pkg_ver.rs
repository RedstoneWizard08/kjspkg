use crate::{
    models::pkg::Package,
    schema::{package_version_refs, package_versions},
};
use chrono::NaiveDateTime;
use diesel::pg::Pg;

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
#[diesel(table_name = package_versions)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct PackageVersion {
    pub id: i32,
    pub package: i32,
    pub name: String,
    pub version_number: String,
    #[serde(skip)]
    pub file_id: String,
    pub changelog: Option<String>,
    /// This is a comma-separated list.
    pub kubejs_versions: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub downloads: i32,
}

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
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = package_versions)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(check_for_backend(Pg))]
pub struct NewPackageVersion {
    pub package: i32,
    pub name: String,
    pub version_number: String,
    pub file_id: String,
    pub changelog: Option<String>,
    /// This is a comma-separated list.
    pub kubejs_versions: String,
    pub downloads: i32,
}

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
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = package_version_refs)]
#[diesel(belongs_to(PackageVersion, foreign_key = value))]
#[diesel(check_for_backend(Pg))]
pub struct PackageVersionRef {
    pub value: i32,
}
