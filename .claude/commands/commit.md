---
description: Analiza cambios y genera commits siguiendo Conventional Commits con mensajes en español
allowed-tools:
  - "Bash(git:*)"
  - "Edit(*)"
  - "Write(*)"
---

# Generador de Commits Inteligente

Este comando analiza tus cambios en Git y genera commits siguiendo el estándar **Conventional Commits** con mensajes en español.

---

## Proceso Automático

### 1. **Análisis del Contexto**

Ejecuta en paralelo estos comandos para entender el estado actual:
- `git status` para ver archivos modificados
- `git diff --cached` para cambios en staging
- `git diff HEAD` para todos los cambios
- `git branch --show-current` para la rama actual
- `git log --oneline -5` para contexto de commits recientes

### 2. **Categorización de Cambios**

Analiza los archivos modificados y categorízalos según el tipo de cambio:

**Tipos de Commit (Conventional Commits)**:

- `feat`: Nueva funcionalidad o feature
- `fix`: Corrección de bugs
- `docs`: Solo cambios de documentación (.md, comentarios)
- `style`: Formato de código (espacios, punto y coma, prettier, eslint)
- `refactor`: Reestructuración de código sin cambiar funcionalidad
- `perf`: Mejoras de rendimiento
- `test`: Añadir o modificar tests
- `build`: Cambios en dependencias o build (Cargo.toml, package.json)
- `ci`: Cambios en CI/CD (.github/workflows, scripts)
- `chore`: Mantenimiento general (gitignore, configuración)

**Ámbitos comunes en LumaStack**:
- `auth` - Autenticación
- `projects` - Gestión de proyectos
- `api` - Endpoints de API
- `db` - Base de datos/migraciones
- `ui` - Componentes de interfaz
- `git` - Integración con Git

### 3. **Análisis de Atomicidad**

Determina si los cambios deben separarse en múltiples commits:

✅ **UN commit si**:
- Todos los cambios están relacionados con una sola tarea
- Los cambios son interdependientes
- El commit es reversible como unidad

❌ **MÚLTIPLES commits si**:
- Hay cambios de diferentes tipos (feat + fix + docs)
- Cambios en diferentes features/módulos
- Un commit sería demasiado grande (>10 archivos no relacionados)

### 4. **Generación de Opciones de Mensaje**

Genera **3 opciones de mensaje** siguiendo este formato:

```
<tipo>[ámbito opcional]: <descripción>

[cuerpo opcional]

[footer opcional]
```

**Reglas de formato**:
- **Primera línea**: máximo 72 caracteres
- **Modo imperativo**: "añade", "corrige", "actualiza", "elimina"
- **Minúsculas**: después de los dos puntos
- **Sin punto final**: en la primera línea
- **Descripción clara**: qué hace el cambio, no cómo lo hace

**Breaking Changes**:
- Añadir footer `BREAKING CHANGE:` al final del mensaje con descripción del cambio
- Usar scope especial `(breaking)` si aplica: `feat(breaking): cambia formato de respuesta`

### 5. **Selección y Ejecución**

1. Presenta las 3 opciones numeradas
2. Recomienda la mejor opción con justificación breve
3. Pregunta al usuario si está de acuerdo o quiere modificar
4. Ejecuta el commit con los comandos apropiados de git add y git commit

---

## Ejemplos de Mensajes Correctos

### Simple (solo título)
```
feat(auth): añade formulario de registro
fix(api): corrige validación de email en login
docs: actualiza README con instrucciones de instalación
style: formatea código con prettier
refactor(git): simplifica lógica de escaneo de repositorios
test(auth): añade tests para validación de contraseñas
chore: actualiza dependencias de Rust
```

### Con ámbito y breaking change
```
feat(breaking): cambia formato de respuesta de /projects

BREAKING CHANGE: La respuesta ahora incluye pagination metadata
```

### Con cuerpo explicativo
```
fix(db): corrige race condition en commits cache

El query de INSERT no verificaba duplicados correctamente,
causando errores al escanear repositorios simultáneamente.

Añade índice UNIQUE en commit_hash para prevenir duplicados.
```

### Múltiples cambios relacionados
```
feat(projects): implementa navegación de archivos

- Añade endpoint GET /projects/:id/tree/:path
- Crea componente FileExplorer.vue
- Implementa syntax highlighting con highlight.js
```

---

## Reglas Estrictas

### ✅ SÍ hacer:

- **Analizar TODOS los cambios** antes de commitear
- **Separar commits** si hay cambios no relacionados
- **Usar imperativo**: "añade login" (no "añadido login")
- **Ser específico**: "corrige validación de email" (no "fix bug")
- **Stagear solo lo necesario**: `git add` selectivo
- **Revisar diff** antes de confirmar
- **Mensajes en español**: descripción y cuerpo

### ❌ NO hacer:

- ❌ **NO añadir footer de co-autoría de Claude**
- ❌ **NO usar commits genéricos**: "fix", "update", "changes"
- ❌ **NO mezclar tipos**: feat + fix en un mismo commit
- ❌ **NO commitear archivos generados**: build/, node_modules/, target/
- ❌ **NO usar punto final** en la primera línea
- ❌ **NO exceder 72 caracteres** en la primera línea
- ❌ **NO usar pasado**: "añadido" → usar "añade"

---

## Flujo de Ejecución

Cuando el usuario ejecute `/commit`, sigue este flujo:

1. **Ejecuta** `git status` y `git diff HEAD`
2. **Lista** archivos modificados agrupados por tipo
3. **Pregunta** si hay archivos que no deberían commitearse
4. **Analiza** si se necesita un commit o múltiples
5. **Genera** 3 opciones de mensaje
6. **Recomienda** la mejor opción con justificación
7. **Espera confirmación** del usuario
8. **Ejecuta** `git add` + `git commit` con el mensaje

---

## Casos Especiales

### Si hay cambios sin stagear
```
📝 Cambios detectados pero no en staging.
¿Quieres añadir todos los archivos o seleccionar específicos?

[1] git add -A (todos)
[2] Seleccionar archivos manualmente
```

### Si hay archivos no rastreados importantes
```
⚠️  Archivos sin rastrear detectados:
  - database/schema.sql
  - .env.example

¿Quieres incluirlos en este commit?
```

### Si detecta múltiples tipos de cambios
```
🔀 Detecté cambios de diferentes tipos:
  - feat: Nuevo componente LoginForm.vue
  - fix: Corrección en authService.ts
  - docs: Actualización de README.md

Recomiendo crear 3 commits separados. ¿Proceder?
```

### Si hay breaking changes
```
⚠️  BREAKING CHANGE detectado:
  - Cambio en schema de base de datos
  - Modificación de API endpoint

¿Confirmas que es un breaking change? Se añadirá footer BREAKING CHANGE?
```

---

## Verificación Pre-Commit

Antes de ejecutar el commit, verifica:

- [ ] ¿Los cambios están relacionados? (atomicidad)
- [ ] ¿El mensaje es claro y descriptivo?
- [ ] ¿Se incluyeron todos los archivos necesarios?
- [ ] ¿Se excluyeron archivos generados/temporales?
- [ ] ¿El tipo de commit es correcto?
- [ ] ¿El ámbito es apropiado?
- [ ] ¿Es un breaking change? → Añadir footer BREAKING CHANGE?

---

## Formato de Salida

Presenta la información así:

```
📊 Estado del Repositorio
━━━━━━━━━━━━━━━━━━━━━━
Rama: main
Archivos modificados: 5

📝 Archivos en Staging:
  M  frontend/src/components/LoginForm.vue
  M  frontend/src/stores/auth.ts
  A  frontend/src/composables/useAuth.ts

💡 Opciones de Mensaje:
━━━━━━━━━━━━━━━━━━━━━━

[1] feat(auth): implementa formulario de login con validación

[2] feat(auth): añade componente LoginForm y composable useAuth

[3] feat(auth): implementa autenticación completa en frontend

Añade LoginForm.vue con validación de email/password,
store de autenticación con Pinia, y composable reutilizable.

✅ Recomendación: Opción 2
Razón: Específica y concisa, describe claramente los componentes añadidos

¿Proceder con la opción 2? (sí/no/modificar)
```

---

## Comandos Git Internos

El comando ejecutará los comandos git necesarios para:
- Análisis: git status, git diff, git branch, git log
- Staging: git add según selección del usuario
- Commit: git commit con el mensaje generado
- Confirmación: git log para verificar el commit creado

---

## Notas Finales

- **Prioridad**: Claridad > Brevedad
- **Contexto**: Si el título no es suficiente, añade cuerpo
- **Consistencia**: Sigue el estilo de commits anteriores del proyecto
- **Reversibilidad**: Cada commit debe poder revertirse de forma segura