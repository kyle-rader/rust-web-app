use axum::Router;

use crate::model::lobby::LobbyController;

mod routes_lobby;
mod routes_login;
mod routes_status;

pub const AUTH_HEADER: &str = "auth-token";

pub async fn get_routes_public() -> anyhow::Result<axum::Router> {
    Ok(routes_status::get_routes().merge(routes_login::get_routes()))
}

pub async fn get_routes_api() -> anyhow::Result<axum::Router> {
    Ok(routes_lobby::get_routes(LobbyController::new().await?))
}
