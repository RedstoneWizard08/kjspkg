use anyhow::Result;
use url::Url;

pub trait ApiHelper {
    fn base(&self) -> &String;

    fn url(&self, path: impl AsRef<str>) -> Result<Url> {
        let base = self.base();
        let slash = if base.ends_with("/") { "" } else { "/" };

        Ok(Url::parse(&format!("{}{}{}", base, slash, path.as_ref()))?)
    }
}
