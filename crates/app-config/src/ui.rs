use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub app: String,
    pub tagline: String,
    pub show_beta: bool,
    pub package_kind: PackageKind,
    pub default_theme: String,
    pub package_file_formats: Vec<String>,
    pub game_beta_name: BetaName,
    pub favicon_png: String,
    pub favicon_ico: String,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum PackageKind {
    #[default]
    Mods,
    Packages,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default,
)]
pub enum BetaName {
    #[default]
    Beta,
    Snapshot,
}

impl PackageKind {
    pub fn stringify(&self) -> &'static str {
        match self {
            Self::Mods => "mods",
            Self::Packages => "packages",
        }
    }
}

impl BetaName {
    pub fn stringify(&self) -> &'static str {
        match self {
            Self::Beta => "beta",
            Self::Snapshot => "snapshot",
        }
    }
}

impl UIConfig {
    pub fn env(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();

        map.insert("PUBLIC_APP".into(), self.app.clone());
        map.insert("PUBLIC_TAGLINE".into(), self.tagline.clone());
        map.insert("PUBLIC_SHOW_BETA".into(), self.show_beta.to_string());

        map.insert(
            "PUBLIC_PKG_TYPE".into(),
            self.package_kind.stringify().into(),
        );

        map.insert("PUBLIC_DEFAULT_THEME".into(), self.default_theme.clone());

        map.insert(
            "PUBLIC_PKG_FILE_FORMATS".into(),
            self.package_file_formats.join(","),
        );

        map.insert(
            "PUBLIC_GAME_BETA_NAME".into(),
            self.game_beta_name.stringify().into(),
        );

        map
    }
}

impl Default for UIConfig {
    fn default() -> Self {
        Self {
            app: "ModHost".into(),
            tagline: "Your home for game mods".into(),
            show_beta: true,
            package_kind: PackageKind::Mods,
            default_theme: "modhost".into(),
            package_file_formats: vec![
                ".pak".into(),
                ".jar".into(),
                ".zip".into(),
                ".tgz".into(),
                ".tar.gz".into(),
            ],
            game_beta_name: BetaName::Beta,
            favicon_ico: "default".into(),
            favicon_png: "default".into(),
        }
    }
}
