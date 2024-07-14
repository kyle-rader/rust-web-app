use automata::db;
use automata::mw;
use automata::web::app_state::AppState;
use automata::web::{self, routes};

#[cfg(feature = "embed_assets")]
use automata::assets;
#[cfg(not(feature = "embed_assets"))]
use axum::response::Redirect;

use axum::routing::get;
use axum::{middleware, Router};
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tracing::debug;
use tracing::info;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const DEFAULT_PORT: u16 = 3000;
const DEFAULT_ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();
    welcome_message();

    #[cfg(feature = "embed_assets")]
    assets::print_assets();

    debug!("🛠️  Creating Routes...");
    let app = Router::new();

    #[cfg(feature = "embed_assets")]
    debug!("🛠️  Embedding assets...");
    #[cfg(feature = "embed_assets")]
    let app = app
        .route("/", get(assets::handler))
        .route("/*file", get(assets::handler));

    #[cfg(not(feature = "embed_assets"))]
    debug!("🛠️  No embedded assets: redirect '/' to localhost:5173...");

    #[cfg(not(feature = "embed_assets"))]
    let app = app.route("/", get(|| async { Redirect::to("http://localhost:5173") }));

    // Create database connection pool
    let db_pool = db::get_db_pool()?;

    // Run database migrations before starting the server
    db::run_migrations(db_pool.get()?)?;

    // Create app state (xfer the db_pool into the app state for sharing across routes)
    let app_state = AppState::new(db_pool).await?;

    let api_routes = routes::get_api_routes(&app_state).await?;

    let app = app
        .nest("/api", api_routes)
        .layer(middleware::map_response(web::main_response_mapper))
        .layer(middleware::from_fn_with_state(
            app_state,
            mw::auth::ctx_resolver,
        ))
        .layer(middleware::map_request(web::main_request_mapper))
        .layer(CookieManagerLayer::new());

    // get port from env or use default
    let port = std::env::var("PORT").unwrap_or(DEFAULT_PORT.to_string());
    let address = std::env::var("ADDRESS").unwrap_or(DEFAULT_ADDR.to_string());

    // Get a TCP listener ready to accept incoming connections
    let address = format!("{address}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;

    // Start server
    info!("🛫 Server running on: http://{}\n", address);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    info!("🛬 Goodbye!");
    Ok(())
}

fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=trace,automata=trace,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer().compact().without_time())
        .init();
}

fn welcome_message() {
    #[cfg(debug_assertions)]
    let startup_msg = "🐛 (debug) Starting automata server";
    #[cfg(not(debug_assertions))]
    let startup_msg = "🚀 (release) Starting automata server";
    info!("{startup_msg}");
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
