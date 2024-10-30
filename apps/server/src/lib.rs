#![feature(type_alias_impl_trait)]

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
pub(crate) mod env;
pub(crate) mod error;
pub(crate) mod glue;
pub(crate) mod logger;
pub(crate) mod middleware;
pub(crate) mod models;
pub(crate) mod routes;
pub(crate) mod schema;
pub(crate) mod state;
pub(crate) mod util;
pub(crate) mod worker;

pub use cli::*;
pub use error::*;
pub use logger::*;
pub use models::*;

use axum::serve;
use db::{create_connection, run_migrations};
use glue::make_glue;
use jsglue::{abort::register_exit_handler, util::is_debug};
use routes::create_router;
use shuttle_axum::ShuttleAxum;
use state::AppState;
use std::net::{IpAddr, SocketAddr};
use tokio::{join, net::TcpListener};
use worker::run_worker;

pub async fn start_app(cli: Cli) -> Result<()> {
    info!("Starting app...");

    register_exit_handler()?;

    info!("Connecting to the database...");

    let pool = create_connection(cli.db_url).await?;

    info!("Running migrations...");

    run_migrations(&pool).await?;

    info!("Creating state...");

    let state = AppState::new(
        pool.clone(),
        cli.github_client_id,
        cli.github_client_secret,
        cli.supabase_url,
        cli.supabase_key,
        cli.packages_bucket,
    )?;

    info!("Creating glue...");

    let glue = make_glue()?;

    info!("Starting worker...");

    run_worker(pool);

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

pub async fn create_shuttle_axum() -> ShuttleAxum {
    info!("Starting app...");

    register_exit_handler()?;

    info!("Connecting to the database...");

    let pool = create_connection(None)
        .await
        .map_err(|v| Into::<shuttle_runtime::Error>::into(v))?;

    info!("Running migrations...");

    run_migrations(&pool)
        .await
        .map_err(|v| Into::<shuttle_runtime::Error>::into(v))?;

    info!("Creating state...");

    let state = AppState::new(pool.clone(), None, None, None, None, None)
        .map_err(|v| Into::<shuttle_runtime::Error>::into(v))?;

    info!("Creating glue...");

    let glue = make_glue().map_err(|v| Into::<shuttle_runtime::Error>::into(v))?;

    info!("Starting worker...");

    run_worker(pool);

    info!("Registering routes...");

    let router = create_router(state, glue.clone());

    Ok(router.into())
}
