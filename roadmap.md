# Roadmap de LumaStack

> **Filosofía**: Iteración rápida, MVP funcional primero, features complejas después

---

## 🎯 Fase 1 – MVP Core (2-3 semanas)

**Objetivo**: Sistema funcional básico con autenticación y visualización de repositorios Git

### Features Core
- ✅ **Autenticación con credenciales**
  - Registro de usuarios (email, username, password)
  - Login con JWT (duración 24h)
  - Logout
  - Roles básicos (User/Admin)

- ✅ **Gestión de Repositorios**
  - Escaneo automático de directorios Git configurados
  - Listado de proyectos detectados
  - Visualización de últimos 10 commits por proyecto
  - Navegación de archivos (árbol de directorios)
  - Vista de contenido de archivos con syntax highlighting

- ✅ **Dashboard Básico**
  - Listado de proyectos accesibles
  - Actividad reciente (últimos commits de todos los proyectos)
  - Filtro por proyecto

- ✅ **Permisos por Proyecto**
  - Asignación de usuarios a proyectos
  - Roles por proyecto: Viewer (solo lectura)
  - Admins pueden ver todos los proyectos

### Tech Stack Confirmado
- **Backend**: Rust + Axum + sqlx
- **Frontend**: Vue 3 (Composition API) + Vite + Tailwind
- **Database**: PostgreSQL 13+
- **Auth**: JWT tokens con bcrypt

### Criterios de Aceptación MVP
- [x] Usuario puede registrarse y hacer login
- [x] Admin puede configurar directorios a escanear
- [x] Sistema detecta repositorios Git automáticamente
- [x] Usuario ve solo proyectos asignados
- [x] Usuario puede navegar archivos de un repositorio
- [x] Usuario ve historial de commits
- [x] Tests de cobertura > 60%
- [x] Deployable con Docker Compose

### Exclusiones MVP (mover a Fase 2+)
- ❌ Integración Telegram (completo)
- ❌ Sistema de scripts
- ❌ Sistema de comentarios
- ❌ Notificaciones
- ❌ WebSockets (usar HTTP polling si es necesario)
- ❌ Bloqueo de pulls

**Duración estimada**: 2-3 semanas
**Entregable**: Aplicación funcional deployable en servidor de desarrollo

---

## 🚀 Fase 2 – Colaboración (3-4 semanas)

**Objetivo**: Añadir capacidades de colaboración y notificaciones básicas

### Features
- ✅ **Sistema de Comentarios**
  - Comentarios a nivel de proyecto
  - Comentarios en commits específicos
  - Respuestas a comentarios (1 nivel de anidación)
  - Mención de usuarios con @username

- ✅ **Notificaciones en App**
  - Notificación de nuevos commits en proyectos suscritos
  - Notificación de menciones en comentarios
  - Centro de notificaciones con estado leído/no leído
  - Polling cada 60 segundos para actualizar

- ✅ **Gestión de Proyectos Mejorada**
  - Roles por proyecto: Viewer, Contributor, Admin
  - Contributors pueden comentar
  - Admins de proyecto pueden gestionar miembros

- ✅ **Búsqueda Básica**
  - Búsqueda de proyectos por nombre
  - Búsqueda de commits por mensaje/autor
  - Búsqueda de archivos en repositorio

### Mejoras Técnicas
- API paginada para commits (mostrar más de 10)
- Cache de commits en base de datos
- Optimización de queries con índices

**Duración estimada**: 3-4 semanas
**Entregable**: Sistema colaborativo con notificaciones funcionales

---

## 📱 Fase 3 – Integración Telegram (2-3 semanas)

**Objetivo**: Integración completa con Telegram para notificaciones externas

### Features
- ✅ **Vinculación de Cuentas**
  - Proceso de vinculación usuario <-> Telegram chat_id
  - Comando /start en bot para vincular
  - Desvinculación desde configuración web

- ✅ **Notificaciones Telegram**
  - Envío de notificaciones de commits a Telegram
  - Notificación de menciones en comentarios
  - Configuración por usuario de qué recibir

- ✅ **Bot Interactivo Básico**
  - Comando /projects para listar proyectos
  - Comando /commits <proyecto> para ver últimos commits
  - Links a la app web en mensajes

- ⚠️ **Enlaces Mágicos (Opcional)**
  - Login sin password vía enlace en Telegram
  - Expiran en 10 minutos, un solo uso

### Tech Stack Adicional
- teloxide (Rust Telegram bot framework)
- Webhook o polling para recibir updates
- Rate limiting para API de Telegram

**Duración estimada**: 2-3 semanas
**Entregable**: Integración Telegram completa con notificaciones

---

## ⚡ Fase 4 – Scripts y Automatización (3-4 semanas)

**Objetivo**: Ejecución controlada de scripts personalizados

### Features
- ✅ **Definición de Scripts**
  - Scripts globales (solo admin puede crear)
  - Scripts por proyecto (admin de proyecto puede crear)
  - Editor de scripts con syntax highlighting
  - Parámetros configurables

- ✅ **Ejecución de Scripts**
  - Ejecución manual desde UI
  - Timeout configurable (máx 5 minutos)
  - Logs de ejecución en tiempo real
  - Historial de ejecuciones

- ✅ **Seguridad**
  - Sandboxing básico (usuario dedicado sin privilegios)
  - Variables de entorno limitadas
  - Sin acceso a filesystem fuera del proyecto
  - Rate limiting por usuario

### Tech Stack Adicional
- tokio::process para ejecución async
- Considerar contenedores para sandboxing real

**Duración estimada**: 3-4 semanas
**Entregable**: Sistema de scripts funcional con logs

---

## 🎨 Fase 5 – Refinamiento (2-3 semanas)

**Objetivo**: Mejorar UX, performance y features avanzadas

### Features
- ✅ **Dashboard Avanzado**
  - Métricas de actividad por proyecto
  - Gráficos de commits en el tiempo
  - Contribuidores más activos
  - Estadísticas de uso de scripts

- ✅ **Vista de Código Mejorada**
  - Diff de commits
  - Blame (quién modificó cada línea)
  - Búsqueda en archivos con regex
  - Descarga de archivos/directorios

- ✅ **Mejoras de Performance**
  - Cache Redis para datos frecuentes
  - WebSockets para notificaciones en tiempo real
  - Compresión de respuestas HTTP
  - CDN para assets estáticos

- ✅ **Scripts Programados**
  - Cron jobs para scripts recurrentes
  - Webhooks para triggers externos

**Duración estimada**: 2-3 semanas
**Entregable**: Sistema refinado listo para producción

---

## 🌍 Fase 6 – Producción y Escalado (Ongoing)

**Objetivo**: Preparar para usuarios reales y escala

### Infraestructura
- Deployment automatizado (CI/CD con GitHub Actions)
- Monitoreo con Prometheus + Grafana
- Logging centralizado con Loki/ELK
- Backups automáticos de base de datos
- HTTPS con Let's Encrypt
- Rate limiting en producción

### Features Adicionales
- Multi-idioma (español/inglés)
- Modo oscuro/claro
- Exportación de datos
- API pública (con API keys)
- Integraciones con GitHub/GitLab (webhooks)

### Escalabilidad
- Horizontal scaling del backend
- Read replicas de PostgreSQL
- Load balancer (nginx/traefik)
- Cache distribuido (Redis Cluster)

**Duración**: Continuous improvement

---

## 📊 Timeline Estimado

```
Mes 1: Fase 1 (MVP Core)
Mes 2: Fase 2 (Colaboración)
Mes 3: Fase 3 (Telegram) + Fase 4 inicio (Scripts)
Mes 4: Fase 4 fin + Fase 5 (Refinamiento)
Mes 5+: Fase 6 (Producción y mejoras continuas)
```

## 🎯 Decisiones de Diseño Importantes

1. **MVP sin Telegram**: Priorizar funcionalidad core antes que integraciones
2. **Polling antes que WebSockets**: Menos complejidad inicial
3. **Scripts pospuestos**: Feature compleja, no crítica para validar producto
4. **Cache gradual**: Empezar sin cache, añadir según necesidad real
5. **Testing desde MVP**: 60% cobertura mínima desde Fase 1

## 📝 Notas de Implementación

- Cada fase debe ser **desplegable a producción**
- Cada fase debe incluir **tests** (mantener > 60% coverage)
- **Code review** obligatorio antes de merge a main
- **Documentación de API** actualizada en cada fase
- **Migraciones de DB** reversibles (up/down scripts)