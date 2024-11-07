use eyre::Result;
use itertools::Itertools;

use super::versions::VersionManifestV2;

pub const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

pub async fn get_minecraft_versions() -> Result<Vec<String>> {
    let data = reqwest::get(VERSION_MANIFEST_URL)
        .await?
        .json::<VersionManifestV2>()
        .await?;

    Ok(data.versions.iter().map(|v| v.id.clone()).collect_vec())
}
