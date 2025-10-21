pub mod users;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Crea y configura el pool de conexiones a PostgreSQL
///
/// # Configuración (vía variables de entorno)
/// - `DB_MIN_CONNECTIONS`: Conexiones mínimas (default: 1)
/// - `DB_MAX_CONNECTIONS`: Conexiones máximas (default: 10)
/// - `DB_ACQUIRE_TIMEOUT`: Timeout para adquirir conexión en segundos (default: 30)
/// - `DB_IDLE_TIMEOUT`: Timeout de inactividad en segundos (default: 600)
///
/// # Errors
/// Retorna error si no se puede conectar a la base de datos
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Conectando a la base de datos...");

    // Leer configuración del pool desde variables de entorno con valores por defecto
    let min_connections = std::env::var("DB_MIN_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(1);

    let max_connections = std::env::var("DB_MAX_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(10);

    let acquire_timeout_secs = std::env::var("DB_ACQUIRE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(30);

    let idle_timeout_secs = std::env::var("DB_IDLE_TIMEOUT")
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(600);

    tracing::info!(
        "Configuración del pool: min={}, max={}, acquire_timeout={}s, idle_timeout={}s",
        min_connections,
        max_connections,
        acquire_timeout_secs,
        idle_timeout_secs
    );

    let pool = PgPoolOptions::new()
        .min_connections(min_connections)
        .max_connections(max_connections)
        .acquire_timeout(Duration::from_secs(acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(idle_timeout_secs))
        .connect(database_url)
        .await?;

    tracing::info!("✓ Pool de conexiones creado exitosamente");

    Ok(pool)
}

/// Ejecuta las migraciones pendientes
///
/// # Errors
/// Retorna error si las migraciones fallan
pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Ejecutando migraciones...");

    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    tracing::info!("✓ Migraciones completadas");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Ignorar por defecto, requiere PostgreSQL corriendo
    async fn test_create_pool() {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost/lumastack_test".to_string());

        let result = create_pool(&database_url).await;
        assert!(result.is_ok());
    }
}
