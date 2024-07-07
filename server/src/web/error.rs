use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

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
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}

impl MainError {
    pub fn client_response(&self) -> (StatusCode, ErrorClient) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ErrorClient::LoginFail),
            Self::AuthFailNoAuthTokenCookie
            | Self::AuthFailTokenWrongFormat
            | Self::AuthFailCtxNotInRequest => (StatusCode::FORBIDDEN, ErrorClient::NoAuth),
            Self::AccountNotFound => (StatusCode::BAD_REQUEST, ErrorClient::InvalidParams),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, ErrorClient::ServiceError),
        }
    }
}

#[derive(Debug, strum_macros::AsRefStr, thiserror::Error)]
pub enum ErrorClient {
    #[error("Login Failed")]
    LoginFail,

    #[error("Authentication required")]
    NoAuth,

    #[error("Invalid request parameters")]
    InvalidParams,

    #[error("Internal server error")]
    ServiceError,
}
