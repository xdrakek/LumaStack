# Code Review Checklist para LumaStack

Este checklist debe ser usado al revisar código nuevo o cambios significativos en el proyecto.

## 🔒 Seguridad

### Autenticación & Autorización
- [ ] Rutas protegidas requieren autenticación (JWT middleware)
- [ ] Verificación de permisos antes de operaciones sensibles
- [ ] No hay hardcoded credentials, API keys o secrets
- [ ] Tokens y secrets se leen desde variables de entorno

### Input Validation
- [ ] Todos los inputs de usuario son validados (backend)
- [ ] Validación de tipos y formatos (email, username, etc.)
- [ ] Sanitización de inputs para prevenir XSS
- [ ] Queries usan prepared statements o sqlx (no SQL injection)
- [ ] Path traversal prevenido en operaciones de archivos

### Datos Sensibles
- [ ] Passwords hasheados con bcrypt (factor 12+)
- [ ] No se loggean datos sensibles (passwords, tokens)
- [ ] Tokens de API encriptados en base de datos
- [ ] CORS configurado correctamente (no `*` en producción)

---

## ⚡ Performance

### Database
- [ ] Queries usan índices existentes
- [ ] No hay problemas N+1 (usar JOINs o batch loading)
- [ ] Connection pooling configurado apropiadamente
- [ ] Transactions usadas donde es necesario
- [ ] Queries pesados usan paginación

### Backend (Rust)
- [ ] Funciones async donde sea apropiado
- [ ] No hay blocking calls en handlers async
- [ ] Clone minimizado (usar referencias cuando sea posible)
- [ ] Allocations innecesarias evitadas
- [ ] Error handling con `?` operator (no `unwrap()` en producción)

### Frontend (Vue)
- [ ] Computed properties para datos derivados (no re-cálculos en templates)
- [ ] Componentes reactivos optimizados (evitar re-renders innecesarios)
- [ ] Lazy loading para rutas y componentes grandes
- [ ] Debounce/throttle en inputs de búsqueda
- [ ] Keys únicas en v-for loops

---

## 🧪 Testing

### Cobertura
- [ ] Tests unitarios para lógica de negocio crítica
- [ ] Tests de integración para endpoints importantes
- [ ] Tests de validación de inputs (casos edge)
- [ ] Tests de autorización (usuarios sin permisos)
- [ ] Coverage > 60% para código nuevo

### Calidad de Tests
- [ ] Tests son determinísticos (no dependen de timing)
- [ ] Tests no dependen de orden de ejecución
- [ ] Mocks apropiados para servicios externos
- [ ] Setup/teardown limpia estado entre tests
- [ ] Nombres de tests descriptivos

---

## 📝 Code Quality

### Rust
- [ ] Código sigue convenciones Rust (snake_case funciones, PascalCase tipos)
- [ ] Funciones pequeñas y enfocadas (< 50 líneas idealmente)
- [ ] Documentación para funciones públicas (`///`)
- [ ] Error types específicos (no `Box<dyn Error>` en APIs públicas)
- [ ] Warnings de Clippy resueltos
- [ ] `cargo fmt` ejecutado

### TypeScript/Vue
- [ ] Tipos definidos (no `any` sin justificación)
- [ ] Componentes pequeños y reutilizables (< 200 líneas)
- [ ] Props tipadas correctamente
- [ ] Composition API con `<script setup>` usado consistentemente
- [ ] ESLint warnings resueltos

### General
- [ ] Nombres descriptivos (variables, funciones, tipos)
- [ ] Sin código comentado (usar git para historial)
- [ ] Sin console.log en producción (usar logging apropiado)
- [ ] Imports organizados y sin duplicados
- [ ] Código DRY (no duplicación innecesaria)

---

## 🗄️ Database

### Migrations
- [ ] Migration tiene script de rollback (down.sql)
- [ ] Migration es idempotente (puede ejecutarse múltiples veces)
- [ ] Foreign keys con ON DELETE/UPDATE apropiados
- [ ] Índices añadidos para nuevas columnas donde sea necesario
- [ ] Nombres de columnas consistentes con schema existente

### Queries
- [ ] Queries eficientes (EXPLAIN ANALYZE verificado)
- [ ] Paginación implementada para listados grandes
- [ ] Filtros usan índices existentes
- [ ] Transacciones para operaciones multi-tabla
- [ ] Prepared statements (sqlx compile-time checking)

---

## 🎨 Frontend Específico

### Componentes Vue
- [ ] Props con valores default apropiados
- [ ] Emits declarados explícitamente
- [ ] Reactivity correcta (ref/reactive usado apropiadamente)
- [ ] Lifecycle hooks necesarios (onMounted, onUnmounted)
- [ ] Cleanup en onUnmounted (event listeners, timers)

### Estado (Pinia)
- [ ] Estado global solo para datos compartidos
- [ ] Actions para lógica async
- [ ] Getters para datos derivados
- [ ] State mutations solo en actions

### UX
- [ ] Loading states para operaciones async
- [ ] Error messages descriptivos para usuarios
- [ ] Validación de formularios con feedback visual
- [ ] Confirmación para acciones destructivas
- [ ] Responsive design (mobile, tablet, desktop)

---

## 🔧 Backend Específico

### API Design
- [ ] Endpoints RESTful consistentes
- [ ] Status codes HTTP apropiados (200, 201, 400, 401, 404, 500)
- [ ] Responses con formato consistente (JSON)
- [ ] Errores con mensajes descriptivos
- [ ] Versionado de API considerado (si aplica)

### Logging
- [ ] Logs estructurados con contexto (user_id, request_id)
- [ ] Niveles apropiados (error, warn, info, debug)
- [ ] No se loggean datos sensibles
- [ ] Logs útiles para debugging (no spam)

### Error Handling
- [ ] Errores manejados apropiadamente (no panic! en producción)
- [ ] Errores convertidos a responses HTTP apropiados
- [ ] Errores de terceros wrapeados
- [ ] Stack traces solo en development

---

## 📚 Documentación

- [ ] README actualizado si hay cambios en setup
- [ ] API docs actualizadas (si hay nuevos endpoints)
- [ ] Comentarios para lógica compleja
- [ ] CHANGELOG actualizado (si aplica)
- [ ] Migration docs si hay cambios de schema

---

## ✅ Pre-Commit

Antes de hacer commit, verifica:

```bash
# Backend
cd backend
cargo fmt --check
cargo clippy -- -D warnings
cargo test

# Frontend
cd frontend
npm run lint
npm run type-check
npm run test:unit
```

---

## 🚀 Pre-Deploy

Antes de deploy a producción:

- [ ] Todos los tests pasan (CI/CD green)
- [ ] No hay secrets hardcodeados
- [ ] Environment variables configuradas
- [ ] Database migrations probadas
- [ ] Rollback plan documentado
- [ ] Monitoreo configurado para nuevas features

---

## Notas

- **Pragmatismo sobre pureza**: No todos los items aplican a todos los PRs
- **Prioriza seguridad**: Security issues son bloqueantes
- **Tests importantes**: Coverage no es todo, calidad de tests importa
- **Performance después**: Optimiza solo si hay problema real medido

## Uso

Al revisar código, copia este checklist y marca los items relevantes.
No todos aplican a todos los cambios - usa juicio y prioriza items críticos.
