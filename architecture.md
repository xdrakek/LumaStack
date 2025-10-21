# Arquitectura del Sistema LumaStack

## Resumen Ejecutivo

LumaStack es una plataforma web moderna diseñada para el monitoreo y gestión de repositorios Git con integración avanzada de Telegram para notificaciones. La arquitectura sigue un enfoque pragmático: **monolito modular** para MVP, con capacidad de evolucionar hacia microservicios si la escala lo requiere.

## Visión General de la Arquitectura

### Principios de Diseño

1. **Separación de Responsabilidades**: Frontend y backend completamente desacoplados
2. **Monolito Modular**: Servicios cohesivos en una sola aplicación (más fácil de deployar y debuggear)
3. **API First**: Diseño centrado en APIs RESTful
4. **Pragmatismo sobre Perfección**: Empezar simple, evolucionar según necesidad real
5. **Seguridad desde el Diseño**: Implementación de mejores prácticas de seguridad desde el MVP

### Stack Tecnológico

**Frontend**:
- Vue.js 3 con Composition API (script setup)
- Tailwind CSS para styling
- shadcn/vue para componentes de UI
- Pinia para manejo de estado
- Vite como bundler

**Backend**:
- Rust con framework Axum
- PostgreSQL como base de datos principal
- JWT para autenticación
- WebSockets para comunicación en tiempo real


## Diagrama de Arquitectura Simplificada (MVP)

```
┌─────────────────────────────────────────────────────────┐
│            FRONTEND (Vue 3 SPA)                         │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Components (shadcn/vue + Tailwind)              │   │
│  │  - Auth (Login, Register)                        │   │
│  │  - Projects (List, Detail, FileExplorer)         │   │
│  │  - Dashboard                                     │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Pinia Stores                                    │   │
│  │  - authStore (user, token)                       │   │
│  │  - projectsStore (projects, commits)             │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Services (Axios)                                │   │
│  │  - api.ts (base config)                          │   │
│  │  - authService.ts                                │   │
│  │  - projectsService.ts                            │   │
│  └──────────────────────────────────────────────────┘   │
└───────────────────┬─────────────────────────────────────┘
                    │ HTTP/REST (JSON)
                    │ Authorization: Bearer <JWT>
┌───────────────────▼─────────────────────────────────────┐
│         BACKEND (Rust + Axum)                           │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Router (main.rs)                                │   │
│  │  ├─ /api/auth/*      → auth handlers            │   │
│  │  ├─ /api/projects/*  → project handlers         │   │
│  │  └─ Middleware: JWT auth, CORS, logging         │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Handlers Layer (handlers/)                      │   │
│  │  - auth.rs (register, login)                     │   │
│  │  - projects.rs (list, get, files, commits)       │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Services Layer (services/)                      │   │
│  │  - AuthService (JWT, bcrypt)                     │   │
│  │  - GitService (git2-rs: repos, commits, files)   │   │
│  │  - ProjectService (business logic)               │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Repository Layer (repositories/)                │   │
│  │  - UserRepository (DB queries)                   │   │
│  │  - ProjectRepository (DB queries)                │   │
│  │  - Uses sqlx with compile-time checked queries   │   │
│  └──────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────┐   │
│  │  Models (models/)                                │   │
│  │  - User, Project, Commit, ProjectMember          │   │
│  └──────────────────────────────────────────────────┘   │
└───────────────────┬─────────────────────────────────────┘
                    │ sqlx (async)
┌───────────────────▼─────────────────────────────────────┐
│            PostgreSQL 18                                │
│  Tables:                                                │
│  - users (id, username, email, password_hash, role)     │
│  - projects (id, name, repository_path, description)    │
│  - project_members (project_id, user_id, role)          │
│  - commits (id, project_id, hash, author, message)      │
└─────────────────────────────────────────────────────────┘
```

## Arquitectura de Componentes

### 1. Capa de Presentación (Frontend)

#### Estructura de Directorios

```
frontend/
├── src/
│   ├── components/
│   │   ├── ui/              # Componentes shadcn/vue base
│   │   │   ├── Button.vue
│   │   │   ├── Input.vue
│   │   │   └── Card.vue
│   │   ├── layout/          # Layout components
│   │   │   ├── AppHeader.vue
│   │   │   ├── AppSidebar.vue
│   │   │   └── MainLayout.vue
│   │   └── features/        # Feature-specific components
│   │       ├── auth/
│   │       │   ├── LoginForm.vue
│   │       │   └── RegisterForm.vue
│   │       ├── projects/
│   │       │   ├── ProjectList.vue
│   │       │   ├── ProjectCard.vue
│   │       │   ├── CommitList.vue
│   │       │   └── FileExplorer.vue
│   │       └── dashboard/
│   │           └── ActivityFeed.vue
│   ├── composables/         # Reusable logic
│   │   ├── useAuth.ts
│   │   ├── useApi.ts
│   │   └── useNotification.ts
│   ├── stores/              # Pinia stores
│   │   ├── auth.ts
│   │   └── projects.ts
│   ├── services/            # API clients
│   │   ├── api.ts           # Axios base config
│   │   ├── authService.ts
│   │   └── projectsService.ts
│   ├── router/
│   │   └── index.ts         # Vue Router config
│   ├── views/               # Page components
│   │   ├── LoginView.vue
│   │   ├── DashboardView.vue
│   │   ├── ProjectsView.vue
│   │   └── ProjectDetailView.vue
│   ├── types/               # TypeScript types
│   │   └── models.ts
│   ├── utils/               # Utility functions
│   │   └── validators.ts
│   ├── App.vue
│   └── main.ts
├── public/
├── package.json
├── vite.config.ts
├── tailwind.config.js
└── tsconfig.json
```

#### Patrones de Diseño Implementados

- **Composition API con `<script setup>`**: Para mejor organización y reutilización de lógica
- **Feature-based Components**: Organización por funcionalidad en `features/`
- **Atomic Design Light**: ui/ (atoms) → features/ (molecules/organisms) → views/ (pages)
- **Composables para Lógica Compartida**: useAuth, useApi, etc.

### 2. Capa de Aplicación (Backend)

#### Estructura de Directorios

```
backend/
├── src/
│   ├── main.rs                    # Entry point, router setup
│   ├── lib.rs                     # Library exports
│   ├── config/
│   │   ├── mod.rs
│   │   └── database.rs            # DB connection pool
│   ├── models/                    # Data structures
│   │   ├── mod.rs
│   │   ├── user.rs                # User, NewUser, UserRole
│   │   ├── project.rs             # Project, NewProject
│   │   ├── project_member.rs      # ProjectMember, MemberRole
│   │   └── commit.rs              # Commit (cached from Git)
│   ├── handlers/                  # HTTP request handlers
│   │   ├── mod.rs
│   │   ├── auth.rs                # POST /register, /login
│   │   └── projects.rs            # GET /projects, /projects/:id, etc.
│   ├── services/                  # Business logic
│   │   ├── mod.rs
│   │   ├── auth_service.rs        # JWT generation, password validation
│   │   ├── git_service.rs         # Git operations (git2 crate)
│   │   └── project_service.rs     # Project business logic
│   ├── repositories/              # Database access layer
│   │   ├── mod.rs
│   │   ├── user_repository.rs     # User CRUD with sqlx
│   │   ├── project_repository.rs
│   │   └── commit_repository.rs
│   ├── middleware/
│   │   ├── mod.rs
│   │   ├── auth.rs                # JWT validation middleware
│   │   └── cors.rs                # CORS configuration
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── error.rs               # Custom error types
│   │   └── validators.rs          # Input validation
│   └── db/
│       └── migrations/            # sqlx migrations
│           ├── 20250101000001_create_users.sql
│           ├── 20250101000002_create_projects.sql
│           └── ...
├── tests/
│   ├── integration/               # Integration tests
│   │   ├── auth_tests.rs
│   │   └── projects_tests.rs
│   └── common/
│       └── mod.rs                 # Test utilities
├── Cargo.toml
├── Cargo.lock
├── .env.example
└── sqlx-data.json                 # sqlx offline mode cache
```

#### Servicios Principales (MVP)

1. **AuthService**:
   - Generación y validación de JWT tokens
   - Hash y verificación de passwords con bcrypt
   - Validación de roles

2. **GitService**:
   - Escaneo de directorios para repositorios Git
   - Lectura de commits con git2-rs
   - Navegación de árbol de archivos
   - Lectura de contenido de archivos

3. **ProjectService**:
   - Lógica de negocio de proyectos
   - Verificación de permisos por proyecto
   - Coordinación entre GitService y ProjectRepository

#### Servicios Futuros (Post-MVP)

4. **TelegramService** (Fase 3): Integración con Bot API de Telegram
5. **ScriptService** (Fase 4): Ejecución y gestión de scripts
6. **NotificationService** (Fase 2): Gestión de notificaciones

### 3. Capa de Datos

#### Base de Datos Principal (PostgreSQL)

**Esquema Mejorado con Índices** (ver `database/schema.sql` para versión completa):

```sql
-- ============================================
-- FASE 1 - MVP TABLES
-- ============================================

-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user', -- 'user' | 'admin'
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = TRUE;

-- Projects table
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    repository_path VARCHAR(500) NOT NULL UNIQUE,
    description TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    last_scanned_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_projects_path ON projects(repository_path);
CREATE INDEX idx_projects_public ON projects(is_public) WHERE is_public = TRUE;

-- Project members (many-to-many: users <-> projects)
CREATE TABLE project_members (
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'viewer', -- 'viewer' | 'contributor' | 'admin'
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (project_id, user_id)
);

CREATE INDEX idx_project_members_user ON project_members(user_id);
CREATE INDEX idx_project_members_project ON project_members(project_id);

-- Commits cache (performance optimization)
CREATE TABLE commits (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    commit_hash VARCHAR(40) UNIQUE NOT NULL,
    author_name VARCHAR(255) NOT NULL,
    author_email VARCHAR(255),
    message TEXT NOT NULL,
    committed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_commits_project_date ON commits(project_id, committed_at DESC);
CREATE INDEX idx_commits_hash ON commits(commit_hash);

-- ============================================
-- FASE 2 - COLLABORATION TABLES
-- ============================================

-- Comments (added in Phase 2)
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    commit_hash VARCHAR(40),
    content TEXT NOT NULL,
    parent_comment_id INTEGER REFERENCES comments(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_comments_project ON comments(project_id, created_at DESC);
CREATE INDEX idx_comments_parent ON comments(parent_comment_id) WHERE parent_comment_id IS NOT NULL;

-- Notifications (added in Phase 2)
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- 'commit' | 'comment' | 'mention'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    link VARCHAR(500),
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_notifications_user_unread ON notifications(user_id, is_read, created_at DESC);

-- ============================================
-- FASE 3 - TELEGRAM INTEGRATION
-- ============================================

-- Add telegram_chat_id to users (migration in Phase 3)
ALTER TABLE users ADD COLUMN telegram_chat_id BIGINT UNIQUE;
CREATE INDEX idx_users_telegram ON users(telegram_chat_id) WHERE telegram_chat_id IS NOT NULL;

-- ============================================
-- FASE 4 - SCRIPTS SYSTEM
-- ============================================

-- Scripts table (added in Phase 4)
CREATE TABLE scripts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    content TEXT NOT NULL,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE, -- NULL for global scripts
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_scripts_project ON scripts(project_id) WHERE project_id IS NOT NULL;
CREATE INDEX idx_scripts_global ON scripts(project_id) WHERE project_id IS NULL;

-- Script executions log
CREATE TABLE script_executions (
    id SERIAL PRIMARY KEY,
    script_id INTEGER REFERENCES scripts(id) ON DELETE CASCADE,
    executed_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    status VARCHAR(20) NOT NULL, -- 'running' | 'success' | 'failed' | 'timeout'
    exit_code INTEGER,
    output TEXT,
    started_at TIMESTAMP NOT NULL,
    finished_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX idx_script_executions_script ON script_executions(script_id, started_at DESC);
CREATE INDEX idx_script_executions_user ON script_executions(executed_by, started_at DESC);
```

#### Estrategia de Migraciones

- **Fase 1 (MVP)**: Tables `users`, `projects`, `project_members`, `commits`
- **Fase 2**: Add `comments`, `notifications`
- **Fase 3**: Alter `users` to add `telegram_chat_id`
- **Fase 4**: Add `scripts`, `script_executions`

Todas las migraciones son reversibles (up/down scripts) usando sqlx migrate.

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