use crate::schema::{user_tokens, users};
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
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub github_id: i32,
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
#[diesel(table_name = users)]
#[diesel(check_for_backend(Pg))]
pub struct NewUser {
    pub username: String,
    pub github_id: i32,
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
    Identifiable,
    Queryable,
    Selectable,
    Associations,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = user_tokens)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct UserToken {
    pub id: i32,
    pub user_id: i32,
    pub value: String,
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
    Associations,
    Insertable,
    ToSchema,
    ToResponse,
)]
#[diesel(table_name = user_tokens)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct NewUserToken {
    pub user_id: i32,
    pub value: String,
}
