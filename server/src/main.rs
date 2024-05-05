use axum::{routing::get, Json, Router};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod assets;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(debug_assertions)]
    let startup_msg = "🐛 (debug) Starting automata server";
    #[cfg(not(debug_assertions))]
    let startup_msg = "🚀 (release) Starting automata server";
    println!("{startup_msg}");

    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    assets::print_assets();

    // Build server
    let app = Router::new()
        .route("/api/status", get(api_status))
        .route("/", get(assets::handler))
        .route("/*file", get(assets::handler));

    // get port from env or use default
    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    let address = std::env::var("ADDRESS").unwrap_or(DEFAULT_ADDR.to_string());

    // Get a TCP listener ready to accept incoming connections
    let address = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    // Start server
    println!("🛫 Server running on: http://{}", address);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    println!("🛬 Goodbye!");
    Ok(())
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Error: failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("⚠️ Shutting down...");
}
