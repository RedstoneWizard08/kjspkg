use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};

/// A manifest representing a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProjectManifest {
    /// The root directory for KubeJS. (Should be the `kubejs/` directory.)
    pub root: PathBuf,

    /// A list of packages to information about them.
    pub packages: HashMap<String, PackageInfo>,
}

/// Information about an installed package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageInfo {
    /// The package's name.
    pub name: String,

    /// The package's version.
    pub version: String,

    /// The SHA-1 hash of the version file.
    pub hash: String,

    /// A list of files & folders this package contributed.
    pub files: Vec<String>,
}

impl ProjectManifest {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            ..Default::default()
        }
    }

    pub fn save(&self) -> Result<()> {
        fs::write("project.json", serde_json::to_string_pretty(self)?)?;

        Ok(())
    }
}
