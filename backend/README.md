# LumaStack Backend

Backend de LumaStack construido con Rust + Axum + PostgreSQL + SQLx.

## Requisitos

- Rust 1.75+ (instalar desde [rustup.rs](https://rustup.rs/))
- PostgreSQL 18 (instalado y corriendo)
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
- **sqlx** - Cliente PostgreSQL async con type-safety
- **tower** - Ecosystem de middleware
- **tower-http** - Middleware HTTP (CORS, tracing)
- **serde** - Serialización/deserialización
- **tracing** - Logging estructurado
- **chrono** - Manejo de fechas y timestamps
- **thiserror** - Manejo de errores personalizado

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

## Configuración de PostgreSQL 18

### Instalación con Docker (Recomendado)

```bash
docker run -d \
  --name postgresql \
  -p 5432:5432 \
  -e POSTGRES_PASSWORD=password \
  -e POSTGRES_USER=admin \
  -v lumastack-db-data:/var/lib/postgresql/data \
  postgres:18
```

### Crear base de datos

```bash
# Conectarse al contenedor
docker exec -it postgresql psql -U admin

# Crear usuario y base de datos
CREATE USER lumastack WITH PASSWORD 'password';
CREATE DATABASE lumastack OWNER lumastack;
GRANT ALL PRIVILEGES ON DATABASE lumastack TO lumastack;
\q
```

## Variables de Entorno

Crea un archivo `.env` en el directorio `backend/` (copia desde `.env.example`):

```bash
cp .env.example .env
```

Contenido del `.env`:

```env
# Servidor
PORT=3000
HOST=0.0.0.0
RUST_LOG=lumastack_backend=debug,tower_http=debug

# Base de datos PostgreSQL 18
DATABASE_URL=postgresql://lumastack:password@localhost/lumastack

# Configuración del pool de conexiones (opcional, con valores por defecto sensibles)
# DB_MIN_CONNECTIONS=1           # Mínimo de conexiones (default: 1)
# DB_MAX_CONNECTIONS=10          # Máximo de conexiones (default: 10, recomendado 20-50 para producción)
# DB_ACQUIRE_TIMEOUT=30          # Timeout para adquirir conexión en segundos (default: 30)
# DB_IDLE_TIMEOUT=600            # Timeout de inactividad en segundos (default: 600)

# JWT
JWT_SECRET=your-secret-key-here-change-in-production
JWT_EXPIRATION=86400

# Telegram Bot
TELEGRAM_BOT_TOKEN=your-telegram-bot-token-here
```

### Recomendaciones para Producción

**Pool de Conexiones:**
- Para cargas moderadas (10-50 usuarios concurrentes): `DB_MAX_CONNECTIONS=20`
- Para cargas altas (50-100 usuarios concurrentes): `DB_MAX_CONNECTIONS=50`
- Para cargas muy altas (100+ usuarios): `DB_MAX_CONNECTIONS=100`
- La regla general: `max_connections = (núcleos_cpu × 2) + número_de_discos`

## Estado Actual

**Version:** 0.1.0 (MVP en desarrollo)

### ✅ Implementado

- ✅ Servidor HTTP con Axum
- ✅ Estructura modular (handlers, models, db, services)
- ✅ Sistema de logging con tracing
- ✅ **Integración completa con PostgreSQL 18 + SQLx**
  - Pool de conexiones configurado
  - Sistema de migraciones automáticas
  - Modelo User con roles (user/admin)
  - Queries CRUD type-safe para usuarios
  - Manejo de errores personalizado
- ✅ Endpoints operacionales:
  - `GET /` - Información de la API
  - `GET /health` - Health check con verificación de BD

### 🔜 Próximos pasos

- Implementar bcrypt para hash de passwords
- Crear handlers de autenticación (register, login)
- Implementar JWT authentication
- Agregar endpoints REST para gestión de usuarios
- Middleware de autenticación
- Configurar CORS para frontend

## Recursos

- [Documentación de Axum](https://docs.rs/axum/latest/axum/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust Book](https://doc.rust-lang.org/book/)
