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
    pub kubejs: Vec<Option<String>>,
    pub loaders: Vec<Option<String>>,
    pub minecraft: Vec<Option<String>>,
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
    pub kubejs: Vec<Option<String>>,
    pub loaders: Vec<Option<String>>,
    pub minecraft: Vec<Option<String>>,
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

/// This is ONLY for api doc purposes. It isn't actually used internally.
/// This is serialized as form data, each key representing a field.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse)]
pub struct PackageVersionInit {
    /// The name of the package.
    pub name: String,

    /// The version number.
    pub version_nummber: String,

    /// An optional changelog.
    pub changelog: Option<String>,

    /// A list of KubeJS versions that this version works on.
    /// This should be a comma-separated list.
    pub kubejs: String,

    /// A list of loaders this version works on.
    pub loaders: String,

    /// A list of Minecraft versions this works on.
    pub minecraft: String,

    /// The file content.
    pub file: Vec<u8>,
}
