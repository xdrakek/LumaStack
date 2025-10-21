use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod cli;
mod db;
mod handlers;
mod models;

use cli::{Cli, Commands};
use handlers::{health_handler, root_handler, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lumastack_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL")?;

    // Create database connection pool
    let pool = db::create_pool(&database_url).await?;

    // Run migrations
    db::run_migrations(&pool).await?;

    // Handle CLI commands
    match cli.command {
        Some(Commands::CreateAdmin {
            email,
            username,
            password,
        }) => {
            // Ejecutar comando create-admin
            cli::commands::create_admin(&pool, email, username, password).await?;
            return Ok(());
        }
        Some(Commands::Serve) | None => {
            // Continuar con el servidor (default)
        }
    }

    // Create application state
    let state = AppState { db: pool };

    // Build application router with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Get server configuration from environment
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    // Define the address to listen on
    let addr = SocketAddr::from((host.parse::<std::net::IpAddr>()?, port));

    tracing::info!("🚀 LumaStack backend listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
