use axum::{extract::State, response::IntoResponse, routing::post, Json};
use serde::Serialize;
use thiserror::Error;
use tower_cookies::Cookies;

use crate::model::lobby::{ErrorLobby, Lobby, LobbyController, LobbyForCreate};

pub fn get_routes(controller: LobbyController) -> axum::Router {
    axum::Router::new()
        .route("/lobby", post(create_lobby))
        .with_state(controller)
}

#[derive(Debug, Error)]
enum Error {
    #[error(transparent)]
    Model(#[from] ErrorLobby),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, format!("{:?}", self)).into_response()
    }
}

async fn create_lobby(
    _cookies: Cookies,
    State(lobby_controller): State<LobbyController>,
    Json(lobby_create): Json<LobbyForCreate>,
) -> Result<Json<Lobby>, Error> {
    let lobby = lobby_controller.create_lobby(lobby_create).await?;
    Ok(Json(lobby))
}
