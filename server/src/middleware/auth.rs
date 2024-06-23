use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use tower_cookies::Cookies;
use tracing::{debug, trace};

use crate::web::AUTH_HEADER;

#[derive(Debug, Error)]
pub enum ErrorAuth {
    #[error("Missing token")]
    MissingToken,
    #[error("Invalid token")]
    InvalidToken,
}

impl IntoResponse for ErrorAuth {
    fn into_response(self) -> Response {
        (axum::http::StatusCode::UNAUTHORIZED, format!("{:?}", self)).into_response()
    }
}

pub async fn require_auth(
    cookies: Cookies,
    req: Request<Body>,
    next: Next,
) -> Result<Response, ErrorAuth> {
    let auth_token = cookies.get(AUTH_HEADER).map(|c| c.value().to_string());
    trace!("🔐 REQUIRE AUTH: token={:?}", auth_token);

    // TODO: Really validate token

    auth_token.ok_or(ErrorAuth::MissingToken)?;
    Ok(next.run(req).await)
}
