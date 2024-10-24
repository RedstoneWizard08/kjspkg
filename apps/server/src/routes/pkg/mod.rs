pub mod author;
pub mod info;
pub mod list;
pub mod ver;

use crate::state::AppState;
use axum::{
    routing::{delete, get, patch, put},
    Router,
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/", get(list::list_handler))
        .route("/", put(list::create_handler))
        .route("/:id", get(info::info_handler))
        .route("/:id", patch(info::update_handler))
        .route("/:id", delete(info::delete_handler))
        .route("/:id/authors", get(author::list_handler))
        .route("/:id/authors", put(author::add_handler))
        .route("/:id/versions", get(ver::list_handler))
        .route("/:id/versions", put(ver::create_handler))
        .route("/:id/versions/:version", get(ver::info_handler))
        .route("/:id/versions/:version", patch(ver::update_handler))
        .route("/:id/versions/:version", delete(ver::delete_handler))
        .route(
            "/:id/versions/:version/download",
            get(ver::download_handler),
        )
        .with_state(state)
}