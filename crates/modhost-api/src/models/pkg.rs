use super::User;
use chrono::NaiveDateTime;

/// A struct representing the fields required to create a package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NewPackage {
    /// This package's slug (URL ID).
    pub slug: String,

    /// The name of the package.
    pub name: String,

    /// A short description of the package.
    pub description: String,

    /// This package's README, a long-form description of the package.
    pub readme: String,

    /// A link to the source code repository of this package.
    pub source: Option<String>,

    /// A link to this package's issue tracker.
    pub issues: Option<String>,

    /// A link to this package's wiki.
    pub wiki: Option<String>,
}

/// A struct representing the fields that can be updated in a package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageUpdate {
    /// The name of the package.
    pub name: Option<String>,

    /// A short description of the package.
    pub description: Option<String>,

    /// This package's README, a long-form description of the package.
    pub readme: Option<String>,

    /// A link to the source code repository of this package.
    pub source: Option<String>,

    /// A link to this package's issue tracker.
    pub issues: Option<String>,

    /// A link to this package's wiki.
    pub wiki: Option<String>,
}

/// A struct representing a package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageWithData {
    /// The package's numerical ID.
    pub id: i32,

    /// This package's slug (URL ID).
    pub slug: String,

    /// The name of the package.
    pub name: String,

    /// A short description of the package.
    pub description: String,

    /// A list of this package's authors.
    pub authors: Vec<User>,

    /// This package's README, a long-form description of the package.
    pub readme: String,

    /// A link to the source code repository of this package.
    pub source: Option<String>,

    /// A link to this package's issue tracker.
    pub issues: Option<String>,

    /// A link to this package's wiki.
    pub wiki: Option<String>,

    /// The number of views this package has.
    pub views: i32,

    /// The number of downloads this package has.
    pub downloads: i32,

    /// The date/time when this package was created.
    pub created_at: NaiveDateTime,

    /// The date/time when this package was last updated.
    pub updated_at: NaiveDateTime,
}
