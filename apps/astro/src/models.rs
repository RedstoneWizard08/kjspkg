use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Revision {
    // We don't care about any of the other fields
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    // We don't care about any of the other fields
    pub revision: Revision,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaWiki {
    // We don't care about the siteinfo block.
    pub page: Page,
}
