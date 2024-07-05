use axum::{
    extract::State,
    middleware,
    routing::{get, post},
    Json, Router,
};
use login::api_login;

use crate::{
    model::user::{ControllerUser, User},
    web::app_state::AppState,
};

use super::{ctx::Ctx, error::MainError};

mod lobby;
mod login;
mod status;

pub async fn get_api_routes() -> anyhow::Result<(AppState, axum::Router)> {
    let app_state = AppState::new().await?;

    let routes_public: Router = Router::new()
        .route("/status", get(status::api_status))
        .route("/login", post(api_login))
        .with_state(app_state.clone());

    let routes_private: Router = Router::new()
        .route("/lobby", post(lobby::create_lobby))
        .route("/lobbies", get(lobby::get_lobbies))
        // .route("/account/me", get(account_me))
        .with_state(app_state.clone())
        .route_layer(middleware::from_fn(crate::mw::auth::require_auth));

    let router = routes_public.merge(routes_private);
    Ok((app_state, router))
}

// async fn account_me(
//     ctx: Ctx,
//     State(ctl_account): State<ControllerUser>,
// ) -> Result<Json<User>, MainError> {
//     let account = ctl_account
//         .get_account(ctx.account_id)
//         .await
//         .map_err(|_| MainError::AccountNotFound)?;
//     Ok(Json(account))
// }
