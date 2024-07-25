use axum::{
    middleware,
    routing::{get, post},
    Router,
};

use crate::web::app_state::AppState;

mod lobby;
mod status;
mod user;

pub async fn get_api_routes(app_state: &AppState) -> anyhow::Result<axum::Router> {
    let routes_public: Router = Router::new()
        .route("/status", get(status::api_status))
        .route("/login", post(user::login))
        .route("/register", post(user::register))
        .with_state(app_state.clone());

    let routes_private: Router = Router::new()
        .route("/lobby", post(lobby::create_lobby))
        .route("/lobbies", get(lobby::get_lobbies))
        // .route("/account/me", get(account_me))
        .with_state(app_state.clone())
        .route_layer(middleware::from_fn(crate::mw::auth::require_auth));

    let router = routes_public.merge(routes_private);
    Ok(router)
}
