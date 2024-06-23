use axum::{http::StatusCode, response::IntoResponse, routing::post, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

use crate::web;

pub fn get_routes() -> axum::Router {
    axum::Router::new().route("/login", post(api_login))
}

#[derive(Debug)]
enum ErrorLogin {
    Failed,
}

impl IntoResponse for ErrorLogin {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, format!("{:?}", self)).into_response()
    }
}

#[derive(Debug, Deserialize)]
struct PayloadLogin {
    email: String,
    password: String,
}

async fn api_login(
    cookies: Cookies,
    payload: Json<PayloadLogin>,
) -> Result<Json<Value>, ErrorLogin> {
    debug!("API_LOGIN: {:?}", payload);

    // TODO: Create real login logic with DB
    if payload.email != "goodguy" || payload.password != "password" {
        return Err(ErrorLogin::Failed);
    }

    // TODO: Create Session and set Cookies
    cookies.add(Cookie::new(web::AUTH_HEADER, "user1.exp.sign"));

    // Create success body
    Ok(Json(json!({
        "result": {
          "success": true,
        }
    })))
}
