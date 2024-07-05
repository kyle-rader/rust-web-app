use axum::{
    body::Body,
    extract::{FromRequestParts, Request, State},
    http::request::Parts,
    middleware::Next,
    response::Response,
};
use tower_cookies::{Cookie, Cookies};
use tracing::trace;

use crate::{
    service,
    web::{ctx::Ctx, error::MainError, AUTH_HEADER},
};

/// Middleware to require authentication for a route
pub async fn require_auth(
    ctx: Result<Ctx, MainError>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, MainError> {
    trace!("🔐  Require Auth: ctx: {ctx:?}");

    // TODO: Is that really always the error?
    ctx?;

    Ok(next.run(req).await)
}

pub async fn ctx_resolver(
    State(ctl_jwt): State<service::jwt::JwtController>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, MainError> {
    let auth_token = cookies.get(AUTH_HEADER).map(|c| c.value().to_string());

    // Compute Result<Ctx, MainError>
    let ctx_result = auth_token
        .ok_or(MainError::AuthFailNoAuthTokenCookie)
        .and_then(|token| {
            ctl_jwt
                .verify(&token)
                .map_err(|_jwt_error| MainError::AuthFailTokenWrongFormat)
        })
        .map(|claims| Ctx::new(claims.sub));

    // Remove the cookie if something went wrong other than NoAuthToken
    if let Err(MainError::AuthFailNoAuthTokenCookie) = ctx_result {
        cookies.remove(Cookie::from(AUTH_HEADER));
    }

    if ctx_result.is_ok() {
        trace!("⚡️ ✅ Ctx Resolver: {ctx_result:?}");
    } else {
        trace!("⚡️ ❌ Ctx Resolver: {ctx_result:?}");
    }

    // Set Ctx in request
    req.extensions_mut().insert(ctx_result);

    Ok(next.run(req).await)
}

#[async_trait::async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = MainError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, MainError> {
        let r = parts.extensions.get::<Result<Ctx, MainError>>();
        let res = match r {
            Some(Ok(ctx)) => Ok(ctx.clone()),
            Some(Err(e)) => Err(e.clone()),
            _ => Err(MainError::AuthFailCtxNotInRequest),
        };

        if res.is_ok() {
            trace!("🏗️  ✅ Ctx Extractor: {res:?}");
        } else {
            trace!("🏗️  ❌ Ctx Extractor: {res:?}");
        }

        res
    }
}
