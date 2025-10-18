# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

LumaStack is a modern web platform for monitoring and managing Git repositories with advanced Telegram integration. It provides visualization of project files, commit tracking, custom script execution, and intelligent notifications through Telegram.

**Current Status**: Documentation/planning phase - implementation has not yet begun. The project has comprehensive architecture documentation and requirements but no actual code implementation.

## Technology Stack

**Frontend**:
- Vue.js 3 with Composition API (using `<script setup>`) and TypeScript
- Tailwind CSS for styling
- shadcn/vue for UI components
- Pinia for state management
- Vite as build tool

**Backend**:
- Rust with Axum framework
- PostgreSQL 13+ as primary database
- JWT for authentication
- WebSockets for real-time communication

**External Integrations**:
- Telegram Bot API for notifications and interactive commands

## Architecture

### Three-Layer Architecture

1. **Presentation Layer (Frontend)**:
   - Feature-based organization under `frontend/src/`
   - Components organized by type: `ui/`, `forms/`, `layout/`, `features/`
   - Stores in `stores/` using Pinia (auth, projects, notifications)
   - API services in `services/` (api.ts, auth.ts, websocket.ts)
   - Reusable logic in `composables/`

2. **Application Layer (Backend)**:
   - Modular Rust structure under `backend/src/`
   - HTTP handlers in `handlers/` (auth, projects, scripts, notifications)
   - Business logic in `services/` (AuthService, GitService, TelegramService, ScriptService, NotificationService)
   - Data models in `models/`
   - Middleware for auth, CORS in `middleware/`
   - Database management in `db/` with migrations

3. **Data Layer**:
   - PostgreSQL with core tables: users, projects, scripts, notifications
   - Each table includes created_at/updated_at timestamps
   - Foreign key relationships enforced at DB level

### Key Service Responsibilities

- **AuthService**: User authentication (credentials + Telegram magic links), JWT token generation, role-based access control
- **GitService**: Repository scanning, commit history tracking, file navigation with syntax highlighting
- **TelegramService**: Bot integration, notification delivery, magic link generation, command handling
- **ScriptService**: Custom script definition, sandboxed execution with timeouts, logging
- **NotificationService**: Event-driven notifications, mention detection (@usuario), user subscription management

## Security

- **Authentication**: JWT tokens with 24-hour expiration, refresh token support
- **Password Hashing**: bcrypt with factor 12+
- **Authorization**: Role-based access control (User/Admin roles)
- **Rate Limiting**: 5 login attempts per minute, API endpoint protection
- **Input Validation**: Client and server-side validation, SQL injection prevention via prepared statements
- **HTTPS**: Required in production for all communications
- **Sensitive Data**: Telegram tokens encrypted at rest

## Core Features

1. **Authentication**:
   - Traditional username/password login
   - Telegram magic links (10-minute expiration, single-use)
   - Account linking with Telegram chat_id
   - Role management (User vs Admin permissions)

2. **Repository Management**:
   - Auto-detection of Git repositories from configured directories
   - Tree-view file navigation with syntax highlighting
   - Commit history display (last 10 commits with metadata)
   - Repository lock/unlock to prevent unauthorized pulls
   - Real-time pull detection

3. **Script Execution**:
   - Global scripts (admin-defined) and project-specific scripts
   - Sandboxed execution environment with 5-minute max timeout
   - Real-time execution logs
   - Execution history with filtering

4. **Notifications & Comments**:
   - Per-project notification subscriptions
   - Auto-notification on @mentions in comments
   - Telegram integration for commenting from chat
   - Markdown support in comments

5. **Dashboard**:
   - Last 7 days activity summary
   - Recent commits across projects
   - Recent script executions
   - Recent comments

## Development Phases

Per `roadmap.md`:

- **Fase 1 (MVP)**: Basic auth, roles, project scanning, pull tracking, basic dashboard, initial Telegram notifications
- **Fase 2 (Beta)**: Comments with mentions, pull blocking, basic scripts, notification configuration
- **Fase 3 (Production)**: Code preview, advanced dashboards, scheduled scripts with comprehensive logging
- **Fase 4 (Advanced)**: Multi-language support

## Performance & Scalability Requirements

- Page load time: < 2 seconds
- API response time: < 500ms
- Search completion: < 1 second
- Support for 100 concurrent users
- 1000 requests/minute throughput
- Horizontal scaling capability
- 99.9% uptime target

## Database Schema Notes

Key tables (see `architecture.md` for full schema):

- `users`: Includes telegram_user_id for linking, role enum (user/admin)
- `projects`: Includes is_blocked flag for pull control
- `scripts`: Can be global or project-specific (project_id nullable)
- `notifications`: Type-based (commit, comment, mention), includes is_read flag

Foreign keys enforce referential integrity. All timestamps use PostgreSQL TIMESTAMP with DEFAULT NOW().

## Important Context

- **Language**: Documentation is primarily in Spanish; code should follow English conventions for variables/functions
- **Testing**: Target > 80% code coverage for both frontend and backend
- **Code Style**:
  - Vue components use Composition API with `<script setup lang="ts">`
  - TypeScript strict mode enabled for type safety
  - Type-safe API clients and WebSocket message contracts
  - Rust follows idiomatic patterns
- **Real-time Updates**: Use WebSockets for live commit notifications and script execution status
- **Telegram Integration**: Respect Telegram API rate limits; implement retry with backoff
