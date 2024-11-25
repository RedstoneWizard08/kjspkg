/// A struct representing a user.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct User {
    /// The user's ID.
    pub id: i32,

    /// The user's GitHub ID.
    pub github_id: i32,

    /// The user's username.
    pub username: String,
}
