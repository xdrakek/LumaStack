use sqlx::PgPool;
use crate::models::{User, CreateUser, UpdateUser, UserRole};

/// Errores relacionados con operaciones de usuarios
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Usuario no encontrado")]
    NotFound,

    #[error("El usuario ya existe: {0}")]
    AlreadyExists(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, UserError>;

/// Crea un nuevo usuario en la base de datos
///
/// # Errors
/// - `UserError::AlreadyExists` si el username o email ya existen
/// - `UserError::Database` si hay un error de base de datos
pub async fn create_user(pool: &PgPool, user: CreateUser, password_hash: String) -> Result<User> {
    let result = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (username, email, password_hash, role)
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        user.username,
        user.email,
        password_hash,
        user.role as UserRole
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(user) => Ok(user),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Err(UserError::AlreadyExists(
                db_err.message().to_string()
            ))
        }
        Err(e) => Err(UserError::Database(e)),
    }
}

/// Busca un usuario por ID
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
pub async fn find_user_by_id(pool: &PgPool, id: i32) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(UserError::NotFound)
}

/// Busca un usuario por email
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
pub async fn find_user_by_email(pool: &PgPool, email: &str) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(UserError::NotFound)
}

/// Busca un usuario por username
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?
    .ok_or(UserError::NotFound)
}

/// Lista todos los usuarios activos
///
/// # Errors
/// - `UserError::Database` si hay un error de base de datos
pub async fn list_users(pool: &PgPool, limit: i64, offset: i64) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        FROM users
        WHERE is_active = true
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
        "#,
        limit,
        offset
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

/// Actualiza un usuario existente
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
pub async fn update_user(pool: &PgPool, id: i32, update: UpdateUser) -> Result<User> {
    // Primero verificamos que el usuario existe
    let mut user = find_user_by_id(pool, id).await?;

    // Actualizamos solo los campos que vienen en el update
    if let Some(username) = update.username {
        user.username = username;
    }
    if let Some(email) = update.email {
        user.email = email;
    }
    if let Some(password_hash) = update.password {
        user.password_hash = password_hash;
    }
    if let Some(role) = update.role {
        user.role = role;
    }
    if let Some(is_active) = update.is_active {
        user.is_active = is_active;
    }

    // Ejecutamos el update
    let updated = sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET
            username = $2,
            email = $3,
            password_hash = $4,
            role = $5,
            is_active = $6
        WHERE id = $1
        RETURNING
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        id,
        user.username,
        user.email,
        user.password_hash,
        user.role as UserRole,
        user.is_active
    )
    .fetch_one(pool)
    .await?;

    Ok(updated)
}

/// Desactiva un usuario (soft delete)
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
pub async fn deactivate_user(pool: &PgPool, id: i32) -> Result<User> {
    sqlx::query_as!(
        User,
        r#"
        UPDATE users
        SET is_active = false
        WHERE id = $1
        RETURNING
            id,
            username,
            email,
            password_hash,
            role as "role: UserRole",
            is_active as "is_active!",
            created_at as "created_at!",
            updated_at as "updated_at!"
        "#,
        id
    )
    .fetch_optional(pool)
    .await?
    .ok_or(UserError::NotFound)
}

/// Elimina permanentemente un usuario (hard delete) - solo para testing
///
/// # Errors
/// - `UserError::NotFound` si el usuario no existe
/// - `UserError::Database` si hay un error de base de datos
#[cfg(test)]
pub async fn delete_user(pool: &PgPool, id: i32) -> Result<()> {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        Err(UserError::NotFound)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:password@localhost/lumastack_test".to_string());

        PgPool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    #[ignore]
    async fn test_create_and_find_user() {
        let pool = setup_test_pool().await;

        let new_user = CreateUser {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
            role: UserRole::User,
        };

        let user = create_user(&pool, new_user, "hashed_password".to_string())
            .await
            .unwrap();

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");

        let found = find_user_by_id(&pool, user.id).await.unwrap();
        assert_eq!(found.id, user.id);

        // Cleanup
        delete_user(&pool, user.id).await.unwrap();
    }
}
