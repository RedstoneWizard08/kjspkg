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

pub mod api;
pub mod auth;
pub mod bun;
pub mod env;
pub mod glue;
pub mod logger;
pub mod macros;
pub mod middleware;
pub mod routes;
pub mod search;
pub mod server;
pub mod state;
pub mod ui;
pub mod util;
pub mod worker;

pub use logger::*;
pub use routes::meta::loaders::ModLoader;
pub use routes::meta::vers::GameVersion;
pub use server::*;

pub type Result<T, E = app_core::AppError> = app_core::Result<T, E>;
