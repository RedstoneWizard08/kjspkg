#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate axum;

#[macro_use]
extern crate utoipa;

pub(crate) mod api;
pub(crate) mod auth;
pub(crate) mod cli;
pub(crate) mod db;
pub(crate) mod error;
pub(crate) mod glue;
pub(crate) mod logger;
pub(crate) mod middleware;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod schema;
pub(crate) mod state;
pub(crate) mod util;

use std::net::{IpAddr, SocketAddr};

use axum::serve;
pub use cli::*;
pub use error::*;
use glue::make_glue;
use jsglue::{abort::register_exit_handler, util::is_debug};
pub use logger::*;
pub use models::*;

use anyhow::Result;
use db::{create_connection, run_migrations};
use routes::create_router;
use state::AppState;
use tokio::{join, net::TcpListener};

pub async fn start_app(cli: Cli) -> Result<()> {
    info!("Starting app...");

    register_exit_handler()?;

    info!("Connecting to the database...");

    let pool = create_connection(cli.db_url).await?;

    info!("Running migrations...");

    run_migrations(&pool).await?;

    info!("Creating state...");

    let state = AppState::new(
        pool,
        cli.github_client_id,
        cli.github_client_secret,
        cli.supabase_url,
        cli.supabase_key,
        cli.packages_bucket,
    )?;

    info!("Creating glue...");

    let glue = make_glue()?;

    info!("Registering routes...");

    let router =
        create_router(state, glue.clone()).into_make_service_with_connect_info::<SocketAddr>();

    info!("Binding listener...");

    let ip: IpAddr = cli.host.parse()?;
    let addr = SocketAddr::from((ip, cli.port));
    let listener = TcpListener::bind(&addr).await?;

    info!("Started! Listening on {}:{}", cli.host, cli.port);

    let server = tokio::spawn(async move { serve(listener, router).await });

    if is_debug() {
        info!("Starting client...");

        let client = glue.spawn().await;
        let (a, b) = join!(client, server);

        a?;
        b??;

        return Ok(());
    }

    server.await??;

    Ok(())
}
