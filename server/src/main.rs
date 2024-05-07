use axum::{routing::get, Json, Router};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(feature = "embed_assets")]
mod assets;

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=trace,tower_http=trace,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().without_time().compact())
        .init();

    #[cfg(debug_assertions)]
    let startup_msg = "🐛 (debug) Starting automata server";
    #[cfg(not(debug_assertions))]
    let startup_msg = "🚀 (release) Starting automata server";
    info!("{startup_msg}");

    #[cfg(feature = "embed_assets")]
    assets::print_assets();

    let app = Router::new().route("/api/status", get(api_status));

    #[cfg(feature = "embed_assets")]
    let app = app
        .route("/", get(assets::handler))
        .route("/*file", get(assets::handler));

    let app = app.layer(TraceLayer::new_for_http());

    // get port from env or use default
    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    let address = std::env::var("ADDRESS").unwrap_or(DEFAULT_ADDR.to_string());

    // Get a TCP listener ready to accept incoming connections
    let address = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    // Start server
    info!("🛫 Server running on: http://{}", address);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("🛬 Goodbye!");
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
