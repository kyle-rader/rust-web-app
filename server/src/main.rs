use axum::{http::StatusCode, routing::get, Router};
use tokio::signal;
use tracing::instrument::WithSubscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

    // Build server
    let app = Router::new().route("/", get(root));

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

async fn root() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Hello, World!")
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
