use std::io::{Cursor, Read};

use anyhow::{anyhow, Result};
use axum::body::Bytes;
use db::PackageManifest;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn verify_package(bytes: &Bytes) -> Result<()> {
    let mut data = GzDecoder::new(Cursor::new(bytes));
    let mut gunzip = Vec::new();

    data.read_to_end(&mut gunzip)?;

    let mut archive = Archive::new(Cursor::new(gunzip));

    for entry in archive.entries()? {
        if let Ok(mut entry) = entry {
            if entry.path()?.to_str().unwrap() == "kjspkg.json" {
                let mut data = String::new();

                entry.read_to_string(&mut data)?;

                serde_json::from_str::<PackageManifest>(&data)?; // Just checking :P

                return Ok(());
            }
        }
    }

    Err(anyhow!(
        "Invalid tar.gz archive or missing/invalid kjspkg.json!"
    ))
}
