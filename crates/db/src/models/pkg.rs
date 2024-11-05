use super::User;
use crate::schema::packages;
use chrono::NaiveDateTime;
use diesel::pg::Pg;

/// A package.
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
    /// The package's ID.
    pub id: i32,

    /// The package's name.
    pub name: String,

    /// The package's URL slug.
    pub slug: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// The date the package was created.
    pub created_at: NaiveDateTime,

    /// The date the package was last updated.
    pub updated_at: NaiveDateTime,

    /// The package's view count.
    pub views: i32,

    /// The amount of downloads a package has.
    pub downloads: i32,

    /// An optional link to the package's source code.
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    pub wiki: Option<String>,
}

/// A model for creating a new package.
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
    /// The package's URL slug.
    pub slug: String,

    /// The package's name.
    pub name: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// An optional link to the package's source code.
    #[serde(default)]
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    #[serde(default)]
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    #[serde(default)]
    pub wiki: Option<String>,
}

/// A package with additional data.
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToSchema, ToResponse,
)]
pub struct PackageData {
    /// The package's ID.
    pub id: i32,

    /// The package's name.
    pub name: String,

    /// The package's URL slug.
    pub slug: String,

    /// The package's README.
    pub readme: String,

    /// A short description of the package.
    pub description: String,

    /// An optional link to the package's source code.
    pub source: Option<String>,

    /// An optional link to the package's issue tracker.
    pub issues: Option<String>,

    /// An optional link to the package's wiki.
    pub wiki: Option<String>,

    /// The date the package was created.
    pub created_at: NaiveDateTime,

    /// The date the package was last updated.
    pub updated_at: NaiveDateTime,

    /// The number of views the package has.
    pub views: i32,

    /// The number of downloads the package has.
    pub downloads: i32,

    /// This package's authors.
    pub authors: Vec<User>,
}

impl Package {
    pub fn with_authors(self, authors: Vec<User>) -> PackageData {
        PackageData {
            id: self.id,
            name: self.name,
            slug: self.slug,
            readme: self.readme,
            description: self.description,
            source: self.source,
            issues: self.issues,
            wiki: self.wiki,
            created_at: self.created_at,
            updated_at: self.updated_at,
            views: self.views,
            downloads: self.downloads,
            authors,
        }
    }
}
