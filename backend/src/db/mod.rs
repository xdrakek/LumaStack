pub mod users;

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Crea y configura el pool de conexiones a PostgreSQL
///
/// # Configuración
/// - Min connections: 1
/// - Max connections: 10
/// - Acquire timeout: 30 segundos
/// - Idle timeout: 10 minutos
///
/// # Errors
/// Retorna error si no se puede conectar a la base de datos
pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Conectando a la base de datos...");

    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(600))
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
