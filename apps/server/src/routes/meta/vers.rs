use crate::{state::AppState, Result};
use axum::{extract::State, Json};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, ToResponse)]
pub struct GameVersion {
    pub id: String,
    pub beta: bool,
}

/// Get Game Versions
///
/// Get a list of game versions.
#[utoipa::path(
    get,
    path = "/api/v1/meta/game_versions",
    tag = "Meta",
    responses(
        (status = 200, description = "Got game versions!", body = Vec<GameVersion>),
        (status = INTERNAL_SERVER_ERROR, description = "An internal error occured!"),
    ),
)]
#[debug_handler]
pub async fn game_versions_handler(
    State(state): State<AppState>,
) -> Result<Json<Vec<GameVersion>>> {
    Ok(Json(state.game_versions))
}
