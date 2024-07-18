use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::{debug, trace};

use crate::{
    db::{get_db_conn, DbPool},
    model::user::{self, UserPublic},
    service::{self, jwt::Claims},
    web::{self, error::MainError},
};

#[derive(Debug, Deserialize)]
pub struct PayloadLogin {
    email: String,
    password: String,
}

pub async fn login(
    State(ctl_jwt): State<service::jwt::JwtController>,
    State(db_pool): State<DbPool>,
    cookies: Cookies,
    payload: Json<PayloadLogin>,
) -> Result<Json<Value>, MainError> {
    let conn = get_db_conn(&db_pool)?;

    let claims: Claims = user::login(conn, &payload.email, &payload.password).await?;

    let token = ctl_jwt.sign(&claims).map_err(|jwt_error| {
        debug!("❌ Login JWT Signing Error {jwt_error}");
        MainError::LoginFail
    })?;

    cookies.add(Cookie::new(web::AUTH_HEADER, token));

    debug!("✅ Login {}", claims.email);

    // Create success body
    Ok(Json(json!({
        "result": {
          "success": true,
        }
    })))
}

pub async fn register(
    State(db_pool): State<DbPool>,
    Json(fields): Json<user::UserNewFields>,
) -> Result<Json<UserPublic>, MainError> {
    trace!("Register:\n{fields:#?}");

    let conn = get_db_conn(&db_pool)?;

    let user = user::create(conn, fields).await?;

    debug!("✅ Register {}", user.email);

    Ok(Json(user.into()))
}
