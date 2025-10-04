# Arquitectura del Sistema LumaStack

## Resumen Ejecutivo

LumaStack es una plataforma web moderna diseñada para el monitoreo y gestión de repositorios Git con integración avanzada de Telegram para notificaciones. La arquitectura sigue principios de diseño moderno con separación clara de responsabilidades, escalabilidad horizontal y alta disponibilidad.

## Visión General de la Arquitectura

### Principios de Diseño

1. **Separación de Responsabilidades**: Frontend y backend completamente desacoplados
2. **Microservicios**: Servicios independientes para diferentes funcionalidades
3. **API First**: Diseño centrado en APIs RESTful
4. **Escalabilidad**: Arquitectura preparada para crecimiento horizontal
5. **Seguridad**: Implementación de mejores prácticas de seguridad desde el diseño

### Stack Tecnológico

**Frontend**:
- Vue.js 3 con Composition API (script setup)
- Tailwind CSS para styling
- shadcn/vue para componentes de UI
- Pinia para manejo de estado
- Vite como bundler

**Backend**:
- Rust con framework Axum/Actix-web
- PostgreSQL como base de datos principal
- JWT para autenticación
- WebSockets para comunicación en tiempo real


## Arquitectura de Componentes

### 1. Capa de Presentación (Frontend)

#### Estructura de Componentes

```
src/
├── components/
│   ├── ui/           # Componentes base de UI
│   ├── forms/        # Componentes de formularios
│   ├── layout/       # Componentes de layout
│   └── features/     # Componentes específicos por feature
├── stores/           # Gestión de estado con Pinia
├── services/         # Servicios para API calls
├── composables/      # Lógica reutilizable
└── utils/           # Utilidades y helpers
```

#### Patrones de Diseño Implementados

- **Composition API**: Para mejor organización y reutilización de lógica
- **Feature-based Architecture**: Organización por funcionalidad
- **Atomic Design**: Componentes desde básicos hasta complejos
- **Observer Pattern**: Para reactividad y estado global

### 2. Capa de Aplicación (Backend)

#### Estructura de Servicios

```rust
src/
├── handlers/         # Controladores HTTP
├── services/         # Lógica de negocio
├── models/          # Modelos de datos
├── middleware/      # Middleware personalizado
├── config/          # Configuración de la aplicación
└── utils/           # Utilidades y helpers
```

#### Servicios Principales

1. **AuthService**: Autenticación y autorización
2. **GitService**: Integración con repositorios Git
3. **TelegramService**: Integración con Bot API de Telegram
4. **ScriptService**: Ejecución y gestión de scripts
5. **NotificationService**: Gestión de notificaciones

### 3. Capa de Datos

#### Base de Datos Principal (PostgreSQL)

**Esquema Principal**:

```sql
-- Usuarios
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    telegram_user_id BIGINT UNIQUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Proyectos
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    path VARCHAR(500) NOT NULL,
    description TEXT,
    is_blocked BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);


-- Scripts
CREATE TABLE scripts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    content TEXT NOT NULL,
    is_global BOOLEAN DEFAULT FALSE,
    project_id INTEGER REFERENCES projects(id),
    created_by INTEGER REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW()
);

-- Notificaciones
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    project_id INTEGER REFERENCES projects(id),
    type notification_type NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);
```

## Integración de APIs Externas
### Telegram Bot API

**Funcionalidades**:
- Envío de notificaciones
- Autenticación con enlaces mágicos
- Gestión de comandos de bot
- Respuestas a menciones

**Implementación**:
```rust
pub struct TelegramService {
    bot: Bot,
    webhook_url: String,
}

impl TelegramService {
    pub async fn send_notification(&self, chat_id: i64, message: &str) -> Result<()> {
        // Envío de mensajes
    }
    
    pub async fn create_magic_link(&self, user_id: i32) -> Result<String> {
        // Generación de enlaces mágicos
    }
}
```

## Seguridad

### Autenticación y Autorización

1. **JWT Tokens**: Para sesiones stateless
2. **Role-based Access Control**: Usuarios y administradores
3. **Rate Limiting**: Para prevenir abuso de APIs
4. **CORS Configuration**: Para restricción de dominios

### Encriptación

- **Passwords**: Hashing con bcrypt
- **Datos sensibles**: Encriptación AES-256
- **Comunicación**: HTTPS obligatorio en producción

### Validación de Entrada

- **Sanitización**: Todos los inputs del usuario
- **Validación de esquemas**: Con bibliotecas de validación
- **SQL Injection Prevention**: Uso de prepared statements

## Monitoreo y Observabilidad

### Logging

```rust
use tracing::{info, warn, error, debug};

// Structured logging example
info!(
    user_id = user.id,
    action = "login",
    ip = request.ip(),
    "User logged in successfully"
);
```



## Consideraciones de Rendimiento

### Frontend

- **Code Splitting**: Carga bajo demanda
- **Tree Shaking**: Eliminación de código no usado
- **Asset Optimization**: Compresión de imágenes y recursos
- **Service Worker**: Para caching offline

### Backend

- **Connection Pooling**: Gestión eficiente de conexiones DB
- **Query Batching**: Reducción de consultas N+1
- **Background Jobs**: Para tareas pesadas
- **Circuit Breaker**: Para APIs externas

## Conclusión

La arquitectura de LumaStack está diseñada para ser robusta, escalable y mantenible. Con una separación clara de responsabilidades, integración moderna de APIs y prácticas de seguridad de última generación, la plataforma está preparada para crecer y evolucionar con las necesidades del negocio.

La elección de tecnologías modernas como Vue 3, Rust y PostgreSQL garantiza un rendimiento óptimo y una experiencia de desarrollo superior, mientras que la estructura modular permite un mantenimiento eficiente y actualizaciones incrementales.