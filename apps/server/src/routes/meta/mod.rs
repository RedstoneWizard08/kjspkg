pub mod badge;

use crate::state::AppState;
use axum::{routing::get, Router};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/badge/version/:version", get(badge::version_handler))
        .with_state(state)
}
