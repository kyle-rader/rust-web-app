use axum::{routing::get, Json, Router};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod assets;

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

    // run app with hyper
    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("🛫 Server running on: http://{}", addr);
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
