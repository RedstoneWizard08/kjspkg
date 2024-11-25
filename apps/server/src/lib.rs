#[macro_use]
extern crate serde;

#[macro_use]
extern crate tracing;

#[macro_use]
extern crate axum;

#[macro_use]
extern crate utoipa;

#[macro_use]
extern crate lazy_static;

pub(crate) mod api;
pub(crate) mod auth;
pub(crate) mod bun;
pub(crate) mod env;
pub(crate) mod glue;
pub(crate) mod logger;
pub(crate) mod middleware;
pub(crate) mod routes;
pub(crate) mod server;
pub(crate) mod state;
pub(crate) mod ui;
pub(crate) mod util;
pub(crate) mod worker;

pub use logger::*;
pub use routes::meta::vers::GameVersion;
pub use server::*;

pub type Result<T, E = app_core::AppError> = app_core::Result<T, E>;
