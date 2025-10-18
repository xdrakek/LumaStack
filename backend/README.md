# LumaStack Backend

Backend de LumaStack construido con Rust y Axum.

## Requisitos

- Rust 1.70+ (instalar desde [rustup.rs](https://rustup.rs/))
- Cargo (incluido con Rust)

## Estructura del Proyecto

```
backend/
├── src/
│   ├── handlers/      # Controladores HTTP (auth, projects, scripts, etc.)
│   ├── services/      # Lógica de negocio (AuthService, GitService, etc.)
│   ├── models/        # Modelos de datos y DTOs
│   ├── middleware/    # Middleware personalizado (auth, CORS, etc.)
│   ├── db/            # Configuración y gestión de base de datos
│   └── main.rs        # Punto de entrada de la aplicación
├── db/
│   └── migrations/    # Migraciones SQL de PostgreSQL
└── Cargo.toml         # Configuración y dependencias
```

## Dependencias Principales

- **axum** - Framework web moderno y ergonómico
- **tokio** - Runtime asíncrono
- **tower** - Ecosystem de middleware
- **tower-http** - Middleware HTTP (CORS, tracing)
- **serde** - Serialización/deserialización
- **tracing** - Logging estructurado

## Ejecutar el Proyecto

### Desarrollo

```bash
# Compilar y ejecutar en modo desarrollo
cargo run

# El servidor estará disponible en http://localhost:3000
```

### Compilación

```bash
# Compilar en modo release (optimizado)
cargo build --release

# El binario estará en target/release/lumastack-backend
```

### Testing

```bash
# Ejecutar tests
cargo test

# Ejecutar tests con output detallado
cargo test -- --nocapture
```

### Verificación de Código

```bash
# Verificar formato
cargo fmt --check

# Aplicar formato
cargo fmt

# Ejecutar clippy (linter)
cargo clippy -- -D warnings
```

## Variables de Entorno

Crea un archivo `.env` en el directorio `backend/` con las siguientes variables:

```env
# Servidor
PORT=3000
HOST=0.0.0.0

# Base de datos (configurar en fases posteriores)
DATABASE_URL=postgresql://user:password@localhost/lumastack

# JWT (configurar en fases posteriores)
JWT_SECRET=your-secret-key-here
JWT_EXPIRATION=86400

# Telegram (configurar en fases posteriores)
TELEGRAM_BOT_TOKEN=your-telegram-bot-token
```

## Estado Actual

Version: 0.1.0 (MVP en desarrollo)

**Implementado:**
- Servidor HTTP básico con Axum
- Estructura modular de carpetas
- Sistema de logging con tracing
- Ruta raíz de prueba

**Próximos pasos:**
- Configurar conexión a PostgreSQL
- Implementar handlers de autenticación
- Crear modelos de datos
- Agregar middleware de autenticación JWT
- Configurar CORS para frontend

## Recursos

- [Documentación de Axum](https://docs.rs/axum/latest/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust Book](https://doc.rust-lang.org/book/)
