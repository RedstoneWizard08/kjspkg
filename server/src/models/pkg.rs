use super::User;
use crate::schema::packages;
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = packages)]
#[diesel(check_for_backend(Pg))]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub readme: String,
    pub description: String,
    pub supports_forge: bool,
    pub supports_fabric: bool,
    pub supports_quilt: bool,
    pub supports_neoforge: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub views: i32,
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
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = packages)]
#[diesel(check_for_backend(Pg))]
pub struct NewPackage {
    pub slug: String,
    pub name: String,
    pub readme: String,
    pub description: String,

    #[serde(default)]
    pub supports_forge: bool,

    #[serde(default)]
    pub supports_fabric: bool,

    #[serde(default)]
    pub supports_quilt: bool,

    #[serde(default)]
    pub supports_neoforge: bool,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct PackageData {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub readme: String,
    pub description: String,
    pub supports_forge: bool,
    pub supports_fabric: bool,
    pub supports_quilt: bool,
    pub supports_neoforge: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub views: i32,
    pub downloads: i32,
    pub authors: Vec<User>,
}
