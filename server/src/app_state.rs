use crate::{model, service};

pub struct AppState {
    pub jwt_controller: service::jwt::JwtController,
    pub account_ctl: model::account::AccountController,
}

impl AppState {
    pub async fn new() -> anyhow::Result<Self> {
        Ok(Self {
            jwt_controller: service::jwt::JwtController::new()?,
            account_ctl: model::account::AccountController::new().await?,
        })
    }
}
