use chrono::NaiveDateTime;

/// A struct representing the fields required to create a new package version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct NewPackageVersion {
    /// The display name of this version.
    pub name: String,

    /// This version's version number.
    pub version_number: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of KubeJS versions this version works on.
    /// Note that this is currently not used anywhere.
    pub kubejs: Vec<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<String>,

    /// A list of Minecraft versions this version works on.
    pub minecraft: Vec<String>,
}

/// A struct representing the fields that can be updated in a package version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageVersionUpdate {
    /// The display name of this version.
    pub name: Option<String>,

    /// This version's version number.
    pub version_number: Option<String>,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of KubeJS versions this version works on.
    /// Note that this is currently not used anywhere.
    pub kubejs: Option<Vec<String>>,

    /// A list of loaders this version works on.
    pub loaders: Option<Vec<String>>,

    /// A list of Minecraft versions this version works on.
    pub minecraft: Option<Vec<String>>,
}

/// A struct representing a package version.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageVersion {
    /// The version's numerical ID.
    pub id: i32,

    /// The ID of the package this version belongs to.
    pub package: i32,

    /// The display name of this version.
    pub name: String,

    /// This version's version number.
    pub version_number: String,

    /// An optional markdown-formatted changelog.
    pub changelog: Option<String>,

    /// A list of KubeJS versions this version works on.
    /// Note that this is currently not used anywhere.
    pub kubejs: Vec<String>,

    /// A list of loaders this version works on.
    pub loaders: Vec<String>,

    /// A list of Minecraft versions this version works on.
    pub minecraft: Vec<String>,

    /// The number of downloads this version has.
    pub downloads: i32,

    /// The date/time when this version was created.
    pub created_at: NaiveDateTime,

    /// The date/time when this version was last updated.
    pub updated_at: NaiveDateTime,
}
