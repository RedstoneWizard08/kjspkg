use anyhow::Result;
use chrono::{DateTime, NaiveDateTime, Utc};
use db::{
    gallery_images, package_authors, package_versions, packages, DbConn, NewGalleryImage,
    NewPackage, NewPackageVersion, Package, PackageAuthor, PackageVersion, PackageVisibility,
};
use diesel::{insert_into, SelectableHelper};
use diesel_async::RunQueryDsl;
use s3::Bucket;
use serde::{Deserialize, Serialize};
use serde_this_or_that::{as_bool, as_i64};
use sha1::{Digest, Sha1};
use std::{fs, path::PathBuf};

pub const DESC_PREFIX: &str = "> *If this is your package, please contact **@RedstoneWizard08** on the [Astroneer Modding Discord](https://discord.gg/bBqdVYxu4k) to claim it!*\n\n";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoolField {
    #[serde(rename = "BOOL", deserialize_with = "as_bool")]
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberField {
    #[serde(rename = "N", deserialize_with = "as_i64")]
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StringField {
    #[serde(rename = "S")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListField<T> {
    #[serde(rename = "L")]
    pub value: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModVersionField {
    #[serde(rename = "M")]
    pub value: DumpVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpVersion {
    pub upload_file_key: StringField,
    pub astro_build: StringField,
    pub version: StringField,
    pub updated: NumberField,
    pub release_file_name: StringField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DumpMod {
    pub published: BoolField,
    pub created: NumberField,
    pub contributers: Option<ListField<StringField>>,
    pub downloads: NumberField,
    pub name: StringField,
    pub versions: ListField<ModVersionField>,
    pub license: StringField,
    pub latest_version: StringField,
    pub updated: NumberField,
    pub mod_id: StringField,
    pub short_description: StringField,
    pub homepage: StringField,
    pub description: StringField,
    pub tags: ListField<StringField>,
    pub author: StringField,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Version {
    pub upload_file_key: String,
    pub astro_build: String,
    pub version: String,
    pub updated: NaiveDateTime,
    pub release_file_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    pub published: bool,
    pub created: NaiveDateTime,
    pub contributers: Vec<String>,
    pub downloads: i32,
    pub name: String,
    pub versions: Vec<Version>,
    pub license: String,
    pub latest_version: String,
    pub updated: NaiveDateTime,
    pub mod_id: String,
    pub short_description: String,
    pub homepage: String,
    pub description: String,
    pub tags: Vec<String>,
    pub author: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModsDump {
    #[serde(rename = "Items")]
    pub items: Vec<DumpMod>,

    #[serde(rename = "Count")]
    pub count: i32,

    #[serde(rename = "ScannedCount")]
    pub scanned_count: i32,

    #[serde(rename = "ConsumedCapacity")]
    pub consumed_capacity: Option<i32>, // I don't actually know what this type is and I don't care!
}

impl From<DumpVersion> for Version {
    fn from(value: DumpVersion) -> Self {
        Self {
            astro_build: value.astro_build.value,
            release_file_name: value.release_file_name.value,
            updated: DateTime::<Utc>::from_timestamp_millis(value.updated.value)
                .unwrap()
                .naive_utc(),
            upload_file_key: value.upload_file_key.value,
            version: value.version.value,
        }
    }
}

impl From<DumpMod> for Mod {
    fn from(value: DumpMod) -> Self {
        Self {
            published: value.published.value,
            created: DateTime::<Utc>::from_timestamp_millis(value.created.value)
                .unwrap()
                .naive_utc(),
            contributers: value
                .contributers
                .unwrap_or_default()
                .value
                .into_iter()
                .map(|v| v.value)
                .collect(),
            downloads: value.downloads.value as i32,
            name: value.name.value,
            versions: value
                .versions
                .value
                .into_iter()
                .map(|v| v.value.into())
                .collect(),
            license: value.license.value,
            latest_version: value.latest_version.value,
            updated: DateTime::<Utc>::from_timestamp_millis(value.updated.value)
                .unwrap()
                .naive_utc(),
            mod_id: value.mod_id.value,
            short_description: value.short_description.value,
            homepage: value.homepage.value,
            description: value.description.value,
            tags: value.tags.value.into_iter().map(|v| v.value).collect(),
            author: value.author.value,
        }
    }
}

impl Version {
    pub async fn upload(&self, bucket: &Box<Bucket>) -> Result<String> {
        let mods_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("mods")
            .join("releaseMods");

        let file_path = mods_dir.join(&self.release_file_name);
        let file = fs::read(file_path)?;
        let mut hasher = Sha1::new();

        hasher.update(&file);

        let file_id = format!("{:x}", hasher.finalize());

        bucket.put_object(format!("/{}", &file_id), &file).await?;

        Ok(file_id)
    }

    pub async fn as_ver(
        self,
        pkg: &Package,
        db: &mut DbConn,
        bucket: &Box<Bucket>,
    ) -> Result<PackageVersion> {
        let id = self.upload(bucket).await?;
        let ver = self.into_ver(pkg, id);

        let ver = insert_into(package_versions::table)
            .values(ver)
            .returning(PackageVersion::as_returning())
            .get_result(db)
            .await?;

        Ok(ver)
    }

    pub fn into_ver(self, pkg: &Package, file_id: String) -> NewPackageVersion {
        NewPackageVersion {
            package: pkg.id,
            name: self.version.clone(),
            version_number: self.version,
            file_id,
            changelog: Some("Migrated from astroneermods.space.".into()),
            loaders: vec![Some("AstroModIntegrator".into())],
            game_versions: vec![Some(self.astro_build)],
            downloads: 0,
        }
    }
}

impl Mod {
    pub fn into_pkg(self) -> NewPackage {
        NewPackage {
            slug: self.mod_id,
            name: self.name,
            readme: format!("{}{}", DESC_PREFIX, self.description),
            description: self.short_description,
            source: None,
            issues: None,
            wiki: None,
            tags: self.tags.into_iter().map(|v| Some(v)).collect(),
            visibility: if self.published {
                PackageVisibility::Public
            } else {
                PackageVisibility::Private
            },
            license: Some(self.license),
        }
    }

    pub async fn upload_all(
        self,
        user_id: i32,
        db: &mut DbConn,
        bucket: &Box<Bucket>,
        imgs: &Box<Bucket>,
    ) -> Result<(Package, Vec<PackageVersion>)> {
        let pkg = self.clone().into_pkg();

        let pkg = insert_into(packages::table)
            .values(pkg)
            .returning(Package::as_returning())
            .get_result(db)
            .await?;

        let author = PackageAuthor {
            user_id,
            package: pkg.id,
        };

        insert_into(package_authors::table)
            .values(author)
            .execute(db)
            .await?;

        let imgs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("mods")
            .join("images");

        let img_path = imgs_dir.join(&self.mod_id);

        if img_path.exists() {
            let img = fs::read(img_path)?;
            let mut hasher = Sha1::new();

            hasher.update(&img);

            let img_id = format!("{:x}", hasher.finalize());

            imgs.put_object(format!("/{}", &img_id), &img).await?;

            let img_data = NewGalleryImage {
                name: self.mod_id.clone(),
                description: None,
                ordering: 0, // We want this to be first, but easily allow the user to override it.
                package: pkg.id,
                s3_id: img_id,
            };

            insert_into(gallery_images::table)
                .values(img_data)
                .execute(db)
                .await?;
        } else {
            println!("No image: {}", self.mod_id);
        }

        let mut vers = Vec::new();

        for ver in self.versions.clone() {
            if ver.release_file_name == "" {
                println!(
                    "Encountered invalid version: {} (package: {})",
                    ver.version, pkg.slug
                );
                continue;
            }

            vers.push(ver.as_ver(&pkg, db, bucket).await?);
        }

        Ok((pkg, vers))
    }
}

impl Into<Vec<Mod>> for ModsDump {
    fn into(self) -> Vec<Mod> {
        self.items.into_iter().map(|v| v.into()).collect()
    }
}
