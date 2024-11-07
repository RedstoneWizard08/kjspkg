use clap::ValueEnum;
use eyre::Result;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env::current_dir,
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, ValueEnum, Serialize, Deserialize,
)]
pub enum ModLoader {
    Forge,
    Fabric,
    Quilt,
    NeoForge,
}

/// A manifest representing a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectManifest {
    /// The root directory for KubeJS. (Should be the `kubejs/` directory.)
    pub root: PathBuf,

    /// The Minecraft version this project is using.
    pub minecraft: String,

    /// The mod loader this project is using.
    pub loader: ModLoader,

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

/// A manifest for a package.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct PackageManifest {
    /// The package name.
    pub name: String,

    /// The package author.
    pub authors: Vec<String>,

    /// The package version.
    pub version: String,

    /// The package description.
    pub description: String,

    /// The KubeJS versions this package works on.
    pub kubejs: Vec<String>,

    /// The loaders this package works on.
    pub loaders: Vec<ModLoader>,

    /// The Minecraft versions this package works on.
    pub minecraft: Vec<String>,

    /// This package's dependencies.
    pub dependencies: Vec<String>,

    /// This package's incompatibilities.
    pub incompatibilities: Vec<String>,
}

impl ProjectManifest {
    pub fn new(root: impl Into<PathBuf>, minecraft: String, loader: ModLoader) -> Self {
        Self {
            root: root.into(),
            minecraft,
            loader,
            packages: HashMap::new(),
        }
    }

    pub fn save(&self) -> Result<()> {
        fs::write(
            self.root.parent().unwrap().join("project.json"),
            serde_json::to_string_pretty(self)?,
        )?;

        Ok(())
    }

    pub fn read(dir: Option<PathBuf>) -> Result<Self> {
        // This will already return an `Err` if the file doesn't exist, so we don't need to check manually.
        // I love Rust :D
        Ok(serde_json::from_str(&fs::read_to_string(
            dir.unwrap_or(current_dir()?).join("project.json"),
        )?)?)
    }
}

impl PackageManifest {
    pub fn save(&self, dir: Option<PathBuf>) -> Result<()> {
        fs::write(
            dir.unwrap_or(current_dir()?).join("kjspkg.json"),
            serde_json::to_string_pretty(self)?,
        )?;

        Ok(())
    }

    pub fn read(dir: Option<PathBuf>) -> Result<Self> {
        Ok(serde_json::from_str(&fs::read_to_string(
            dir.unwrap_or(current_dir()?).join("kjspkg.json"),
        )?)?)
    }
}

impl Into<String> for ModLoader {
    fn into(self) -> String {
        match self {
            ModLoader::Fabric => "Fabric",
            ModLoader::Forge => "Forge",
            ModLoader::NeoForge => "NeoForge",
            ModLoader::Quilt => "Quilt",
        }
        .into()
    }
}

impl Display for ModLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", <Self as Into<String>>::into(*self))
    }
}
