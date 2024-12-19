use anyhow::anyhow;
use app_core::Result;
use chrono::NaiveDateTime;
use db::PackageVisibility;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema, ToResponse)]
pub enum Facet {
    GameVersions(Vec<String>),
    Loaders(Vec<String>),
    Tags(Vec<String>),
    Published(NaiveDateTime, NaiveDateTime),
    Updated(NaiveDateTime, NaiveDateTime),
    Downloads(i32, i32),
    Visibility(PackageVisibility),
    Author(i32),
    Manual(String),
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
    Default,
)]
pub enum Sort {
    #[serde(rename = "none")]
    None,

    #[serde(rename = "name")]
    Name,

    #[serde(rename = "published")]
    Published,

    #[serde(rename = "updated")]
    Updated,

    #[serde(rename = "downloads")]
    #[default]
    Downloads,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
    ToSchema,
    ToResponse,
    Default,
)]
pub enum SortMode {
    #[serde(rename = "asc")]
    #[default]
    Ascending,

    #[serde(rename = "desc")]
    Descending,
}

impl Facet {
    pub fn into_filter_string(self) -> String {
        format!(
            "({})",
            match self {
                Self::Visibility(v) => format!("visibility = {}", v.as_str()),
                Self::GameVersions(v) => format!("game_versions IN [{}]", v.join(", ")),
                Self::Loaders(v) => format!("loaders IN [{}]", v.join(", ")),
                Self::Tags(v) => format!("tags IN [{}]", v.join(", ")),
                Self::Published(start, end) => format!(
                    "(created_at >= {}) AND (created_at <= {})",
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp()
                ),
                Self::Updated(start, end) => format!(
                    "(updated_at >= {}) AND (updated_at <= {})",
                    start.and_utc().timestamp(),
                    end.and_utc().timestamp()
                ),
                Self::Downloads(start, end) =>
                    format!("(downloads >= {}) AND (downloads <= {})", start, end),
                Self::Author(v) => format!("author_ids IN [{}]", v),
                Self::Manual(s) => s,
            }
        )
    }

    pub fn parse(it: (String, Vec<String>)) -> Result<Facet> {
        match it.0.as_str() {
            // 'visibility', and 'author', and 'manual' can only be set by the system for security reasons
            "game_versions" => Ok(Facet::GameVersions(it.1)),
            "loaders" => Ok(Facet::Loaders(it.1)),
            "tags" => Ok(Facet::Tags(it.1)),

            "published" => {
                if it.1.len() == 2 {
                    Ok(Self::Published(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(
                        anyhow!("Invalid array length for 'published' facet: {}", it.1.len())
                            .into(),
                    )
                }
            }

            "updated" => {
                if it.1.len() == 2 {
                    Ok(Self::Updated(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(anyhow!("Invalid array length for 'updated' facet: {}", it.1.len()).into())
                }
            }

            "downloads" => {
                if it.1.len() == 2 {
                    Ok(Self::Downloads(it.1[0].parse()?, it.1[1].parse()?))
                } else {
                    Err(
                        anyhow!("Invalid array length for 'downloads' facet: {}", it.1.len())
                            .into(),
                    )
                }
            }

            other => Err(anyhow!("Unknown facet type: {}", other).into()),
        }
    }
}

impl Sort {
    pub fn field(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::Name => "name",
            Self::Published => "created_at",
            Self::Updated => "updated_at",
            Self::Downloads => "downloads",
        }
    }
}

impl SortMode {
    pub fn mode(&self) -> &'static str {
        match self {
            Self::Ascending => "asc",
            Self::Descending => "desc",
        }
    }
}
