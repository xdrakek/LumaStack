use axum::{response::Html, routing::get, Router};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lumastack_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Build application router with routes
    let app = Router::new()
        .route("/", get(root_handler))
        .layer(TraceLayer::new_for_http());

    // Define the address to listen on
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("LumaStack backend listening on {}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/// Root handler that returns a welcome message
async fn root_handler() -> Html<&'static str> {
    Html("<h1>Hola LumaStack</h1>")
}
