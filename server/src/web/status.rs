use axum::{routing::get, Json};

pub fn get_routes() -> axum::Router {
    axum::Router::new().route("/status", get(api_status))
}

#[derive(serde::Serialize)]
enum ApiStatus {
    #[serde(rename = "ok ✅")]
    Ok,
}

#[derive(serde::Serialize)]
struct ApiStatusResponse {
    status: ApiStatus,
}

async fn api_status() -> Json<ApiStatusResponse> {
    Json(ApiStatusResponse {
        status: ApiStatus::Ok,
    })
}
