use std::{process::exit, time::Duration};

use eyre::{eyre, Result};
use indicatif::ProgressBar;
use kjspkg_api::apis::DEFAULT_API_BASE;
use tiny_http::Server;
use url::Url;

use crate::{
    ctx::{CliContext, RawCliContext},
    util::get_spinner_style,
};

pub async fn cmd_login(_cx: &CliContext, instance: Option<String>) -> Result<()> {
    let api_base = if let Some(inst) = instance {
        if inst.contains("/api/v1") {
            inst
        } else if inst.ends_with("/") {
            format!("{}api/v1", inst)
        } else {
            format!("{}/api/v1", inst)
        }
    } else {
        DEFAULT_API_BASE.into()
    };

    let port = portpicker::pick_unused_port().ok_or(eyre!("Could not find an open port!"))?;
    let mut url = Url::parse(&format!("{}/auth/github/login", api_base))?;

    url.query_pairs_mut()
        .append_pair("redirect_uri", &format!("http://localhost:{}", port));

    info!("Please continue in your browser.");

    open::that(url.to_string())?;

    let server = Server::http(format!("0.0.0.0:{}", port)).map_err(|v| eyre!(v))?;
    let token;

    info!("Server listening on port {}", port);

    let pb = ProgressBar::new_spinner().with_style(get_spinner_style());

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("Waiting for token...");

    'listen: loop {
        let req = server.recv()?;
        let url = req.url();

        let url = if url.starts_with("/") {
            format!("http://localhost:{}/{}", port, url)
        } else {
            url.into()
        };

        let url = Url::parse(&url)?;
        let query = url.query_pairs();

        for (k, v) in query {
            if k == "token" {
                token = v.to_string();
                break 'listen;
            }
        }
    }

    pb.finish_and_clear();

    info!("Got a token: {}", token);

    CliContext::of(RawCliContext {
        api_base: Some(api_base),
        token: Some(token),
    })?
    .save()?;

    exit(0); // We have to exit or it'll get overwritten.
}
