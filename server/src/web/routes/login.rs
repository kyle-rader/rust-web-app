use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::trace;

use crate::{
    service::{self, jwt::Claims},
    web::{self, error::MainError},
};

#[derive(Debug, Deserialize)]
pub struct PayloadLogin {
    email: String,
    password: String,
}

pub async fn api_login(
    State(ctl_jwt): State<service::jwt::JwtController>,
    cookies: Cookies,
    payload: Json<PayloadLogin>,
) -> Result<Json<Value>, MainError> {
    trace!("API_LOGIN: {:#?}", payload);

    // TODO: Create real login logic with DB
    if payload.email != "goodguy" || payload.password != "password" {
        return Err(MainError::LoginFail);
    }

    // Create JWT token
    let claims = Claims::new(1, "goodguy".to_string(), "goodguy@contoso.com".to_string());

    let token = ctl_jwt.sign(&claims).map_err(|jwt_error| {
        trace!("❌ Login JWT Signing Error {jwt_error}");
        MainError::LoginFail
    })?;

    cookies.add(Cookie::new(web::AUTH_HEADER, token));

    trace!("✅ Login Success");

    // Create success body
    Ok(Json(json!({
        "result": {
          "success": true,
        }
    })))
}
