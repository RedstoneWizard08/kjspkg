use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VersionInfo {
    pub id: String,

    #[serde(rename = "type")]
    pub kind: String,

    pub url: String,

    #[serde(rename = "releaseTime")]
    pub release_time: String,

    pub sha1: String,

    #[serde(rename = "complianceLevel")]
    pub compliance_level: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LatestVersions {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct VersionManifestV2 {
    pub latest: LatestVersions,
    pub versions: Vec<VersionInfo>,
}
