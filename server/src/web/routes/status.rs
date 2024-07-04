use axum::Json;

#[derive(serde::Serialize)]
enum ApiStatus {
    #[serde(rename = "ok ✅")]
    Ok,
}

#[derive(serde::Serialize)]
pub struct ApiStatusResponse {
    status: ApiStatus,
}

pub async fn api_status() -> Json<ApiStatusResponse> {
    Json(ApiStatusResponse {
        status: ApiStatus::Ok,
    })
}
