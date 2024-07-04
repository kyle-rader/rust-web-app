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
        .map_err(|_err| MainError::Internal)?;
    Ok(Json(lobby))
}
