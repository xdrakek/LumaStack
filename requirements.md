# Especificaciones de Requisitos del Sistema - LumaStack

## 1. Introducción

### 1.1 Propósito
Este documento define los requisitos funcionales y no funcionales para el sistema LumaStack, una plataforma web para monitoreo y gestión de repositorios Git con integración de Telegram.

### 1.2 Alcance
LumaStack proporciona una solución centralizada para:
- Visualización y monitoreo de repositorios Git
- Gestión de usuarios con roles diferenciados
- Integración completa con Telegram para notificaciones
- Ejecución de scripts personalizados
- Sistema de comentarios colaborativo

### 1.3 Definiciones y Acrónimos
- **API**: Application Programming Interface
- **Git**: Sistema de control de versiones distribuido
- **JWT**: JSON Web Token
- **REST**: Representational State Transfer
- **SPA**: Single Page Application
- **CI/CD**: Continuous Integration/Continuous Deployment
- **RBAC**: Role-Based Access Control

## 2. Descripción General del Sistema

### 2.1 Perspectiva del Producto
LumaStack es un sistema web independiente que actúa como interfaz centralizada entre repositorios Git y usuarios, proporcionando capacidades avanzadas de monitoreo y colaboración a través de Telegram.

### 2.2 Funciones del Producto
- Autenticación y autorización de usuarios
- Monitoreo en tiempo real de repositorios Git locales
- Sistema de notificaciones vía Telegram
- Ejecución controlada de scripts
- Colaboración mediante comentarios
- Dashboard de métricas y analytics

### 2.3 Usuarios del Sistema
- **Usuario Regular**: Desarrollador con acceso a proyectos asignados
- **Administrador**: Acceso completo al sistema y gestión de usuarios
- **Sistema Externo**: APIs de Telegram

## 3. Requisitos Funcionales

### 3.1 Gestión de Usuarios y Autenticación

#### RF-001: Registro de Usuarios
**Prioridad**: Alta
**Descripción**: El sistema debe permitir el registro de nuevos usuarios
**Criterios de Aceptación**:
- El usuario proporciona email, nombre de usuario y contraseña
- El sistema valida la unicidad del email y username
- La contraseña debe cumplir política de seguridad (mín. 8 caracteres, mayúsculas, minúsculas, números)
- El usuario por defecto tiene rol "Usuario"

#### RF-002: Autenticación con Credenciales
**Prioridad**: Alta
**Descripción**: El sistema debe autenticar usuarios con email/username y contraseña
**Criterios de Aceptación**:
- Acepta email o username como identificador
- Valida credenciales contra base de datos
- Genera JWT token con duración de 24 horas
- Registra intento de login (exitoso/fallido)
- Implementa rate limiting (5 intentos por minuto)

#### RF-003: Integración con Telegram
**Prioridad**: Alta
**Descripción**: El sistema debe permitir vincular cuentas de Telegram
**Criterios de Aceptación**:
- Usuario inicia proceso de vinculación desde configuración
- Sistema genera token temporal (5 minutos) o genera url
- Usuario envía token al bot de Telegram
- Sistema confirma vinculación y almacena chat_id
- Usuario puede desvincular cuenta cuando desee

#### RF-004: Autenticación con Enlaces Mágicos
**Prioridad**: Media
**Descripción**: El sistema debe permitir login vía enlaces enviados a Telegram
**Criterios de Aceptación**:
- Usuario solicita enlace mágico desde página de login
- Sistema genera enlace único con expiración (10 minutos)
- Enlace se envía al Telegram vinculado del usuario
- Al hacer clic, usuario inicia sesión automáticamente
- Enlaces son de un solo uso

#### RF-005: Gestión de Roles
**Prioridad**: Media
**Descripción**: El sistema debe implementar control de acceso basado en roles
**Criterios de Aceptación**:
- Roles disponibles: Usuario, Administrador
- Administrador puede cambiar roles de usuarios
- Cambios de rol son auditados
- Sistema verifica permisos en cada operación sensible

### 3.2 Gestión de Repositorios

#### RF-006: Configuración de Repositorios
**Prioridad**: Alta
**Descripción**: El sistema debe permitir registrar repositorios Git para monitoreo
**Criterios de Aceptación**:
- Administrador puede agregar directorios de escaneo
- Sistema detecta automáticamente repositorios Git válidos
- Se almacena path, nombre y metadatos básicos
- Repositorios aparecen disponibles para usuarios autorizados

#### RF-007: Visualización de Archivos
**Prioridad**: Alta
**Descripción**: El sistema debe mostrar contenido de repositorios con navegación
**Criterios de Aceptación**:
- Interfaz de navegación tipo árbol de directorios
- Visualización de archivos con resaltado de sintaxis
- Soporte para múltiples lenguajes de programación
- Vista previa de imágenes y documentos
- Breadcrumb para navegación

#### RF-008: Historial de Commits
**Prioridad**: Alta  
**Descripción**: El sistema debe mostrar historial detallado de commits
**Criterios de Aceptación**:
- Lista ultimos 10 commits con metadatos
- Muestra autor, fecha, mensaje



#### RF-009: Monitoreo de Cambios
**Prioridad**: Alta
**Descripción**: El sistema debe detectar automáticamente cuando se hace un pull en un repositorio
**Criterios de Aceptación**:
- Detección de pull en el repositorio
- Actualización en tiempo real via WebSockets
- Log de actividad para auditoría

#### RF-010: Control de Bloqueos
**Prioridad**: Media
**Descripción**: El sistema debe permitir bloquear pulls de repositorios
**Criterios de Aceptación**:
- Administrador puede bloquear/desbloquear repositorios
- Usuario puede solicitar bloqueo (requiere aprobación admin)
- Estado de bloqueo visible en interfaz
- Notificaciones cuando se bloquea/desbloquea
- Historial de cambios de estado

### 3.3 Sistema de Notificaciones

#### RF-011: Configuración de Notificaciones
**Prioridad**: Alta
**Descripción**: El sistema debe permitir personalizar notificaciones por proyecto
**Criterios de Aceptación**:
- Usuario selecciona proyectos para notificaciones
- Notificaciones habilitadas/deshabilitadas por proyecto
- Configuración persiste entre sesiones


#### RF-012: Notificaciones de Comentarios
**Prioridad**: Media
**Descripción**: El sistema debe notificar sobre nuevos comentarios
**Criterios de Aceptación**:
- Notificación cuando se comenta en proyectos suscritos
- Notificación automática en menciones (@usuario)
- Incluye contexto del comentario
- Posibilidad de responder desde Telegram
- Hilo de conversación mantenido

### 3.4 Ejecución de Scripts

#### RF-013: Gestión de Scripts
**Prioridad**: Media
**Descripción**: El sistema debe permitir definir y gestionar scripts ejecutables
**Criterios de Aceptación**:
- Administrador crea scripts globales
- Usuario con permisos crea scripts de proyecto
- Editor con syntax highlighting
- Validación de sintaxis básica
- Categorización y descripción de scripts

#### RF-014: Ejecución de Scripts
**Prioridad**: Media
**Descripción**: El sistema debe ejecutar scripts de forma segura y controlada
**Criterios de Aceptación**:
- Ejecución en ambiente sandboxed
- Timeout configurable (máx. 5 minutos)
- Logging completo de ejecución
- Estado en tiempo real (ejecutando, completado, error)
- Variables de entorno limitadas
- Cancelación de ejecución disponible

#### RF-015: Logs de Ejecución
**Prioridad**: Media
**Descripción**: El sistema debe proporcionar logs detallados de ejecuciones
**Criterios de Aceptación**:
- Output en tiempo real durante ejecución
- Historial persistente de ejecuciones
- Filtrado por script, usuario, fecha, estado
- Exportación de logs

### 3.5 Sistema de Comentarios

#### RF-016: Comentarios en Proyectos
**Prioridad**: Media
**Descripción**: El sistema debe permitir comentar proyectos y archivos
**Criterios de Aceptación**:
- Comentarios a nivel de proyecto y archivo
- Soporte para markdown básico
- Menciones con @usuario
- Edición y eliminación de comentarios propios
- Timestamps y autoría visible
- Respuestas anidadas (1 nivel)

#### RF-017: Integración de Comentarios con Telegram
**Prioridad**: Media
**Descripción**: Los comentarios deben integrarse con notificaciones Telegram
**Criterios de Aceptación**:
- Comentarios enviados automáticamente a Telegram
- Posibilidad de comentar desde Telegram
- Hilos de conversación sincronizados
- Comandos de bot para comentar
- Formato enriquecido en notificaciones

### 3.6 Dashboard y Reportes

#### RF-018: Dashboard Principal
**Prioridad**: Alta
**Descripción**: El sistema debe proveer un dashboard con actividad reciente
**Criterios de Aceptación**:
- Resumen de actividad de últimos 7 días
- Últimos commits por proyecto
- Scripts ejecutados recientemente
- Comentarios recientes

## 4. Requisitos No Funcionales

### 4.1 Rendimiento

#### RNF-001: Tiempo de Respuesta
- Páginas web deben cargar en < 2 segundos
- APIs deben responder en < 500ms
- Búsquedas deben completarse en < 1 segundo
- Navegación de archivos debe ser instantánea (< 100ms)

#### RNF-002: Throughput
- Sistema debe soportar 100 usuarios concurrentes
- APIs deben manejar 1000 requests/minuto
- Base de datos debe soportar 500 transacciones/segundo

#### RNF-003: Escalabilidad
- Arquitectura debe permitir escalado horizontal
- Base de datos debe soportar sharding
- Cache distribuido para sesiones
- Load balancing para múltiples instancias

### 4.2 Confiabilidad

#### RNF-004: Disponibilidad
- Uptime objetivo: 99.9% (8.76 horas/año downtime)
- Recovery Time Objective (RTO): 4 horas
- Recovery Point Objective (RPO): 1 hora
- Monitoreo 24/7 con alertas automáticas

#### RNF-005: Tolerancia a Fallos
- Degradación graceful ante fallos de servicios externos
- Retry automático para APIs externas
- Circuit breaker para prevenir cascada de fallos
- Fallback a modo offline cuando sea posible

### 4.3 Seguridad

#### RNF-006: Autenticación y Autorización
- Implementación de JWT con refresh tokens
- Rate limiting en todos los endpoints críticos
- Validación de entrada en cliente y servidor
- Principio de menor privilegio para permisos

#### RNF-007: Protección de Datos
- Encriptación de datos sensibles en tránsito (HTTPS)
- Encriptación de contraseñas con bcrypt (factor 12+)
- Tokens de Telegram encriptados en BD
- Logs sin información sensible

#### RNF-008: Compliance
- Cumplimiento con GDPR para datos de usuarios EU
- Derecho al olvido implementado
- Audit trail para operaciones sensibles
- Política de retención de datos definida

### 4.4 Usabilidad

#### RNF-009: Interfaz de Usuario
- Diseño responsive para móvil, tablet y desktop
- Accesibilidad WCAG 2.1 AA
- Tiempos de carga perceptibles < 1 segundo
- Interfaz intuitiva sin necesidad de documentación

#### RNF-010: Experiencia de Usuario
- Onboarding guiado para nuevos usuarios
- Atajos de teclado para acciones comunes
- Búsqueda con autocompletado
- Navegación consistente en toda la aplicación

### 4.5 Mantenibilidad

#### RNF-011: Código
- Cobertura de tests > 80%
- Documentación de API actualizada automáticamente
- Code review obligatorio para todos los cambios
- Análisis estático de calidad de código

#### RNF-012: Deployment
- Deployment automatizado con rollback
- Configuración externalized para diferentes ambientes
- Health checks para todos los servicios
- Logs estructurados con niveles apropiados

## 5. Restricciones Técnicas

### 5.1 Tecnológicas
- Frontend: Vue.js 3, Tailwind CSS
- Backend: Rust, Axum, SQLx
- Base de datos: PostgreSQL 18


### 5.2 Regulatorias
- Respeto a rate limits de Telegram API
- Protección de datos según GDPR

## 6. Criterios de Aceptación del Sistema

### 6.1 Funcionales
- Todos los requisitos funcionales implementados y probados
- Cobertura de tests automatizados > 80%
- Documentación de usuario completa
- Performance bajo carga simulada

### 6.2 No Funcionales
- Auditoría de seguridad pasada
- Pruebas de carga exitosas
- Deployment automatizado funcionando
- Monitoreo y alertas configuradas

### 6.3 Usuario
- UAT (User Acceptance Testing) completado
- Feedback de usuarios beta incorporado
- Capacitación de usuarios finales realizada
- Soporte técnico preparado

## 7. Riesgos y Mitigaciones

### 7.1 Técnicos
- **Riesgo**: Rate limiting de APIs externas
  **Mitigación**: Implementar caché inteligente y retry con backoff
- **Riesgo**: Escalabilidad de base de datos
  **Mitigación**: Implementar read replicas y connection pooling

### 7.2 Operacionales
- **Riesgo**: Pérdida de datos
  **Mitigación**: Backups automáticos cifrados y pruebas de restore
- **Riesgo**: Downtime por mantenimiento
  **Mitigación**: Blue-green deployments y ventanas de mantenimiento

### 7.3 Negocio
- **Riesgo**: Cambios en APIs de terceros
  **Mitigación**: Wrapper layer y monitoreo de cambios en APIs
- **Riesgo**: Requisitos de compliance cambiantes
  **Mitigación**: Arquitectura flexible y logging comprehensivo

## 8. Apéndices

### 8.1 Glosario de Términos
- **Pull Request**: Solicitud de fusión de cambios en Git
- **Webhook**: Callback HTTP para eventos en tiempo real
- **Rate Limiting**: Limitación de frecuencia de requests
- **Circuit Breaker**: Patrón para prevenir fallos en cascada

### 8.2 Referencias
- [RFC 7519 - JSON Web Token](https://tools.ietf.org/html/rfc7519)
- [GitHub API Documentation](https://docs.github.com/en/rest)
- [Telegram Bot API](https://core.telegram.org/bots/api)
- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

---

**Documento actualizado**: Octubre 2024
**Versión**: 1.0
**Aprobado por**: Equipo de Arquitectura LumaStack