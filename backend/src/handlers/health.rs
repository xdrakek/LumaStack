use axum::{extract::State, Json};
use chrono::Utc;
use serde_json::{json, Value};
use sqlx::PgPool;

/// Estado de la aplicación compartido entre handlers
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

/// Handler para la ruta raíz que retorna información de la API
///
/// # Endpoint
/// `GET /`
///
/// # Response
/// JSON con información del proyecto, versión, endpoints disponibles, etc.
///
/// # Example Response
/// ```json
/// {
///   "name": "LumaStack API",
///   "description": "Platform for monitoring and managing Git repositories with Telegram integration",
///   "version": "0.1.0",
///   "status": "operational",
///   "timestamp": "2025-10-21T03:15:42.123Z",
///   "environment": "development",
///   "endpoints": {
///     "health": "GET /health",
///     "api_docs": "Coming soon"
///   }
/// }
/// ```
pub async fn root_handler() -> Json<Value> {
    Json(json!({
        "name": "LumaStack API",
        "description": "Platform for monitoring and managing Git repositories with Telegram integration",
        "version": env!("CARGO_PKG_VERSION"),
        "status": "operational",
        "timestamp": Utc::now().to_rfc3339(),
        "environment": std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()),
        "endpoints": {
            "health": "GET /health",
            "documentation": "Coming soon",
            "api_v1": "Coming soon"
        },
        "features": [
            "Git repository monitoring",
            "User authentication",
            "Telegram notifications",
            "Real-time updates via WebSockets"
        ],
        "repository": "https://github.com/xdrakek/LumaStack",
        "authors": env!("CARGO_PKG_AUTHORS")
    }))
}

/// Handler de health check que verifica la conectividad de la base de datos
///
/// # Endpoint
/// `GET /health`
///
/// # Response
/// JSON con el estado del servidor y la base de datos
///
/// # Example Response
/// ```json
/// {
///   "status": "ok",
///   "database": "healthy"
/// }
/// ```
pub async fn health_handler(State(state): State<AppState>) -> Json<Value> {
    // Intentar ejecutar una query simple para verificar la conexión
    let db_status = sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .map(|_| "healthy")
        .unwrap_or("unhealthy");

    Json(json!({
        "status": "ok",
        "database": db_status,
        "version": env!("CARGO_PKG_VERSION")
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_root_handler() {
        let response = root_handler().await;
        let json = response.0;

        // Verificar que contiene los campos esperados
        assert_eq!(json["name"], "LumaStack API");
        assert_eq!(json["status"], "operational");
        assert!(json["version"].as_str().is_some());
        assert!(json["timestamp"].as_str().is_some());
        assert!(json["endpoints"].is_object());
        assert!(json["features"].is_array());
    }

    #[tokio::test]
    async fn test_root_handler_has_required_fields() {
        let response = root_handler().await;
        let json = response.0;

        // Campos obligatorios
        assert!(json.get("name").is_some());
        assert!(json.get("description").is_some());
        assert!(json.get("version").is_some());
        assert!(json.get("status").is_some());
        assert!(json.get("endpoints").is_some());
    }
}
