use crate::{
    models::{pkg::Package, user::User},
    schema::package_authors,
};
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
    Queryable,
    Selectable,
    Insertable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = package_authors)]
#[diesel(belongs_to(Package, foreign_key = package))]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
#[diesel(primary_key(package, user_id))]
pub struct PackageAuthor {
    pub package: i32,
    pub user_id: i32,
}
