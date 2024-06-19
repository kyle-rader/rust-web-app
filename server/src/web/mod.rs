use crate::model::lobby::LobbyController;

mod routes_lobby;
mod routes_login;
mod status;

pub const AUTH_TOKEN: &str = "auth-token";

pub async fn get_routes() -> anyhow::Result<axum::Router> {
    Ok(routes_login::get_routes()
        .merge(status::get_routes())
        .merge(routes_lobby::get_routes(LobbyController::new().await?)))
}
