use crate::manifest::{PackageInfo, ProjectManifest};
use copy_dir::copy_dir;
use eyre::Result;
use flate2::read::GzDecoder;
use sha1::{Digest, Sha1};
use std::{
    fs,
    io::{Cursor, Read},
    path::PathBuf,
};
use tar::Archive;

pub const SCRIPT_DIRS: &[&str] = &["client_scripts", "server_scripts", "startup_scripts"];
pub const ASSET_DIRS: &[&str] = &["assets", "data"];

/// Install a package.
///
/// Args:
/// - kubejs_dir: The path to the `kubejs/` folder.
/// - data: The bytes of the package tarball to install from.
pub fn install_package(
    pkg: impl AsRef<str>,
    ver: impl AsRef<str>,
    data: impl AsRef<[u8]>,
    manifest: &mut ProjectManifest,
) -> Result<()> {
    let pkg = pkg.as_ref();
    let ver = ver.as_ref();
    let kubejs_dir = &manifest.root;
    let data = data.as_ref();

    let mut hasher = Sha1::new();

    hasher.update(&data);

    let hash = format!("{:x}", hasher.finalize());
    let mut data = GzDecoder::new(Cursor::new(data));
    let mut gunzip = Vec::new();

    data.read_to_end(&mut gunzip)?;

    let mut archive = Archive::new(Cursor::new(gunzip));
    let temp_root = kubejs_dir.join(".kjspkg").join("temp");
    let temp_dir = temp_root.join(&pkg).join(&hash);

    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }

    fs::create_dir_all(&temp_dir)?;
    archive.unpack(&temp_dir)?;

    let mut installed: Vec<PathBuf> = Vec::new();

    for dir in SCRIPT_DIRS {
        let path = temp_dir.join(dir);
        let pkg_dir = kubejs_dir.join(dir).join(".kjspkg").join(&pkg);
        let out = pkg_dir.join(&ver);

        if path.exists() {
            if !pkg_dir.exists() {
                fs::create_dir_all(pkg_dir)?;
            }

            if out.exists() {
                fs::remove_dir_all(&out)?;
            }

            copy_dir(path, &out)?;
            installed.push(out.strip_prefix(kubejs_dir)?.into());
        }
    }

    for dir in ASSET_DIRS {
        let path = temp_dir.join(dir);
        let out = kubejs_dir.join(dir);

        if path.exists() {
            let path = temp_dir.join(dir).canonicalize()?;

            if !out.exists() {
                fs::create_dir_all(&out)?;
            }

            for item in fs::read_dir(&path)? {
                if let Ok(item) = item {
                    let absolute = item.path().canonicalize()?;
                    let relative = absolute.strip_prefix(&path)?;
                    let real = out.join(relative);
                    let parent = real.parent();

                    if let Some(parent) = parent {
                        if !parent.exists() {
                            fs::create_dir_all(parent)?;
                        }
                    }

                    if item.metadata()?.is_dir() {
                        if real.exists() {
                            fs::remove_dir_all(&real)?;
                        }

                        copy_dir(item.path(), &real)?;
                    } else {
                        if real.exists() {
                            fs::remove_file(&real)?;
                        }

                        fs::copy(item.path(), &real)?;
                    }

                    installed.push(real.strip_prefix(kubejs_dir)?.into());
                }
            }
        }
    }

    fs::remove_dir_all(temp_dir)?;

    manifest.packages.insert(
        pkg.into(),
        PackageInfo {
            name: pkg.into(),
            hash,
            version: ver.into(),
            files: installed
                .iter()
                .map(|v| v.to_str().unwrap().into())
                .collect(),
        },
    );

    Ok(())
}
