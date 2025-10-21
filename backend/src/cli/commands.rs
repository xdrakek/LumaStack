use sqlx::PgPool;
use std::io::{self, Write};

use crate::db::users::{create_user, find_user_by_email};
use crate::models::user::{CreateUser, UserRole};

/// Crea un usuario administrador
///
/// Si no se proporcionan email, username o password, los solicita interactivamente
pub async fn create_admin(
    pool: &PgPool,
    email: Option<String>,
    username: Option<String>,
    password: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Creando usuario administrador...\n");

    // Obtener email (argumento o input)
    let email = match email {
        Some(e) => e,
        None => {
            print!("Email del administrador: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Validar email
    if !email.contains('@') {
        return Err("Email inválido: debe contener '@'".into());
    }

    // Verificar si el usuario ya existe
    match find_user_by_email(pool, &email).await {
        Ok(_) => {
            println!("❌ Error: Ya existe un usuario con el email '{}'", email);
            return Err("Usuario ya existe".into());
        }
        Err(crate::db::users::UserError::NotFound) => {
            // Bien, no existe, podemos continuar
        }
        Err(e) => {
            return Err(Box::new(e));
        }
    }

    // Obtener username (argumento o input)
    let username = match username {
        Some(u) => u,
        None => {
            print!("Username del administrador: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            input.trim().to_string()
        }
    };

    // Validar username
    if username.len() < 3 {
        return Err("Username inválido: debe tener al menos 3 caracteres".into());
    }

    // Obtener password (argumento o input)
    let password = match password {
        Some(p) => p,
        None => {
            print!("Password del administrador: ");
            io::stdout().flush()?;
            let password = rpassword::read_password()?;

            print!("Confirmar password: ");
            io::stdout().flush()?;
            let password_confirm = rpassword::read_password()?;

            if password != password_confirm {
                return Err("Los passwords no coinciden".into());
            }

            password
        }
    };

    // Validar password
    if password.len() < 8 {
        return Err("Password inválido: debe tener al menos 8 caracteres".into());
    }

    // Hashear password con bcrypt
    println!("\n🔐 Hasheando password...");
    let password_hash = bcrypt::hash(&password, bcrypt::DEFAULT_COST)?;

    // Crear usuario
    println!("💾 Guardando usuario en base de datos...");
    let new_user = CreateUser {
        username,
        email: email.clone(),
        password: password.clone(),
        role: UserRole::Admin,
    };

    match create_user(pool, new_user, password_hash).await {
        Ok(user) => {
            println!("\n✅ Usuario administrador creado exitosamente!");
            println!("   ID: {}", user.id);
            println!("   Username: {}", user.username);
            println!("   Email: {}", user.email);
            println!("   Role: {:?}", user.role);
            println!("\n🎉 Ya puedes iniciar sesión con estas credenciales.");
            Ok(())
        }
        Err(e) => {
            println!("\n❌ Error al crear usuario: {}", e);
            Err(Box::new(e))
        }
    }
}
