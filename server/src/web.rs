use axum::{
    body::Body,
    http::{Request, Response},
    response::IntoResponse,
    Json,
};
use error::MainError;
use serde_json::json;
use tracing::{info, warn};
use uuid::Uuid;

pub mod app_state;
pub mod ctx;
pub mod error;
pub mod routes;

pub const AUTH_HEADER: &str = "auth-token";

pub async fn main_request_mapper(req: Request<Body>) -> Request<Body> {
    let path = req.uri().path();
    let method = req.method();

    info!("🏁 {method} {path}");

    req
}

pub async fn main_response_mapper(res: Response<Body>) -> Response<Body> {
    // Uuid to correlate client and server logs
    let uuid = Uuid::new_v4();

    // Service error is for detailed server logging
    let service_error = res.extensions().get::<MainError>();

    // Client error is for the client to know what went wrong, with minimal information
    let client_error = service_error
        .map(|e| e.client_response())
        .map(|(status, err_client)| {
            let err_client_body = json!({
            "error": {
                "type": err_client.as_ref(),
                "request_id": uuid.to_string(),
                }
                });

            warn!("❌ Client Error Body:\n{err_client_body:#?}");

            (status, Json(err_client_body)).into_response()
        });

    if let Some(e) = service_error {
        warn!("❌ Service Error: {e:#?}");
    }

    let emoji = if res.status().is_success() {
        "✅"
    } else {
        "🛑"
    };
    info!("{emoji} {res:#?}\n");
    client_error.unwrap_or(res)
}
