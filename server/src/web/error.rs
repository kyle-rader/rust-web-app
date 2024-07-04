use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::trace;

#[derive(Debug, Clone, Serialize, thiserror::Error)]
#[serde(tag = "type", content = "data")]
pub enum MainError {
    #[error("Login failed")]
    LoginFail,

    // Auth errors
    #[error("No auth-header cookie found")]
    AuthFailNoAuthTokenCookie,

    #[error("Auth token wrong format")]
    AuthFailTokenWrongFormat,

    #[error("Auth Ctx not in request")]
    AuthFailCtxNotInRequest,

    // Account errors
    #[error("Account not found")]
    AccountNotFound,

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for MainError {
    fn into_response(self) -> axum::response::Response {
        trace!("MainError IntoResponse: {:?}", self);
        let status = match self {
            MainError::LoginFail => StatusCode::UNAUTHORIZED,
            MainError::AuthFailNoAuthTokenCookie => StatusCode::UNAUTHORIZED,
            MainError::AuthFailTokenWrongFormat => StatusCode::UNAUTHORIZED,
            MainError::AuthFailCtxNotInRequest => StatusCode::UNAUTHORIZED,
            MainError::AccountNotFound => StatusCode::NOT_FOUND,
            MainError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
