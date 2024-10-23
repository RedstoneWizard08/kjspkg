pub mod info;
pub mod me;
pub mod pkg;

use crate::state::AppState;
use axum::{routing::get, Router};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/me", get(me::me_handler))
        .route("/:id", get(info::info_handler))
        .route("/:id/packages", get(pkg::list_handler))
        .with_state(state)
}
