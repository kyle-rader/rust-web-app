use axum::extract::FromRef;

use crate::{
    model::{account::AccountController, lobby::LobbyController},
    service::jwt::JwtController,
};

#[derive(Clone)]
pub struct AppState {
    pub ctl_account: AccountController,
    pub ctl_lobby: LobbyController,
    pub ctl_jwt: JwtController,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            ctl_account: AccountController::new().await?,
            ctl_lobby: LobbyController::new().await?,
            ctl_jwt: JwtController::new()?,
        })
    }
}

// region: Implement FromRef<AppState> for Sub State Controllers
impl FromRef<AppState> for AccountController {
    fn from_ref(app_state: &AppState) -> AccountController {
        app_state.ctl_account.clone()
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
