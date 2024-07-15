use axum::{extract::State, Json};
use tracing::trace;

use crate::{
    model::lobby::{Lobby, LobbyController, LobbyForCreate},
    web::{ctx::Ctx, error::MainError},
};

pub async fn create_lobby(
    ctx: Ctx,
    State(ctl_lobby): State<LobbyController>,
    Json(lobby_create): Json<LobbyForCreate>,
) -> Result<Json<Lobby>, MainError> {
    trace!("✅ CREATE LOBBY: by user: {ctx:?}");
    let lobby = ctl_lobby
        .create_lobby(lobby_create)
        .await
        .map_err(|e| MainError::Internal(e.to_string()))?;
    Ok(Json(lobby))
}

pub async fn get_lobbies(
    _ctx: Ctx,
    State(lobbies): State<LobbyController>,
) -> Result<Json<Vec<Lobby>>, MainError> {
    let lobbies = lobbies
        .get_lobbies()
        .await
        .map_err(|e| MainError::Internal(e.to_string()))?;
    Ok(Json(lobbies))
}
