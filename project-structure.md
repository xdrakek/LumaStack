# Estructura del Proyecto LumaStack

Esta es la estructura de carpetas recomendada para el proyecto LumaStack:

```
LumaStack/
├── README.md
├── ARCHITECTURE.md
├── CONTRIBUTING.md
├── LICENSE
├── .gitignore
├── docker-compose.yml
├── .env.example
├── frontend/
│   ├── src/
│   │   ├── components/
│   │   │   ├── ui/
│   │   │   ├── forms/
│   │   │   ├── layout/
│   │   │   └── features/
│   │   │       ├── auth/
│   │   │       ├── projects/
│   │   │       ├── scripts/
│   │   │       └── notifications/
│   │   ├── composables/
│   │   ├── stores/
│   │   │   ├── auth.ts
│   │   │   ├── projects.ts
│   │   │   └── notifications.ts
│   │   ├── views/
│   │   │   ├── Dashboard.vue
│   │   │   ├── Projects.vue
│   │   │   ├── Scripts.vue
│   │   │   └── Settings.vue
│   │   ├── router/
│   │   ├── services/
│   │   │   ├── api.ts
│   │   │   ├── auth.ts
│   │   │   └── websocket.ts
│   │   ├── utils/
│   │   ├── assets/
│   │   ├── styles/
│   │   └── types/
│   ├── tests/
│   │   ├── unit/
│   │   ├── integration/
│   │   └── e2e/
│   ├── public/
│   ├── package.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── tsconfig.json
│   └── vitest.config.ts
├── backend/
│   ├── src/
│   │   ├── main.rs
│   │   ├── lib.rs
│   │   ├── config/
│   │   │   ├── mod.rs
│   │   │   └── database.rs
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   ├── user.rs
│   │   │   ├── project.rs
│   │   │   ├── script.rs
│   │   │   └── notification.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   ├── projects.rs
│   │   │   ├── scripts.rs
│   │   │   └── notifications.rs
│   │   ├── services/
│   │   │   ├── mod.rs
│   │   │   ├── auth_service.rs
│   │   │   ├── git_service.rs
│   │   │   ├── telegram_service.rs
│   │   │   └── script_service.rs
│   │   ├── middleware/
│   │   │   ├── mod.rs
│   │   │   ├── auth.rs
│   │   │   └── cors.rs
│   │   ├── utils/
│   │   │   ├── mod.rs
│   │   │   ├── crypto.rs
│   │   │   └── validators.rs
│   │   └── db/
│   │       ├── mod.rs
│   │       └── migrations/
│   ├── tests/
│   │   ├── integration/
│   │   └── unit/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── .env.example
├── database/
│   ├── migrations/
│   ├── seeds/
│   └── schema.sql
├── scripts/
│   ├── setup.sh
│   ├── build.sh
│   ├── deploy.sh
│   └── test.sh

```

## Descripción de Carpetas Principales

- **frontend/**: Aplicación Vue.js 3 (script setup), Tailwind CSS y shadcn/vue
- **backend/**: API REST en Rust con estructura modular y servicios separados
- **database/**: Migraciones, seeds y esquemas de PostgreSQL
- **scripts/**: Scripts de automatización para setup, build y deploy


## Características de la Estructura

- **Separación clara** entre frontend y backend
- **Organización modular** para escalabilidad
- **Testing** integrado en cada capa

