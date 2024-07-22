use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::model::user::{self, ErrorUser};

#[derive(Debug, Clone, Serialize, thiserror::Error)]
#[serde(tag = "type", content = "data")]
pub enum MainError {
    #[error("Login failed")]
    LoginFail,

    // Auth errors
    #[error("No auth-header cookie found")]
    AuthFailNoAuthTokenCookie,

    #[error("Bad auth token: {0}")]
    AuthFailToken(String),

    #[error("Auth Ctx not in request")]
    AuthFailCtxNotInRequest,

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error(transparent)]
    User(#[from] user::ErrorUser),

    #[error("Error: {0}")]
    ClientError(String),
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
            | Self::AuthFailToken(_)
            | Self::AuthFailCtxNotInRequest => (StatusCode::FORBIDDEN, ErrorClient::NoAuth),
            Self::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorClient::ServiceError),
            Self::User(e) => e.into(),
            Self::ClientError(e) => (
                StatusCode::BAD_REQUEST,
                ErrorClient::BadRequest(e.to_string()),
            ),
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

    #[error("{0}")]
    BadRequest(String),

    #[error("Internal server error")]
    ServiceError,
}

impl From<&ErrorUser> for (StatusCode, ErrorClient) {
    fn from(value: &ErrorUser) -> Self {
        match value {
            ErrorUser::DisplayNameAlreadyExists
            | ErrorUser::EmailAlreadyExists
            | ErrorUser::InvalidEmail
            | ErrorUser::InvalidCredentials
            | ErrorUser::NotFound => (
                StatusCode::BAD_REQUEST,
                ErrorClient::BadRequest(value.to_string()),
            ),
            ErrorUser::Password(e) => (
                StatusCode::BAD_REQUEST,
                ErrorClient::BadRequest(e.to_string()),
            ),
            ErrorUser::Db(_) => (StatusCode::INTERNAL_SERVER_ERROR, ErrorClient::ServiceError),
        }
    }
}
