use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Rol de usuario en el sistema
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "admin")]
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

/// Modelo completo de usuario (con datos sensibles)
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)] // Nunca serializar el hash de password
    pub password_hash: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

/// Usuario sin datos sensibles (para respuestas API)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: UserRole,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role,
            is_active: user.is_active,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Datos para crear un nuevo usuario
#[derive(Debug, Clone, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub role: UserRole,
}

/// Datos para actualizar un usuario
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateUser {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<UserRole>,
    pub is_active: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_role_default() {
        assert_eq!(UserRole::default(), UserRole::User);
    }

    #[test]
    fn test_user_role_display() {
        assert_eq!(UserRole::User.to_string(), "user");
        assert_eq!(UserRole::Admin.to_string(), "admin");
    }

    #[test]
    fn test_user_to_response() {
        use chrono::Utc;

        let now = Utc::now().naive_utc();

        let user = User {
            id: 1,
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "secret_hash".to_string(),
            role: UserRole::User,
            is_active: true,
            created_at: now,
            updated_at: now,
        };

        let response: UserResponse = user.clone().into();
        assert_eq!(response.id, user.id);
        assert_eq!(response.username, user.username);
        assert_eq!(response.email, user.email);
    }
}
