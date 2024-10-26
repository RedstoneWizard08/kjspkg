use anyhow::Result;
use jsglue::{config::GlueConfig, framework::Framework, glue::Glue};

#[cfg(debug_assertions)]
mod client {
    use jsglue::include_dir::Dir;

    pub const CLIENT_DIR: Option<Dir<'static>> = None;
}

#[cfg(not(debug_assertions))]
mod client {
    use jsglue::include_dir::{self, include_dir, Dir};

    pub const CLIENT_DIR: Option<Dir<'static>> =
        Some(include_dir!("$CARGO_MANIFEST_DIR/../../ui/build"));
}

/// Create a new [`Glue`] instance.
pub fn make_glue() -> Result<Glue> {
    Ok(Glue::new(
        GlueConfig::builder()
            .base("http://localhost:4001")
            .dir(client::CLIENT_DIR)
            .project(format!("{}/../../ui", env!("CARGO_MANIFEST_DIR")))
            .cmd("bun")
            .arg("run")
            .arg("dev")
            .framework(Framework::Vite("/vite-hmr"))
            .build()?,
    ))
}
