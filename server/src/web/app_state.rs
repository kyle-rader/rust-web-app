use axum::extract::FromRef;

use crate::{db::DbPool, model::lobby::LobbyController, service::jwt::JwtController};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: DbPool,
    pub ctl_lobby: LobbyController,
    pub ctl_jwt: JwtController,
}

impl AppState {
    pub async fn new(db_pool: DbPool) -> anyhow::Result<Self> {
        Ok(Self {
            db_pool,
            ctl_lobby: LobbyController::new().await?,
            ctl_jwt: JwtController::new()?,
        })
    }
}

// region: Implement FromRef<AppState> for Sub State Controllers
impl FromRef<AppState> for DbPool {
    fn from_ref(app_state: &AppState) -> DbPool {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for LobbyController {
    fn from_ref(app_state: &AppState) -> LobbyController {
        app_state.ctl_lobby.clone()
    }
}

impl FromRef<AppState> for JwtController {
    fn from_ref(app_state: &AppState) -> JwtController {
        app_state.ctl_jwt.clone()
    }
}
// endregion
