use anyhow::Result;
use db::{
    get_full_package, get_package, get_user, package_authors, package_versions, packages, DbConn,
    NewPackage, NewPackageVersion, Package, PackageAuthor, PackageManifest, PackageVersion,
};
use diesel::{insert_into, SelectableHelper};
use diesel_async::RunQueryDsl;
use flate2::{write::GzEncoder, Compression};
use git2::{build::RepoBuilder, FetchOptions, Repository};
use glob::{glob_with, MatchOptions};
use reqwest::Client;
use serde_json::Value;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};
use uuid::Uuid;

use crate::util::MaybeAppendDirAll;

#[derive(Debug, Clone, Copy)]
pub struct LegacyPackage {
    pub name: &'static str,
    pub owner: &'static str,
    pub repo: &'static str,
    pub author: &'static str,

    /// Defaults to "main"
    pub branch: Option<&'static str>,

    /// Defaults to "."
    pub path: Option<&'static str>,
}

impl LegacyPackage {
    pub fn branch(&self) -> &'static str {
        self.branch.unwrap_or("main")
    }

    pub fn path_str(&self) -> &'static str {
        self.path.unwrap_or(".")
    }

    pub fn path(&self) -> PathBuf {
        PathBuf::from(self.path_str())
    }

    pub fn clone_path(&self) -> PathBuf {
        let env = env::var("HOME").expect("Missing $HOME!");

        PathBuf::from(env)
            .join(".kjspkg-migrator")
            .join("repos")
            .join(self.owner)
            .join(self.repo)
            .join(self.branch())
    }

    pub fn clone(&self) -> Result<Repository> {
        if self.clone_path().exists() {
            fs::remove_dir_all(self.clone_path())?;
        }

        let mut opts = FetchOptions::new();

        opts.depth(1); // Builder pattern doesn't work here unfortunately due to it returning a `&mut Self`.

        Ok(RepoBuilder::new()
            .branch(self.branch())
            .fetch_options(opts)
            .clone(
                &format!("https://github.com/{}/{}", self.owner, self.repo),
                &self.clone_path(),
            )?)
    }

    fn readme_path(&self, repo: &PathBuf) -> Option<PathBuf> {
        glob_with(
            &format!("{}/readme.*", repo.to_str()?),
            MatchOptions {
                case_sensitive: false,
                ..Default::default()
            },
        )
        .ok()?
        .next()?
        .map(|v| v.to_path_buf())
        .ok()
    }

    fn kjspkg_path(&self) -> PathBuf {
        self.clone_path().join(self.path()).join(".kjspkg")
    }

    pub fn desc(&self) -> Option<String> {
        let path = self.kjspkg_path();

        if path.exists() {
            let data = fs::read_to_string(path).ok()?;
            let data = serde_json::from_str::<Value>(&data).ok()?;

            data.get("description")
                .map(|v| v.as_str().map(|v| v.to_string()))
                .flatten()
        } else {
            None
        }
    }

    pub fn loaders(&self) -> Option<Vec<String>> {
        let path = self.kjspkg_path();

        if path.exists() {
            let data = fs::read_to_string(path).ok()?;
            let data = serde_json::from_str::<Value>(&data).ok()?;

            data.get("modloaders")
                .map(|v| {
                    v.as_array().map(|v| {
                        v.iter()
                            .filter_map(|v| v.as_str().map(|v| v.to_string()))
                            .collect::<Vec<_>>()
                    })
                })
                .flatten()
        } else {
            None
        }
    }

    pub fn pkg_path(&self) -> PathBuf {
        self.clone_path().join(self.path())
    }

    pub fn readme(&self) -> Option<String> {
        self.readme_path(&self.clone_path().join(self.path()))
            .map(|v| fs::read_to_string(v).ok())
            .flatten()
    }

    pub fn manifest(&self) -> PackageManifest {
        PackageManifest {
            name: self.name.into(),
            authors: vec![self.author.into()],
            version: "0.0.0".into(),
            description: self.desc().unwrap_or_default(),
            kubejs: vec![], // TODO
            loaders: self.loaders().unwrap_or_default(),
            dependencies: vec![],      // TODO
            incompatibilities: vec![], // TODO
        }
    }

    pub fn create_manifest(&self) -> Result<()> {
        fs::write(
            self.pkg_path().join("kjspkg.json"),
            serde_json::to_string_pretty(&self.manifest())?,
        )?;

        Ok(())
    }

    pub fn create_archive(&self) -> Result<PathBuf> {
        let parent = PathBuf::from("packages");
        let path = PathBuf::from(format!("packages/{}_v0.0.0.tgz", self.name));

        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }

        let file = File::create(&path)?;
        let enc = GzEncoder::new(file, Compression::default());
        let mut tar = tar::Builder::new(enc);

        tar.maybe_append_dir_all("client_scripts", self.pkg_path().join("client_scripts"))?;
        tar.maybe_append_dir_all("setup_scripts", self.pkg_path().join("setup_scripts"))?;
        tar.maybe_append_dir_all("server_scripts", self.pkg_path().join("server_scripts"))?;
        tar.maybe_append_dir_all("public", self.pkg_path().join("public"))?;
        tar.maybe_append_dir_all("data", self.pkg_path().join("data"))?;
        tar.maybe_append_dir_all("assets", self.pkg_path().join("assets"))?;
        tar.maybe_append_dir_all("src", self.pkg_path().join("src"))?;
        // TODO: Add custom directories in kjspkg.json

        tar.maybe_append_named(self.pkg_path().join("README.md"), "README.md")?;
        tar.maybe_append_named(self.pkg_path().join("README.txt"), "README.txt")?;
        tar.maybe_append_named(self.pkg_path().join("README"), "README")?;
        tar.maybe_append_named(self.pkg_path().join("LICENSE"), "LICENSE")?;

        // TODO: Maybe we should just use walkdir and ignore .gitignore and .git?

        tar.maybe_append_named(self.pkg_path().join("kjspkg.json"), "kjspkg.json")?;

        Ok(path)
    }

    pub async fn create(&self, conn: &mut DbConn) -> Result<()> {
        info!("[Package] >>> Package Start: {}@v0.0.0", self.name);

        info!(
            "[Package] Cloning source from https://github.com/{}/{} on branch {}",
            self.owner,
            self.repo,
            self.branch()
        );

        self.clone()?;

        info!("[Package] Clone success!");
        info!("[Package] Generating tgz archive...");

        let path = self.create_archive()?;

        info!("[Package] Generation success!");

        if !self.exists(conn).await {
            info!("[Package] Nonexistent in database, adding...");

            let user = get_user(self.author, conn).await?;
            let data = self.data();

            let pkg = insert_into(packages::table)
                .values(&data)
                .returning(Package::as_returning())
                .get_result(conn)
                .await?;

            insert_into(package_authors::table)
                .values(&PackageAuthor {
                    package: pkg.id,
                    user_id: user.id,
                })
                .execute(conn)
                .await?;

            info!("[Package] Created! ID: {}", pkg.id);
        }

        info!("[Package] Fetching package...");

        let pkg = get_package(self.name, conn).await?;

        info!("[Package] Uploading version file...");

        let supabase_url = env::var("SUPABASE_URL")?;
        let supabase_key = env::var("SUPABASE_KEY")?;
        let packages_bucket = env::var("SUPABASE_PACKAGES_BUCKET")?;

        let file_id = Uuid::new_v4().to_string();
        let file_name = format!("{}.tgz", file_id);

        let url = format!(
            "{}/storage/v1/object/{}/{}",
            supabase_url, packages_bucket, file_name
        );

        Client::new()
            .post(url)
            .header("Authorization", format!("Bearer {}", supabase_key))
            .body(fs::read(path)?)
            .send()
            .await?;

        info!("[Package] Done! Creating version...");

        let ver = insert_into(package_versions::table)
            .values(&self.version(pkg.id, file_id))
            .returning(PackageVersion::as_returning())
            .get_result(conn)
            .await?;

        info!("[Package] Done! ID: {}", ver.id);
        info!("[Package] >>> Package End: {}@v0.0.0", self.name);

        Ok(())
    }

    pub fn data(&self) -> NewPackage {
        NewPackage {
            slug: self.name.into(),
            name: self.name.into(),
            readme: self.readme().unwrap_or_default(),
            description: self.desc().unwrap_or_default(),
            source: Some(format!("https://github.com/{}/{}", self.owner, self.repo)),
            issues: None,
            wiki: None,
        }
    }

    pub fn version(&self, id: i32, file: String) -> NewPackageVersion {
        NewPackageVersion {
            package: id,
            name: "Initial Version".into(),
            version_number: "0.0.0".into(),
            file_id: file,
            changelog: Some("The version migrated from the old KJSPKG.".into()),
            kubejs: vec![],
            loaders: vec![],
            minecraft: vec![],
            downloads: 0,
        }
    }

    pub async fn exists(&self, conn: &mut DbConn) -> bool {
        get_package(self.name, conn).await.is_ok()
    }

    pub async fn is_owned(&self, conn: &mut DbConn) -> bool {
        if let Ok(full) = get_full_package(self.name, conn).await {
            full.authors.iter().any(|v| v.username == self.author)
        } else {
            false
        }
    }
}

#[macro_export]
macro_rules! package {
    ($name: expr; = $owner: expr; $repo: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: None,
            path: None,
        }
    };

    ($name: expr; = $owner: expr; $repo: expr; @ $branch: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: Some($branch),
            path: None,
        }
    };

    ($name: expr; = $owner: expr; $repo: expr; $path: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: None,
            path: Some($path),
        }
    };

    ($name: expr; = $owner: expr; $repo: expr; @ $branch: expr; $path: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: Some($branch),
            path: Some($path),
        }
    };

    ($owner: expr; $repo: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: None,
            path: None,
        }
    };

    ($owner: expr; $repo: expr; @ $branch: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: Some($branch),
            path: None,
        }
    };

    ($owner: expr; $repo: expr; $path: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: None,
            path: Some($path),
        }
    };

    ($owner: expr; $repo: expr; @ $branch: expr; $path: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $owner,
            branch: Some($branch),
            path: Some($path),
        }
    };

    ($real: expr; + $name: expr; = $owner: expr; $repo: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: None,
            path: None,
        }
    };

    ($real: expr; + $name: expr; = $owner: expr; $repo: expr; @ $branch: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: Some($branch),
            path: None,
        }
    };

    ($real: expr; + $name: expr; = $owner: expr; $repo: expr; $path: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: None,
            path: Some($path),
        }
    };

    ($real: expr; + $name: expr; = $owner: expr; $repo: expr; @ $branch: expr; $path: expr) => {
        LegacyPackage {
            name: $name,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: Some($branch),
            path: Some($path),
        }
    };

    ($real: expr; + $owner: expr; $repo: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: None,
            path: None,
        }
    };

    ($real: expr; + $owner: expr; $repo: expr; @ $branch: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: Some($branch),
            path: None,
        }
    };

    ($real: expr; + $owner: expr; $repo: expr; $path: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: None,
            path: Some($path),
        }
    };

    ($real: expr; + $owner: expr; $repo: expr; @ $branch: expr; $path: expr) => {
        LegacyPackage {
            name: $repo,
            owner: $owner,
            repo: $repo,
            author: $real,
            branch: Some($branch),
            path: Some($path),
        }
    };
}
