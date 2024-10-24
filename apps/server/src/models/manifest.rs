#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, ToSchema, ToResponse, Serialize, Deserialize,
)]
pub struct PackageManifest {
    /// The package name
    pub name: String,

    /// The package author
    pub authors: Vec<String>,

    /// The package version
    pub version: String,

    /// The package description
    pub description: String,

    /// The KubeJS versions this package works on
    pub kubejs: Vec<String>,

    /// The loaders this package works on
    pub loaders: Vec<String>,

    /// This package's dependencies
    pub dependencies: Vec<String>,

    /// This package's incompatibilities
    pub incompatibilities: Vec<String>,
}
