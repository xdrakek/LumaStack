# 🚀 LumaStack

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Vue 3](https://img.shields.io/badge/Vue-3.0-4FC08D.svg)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-CE422B.svg)](https://www.rust-lang.org/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-13+-336791.svg)](https://postgresql.org/)

> **Plataforma moderna para monitoreo y gestión de repositorios Git con integración avanzada de Telegram**

LumaStack es una solución completa que permite visualizar proyectos Git, monitorear commits, gestionar scripts personalizados y recibir notificaciones inteligentes a través de Telegram. Diseñada para equipos de desarrollo que buscan centralizar el control de sus repositorios con una experiencia de usuario moderna.

## ✨ Características Principales

### 🔐 Autenticación Avanzada
- Registro y login tradicional con usuario/contraseña
- Autenticación vía Telegram con enlaces mágicos
- Gestión de roles (Usuario/Administrador)
- Sesiones seguras con JWT

### 📁 Gestión de Repositorios
- **Visualización de proyectos**: Navega por la estructura de archivos con resaltado de sintaxis
- **Historial de commits**: Rastrea todos los cambios con detalles completos
- **Monitoreo en tiempo real**: Detecta pulls automáticamente
- **Control de bloqueos**: Previene cambios no autorizados en proyectos críticos

### 🤖 Integración con Telegram
- **Notificaciones inteligentes**: Recibe alertas de commits, comentarios y menciones
- **Bot interactivo**: Responde comandos directamente desde Telegram
- **Enlaces mágicos**: Acceso rápido sin contraseña
- **Menciones contextuales**: Notifica automáticamente a usuarios mencionados

### ⚡ Ejecución de Scripts
- **Scripts personalizados**: Define y ejecuta scripts específicos por proyecto
- **Scripts globales**: Herramientas disponibles para todos los proyectos
- **Control de permisos**: Gestiona quién puede ejecutar qué scripts

### 💬 Sistema de Comentarios
- **Colaboración contextual**: Comenta proyectos
- **Menciones automáticas**: Notifica a compañeros con @usuario
- **Historial completo**: Mantén registro de todas las conversaciones
- **Integración Telegram**: Comenta directamente desde el chat

### 📊 Dashboard Inteligente
- **Vista consolidada**: Últimos commits, scripts ejecutados y comentarios

## 🏗️ Arquitectura Técnica

### Stack Tecnológico

**Frontend** 🎨
- **Vue.js 3** con Composition API
- **Tailwind CSS** para styling moderno
- **shadcn/vue** para componentes de UI consistentes
- **Pinia** para gestión de estado reactivo
- **Vite** para desarrollo rápido y build optimizado

**Backend** ⚙️
- **Rust** con Axum/Actix-web para APIs ultra-rápidas
- **PostgreSQL** como base de datos principal
- **WebSockets** para comunicación en tiempo real


### Diagrama de Arquitectura

```
┌─────────────────────────────────────────────────────────────┐
│                    CAPA DE PRESENTACIÓN                     │
│  ┌─────────────────────────────────────────────────────┐   │
│  │         Vue.js 3  + Tailwind                        │   │
│  │              shadcn/vue + Pinia                     │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────┬───────────────────────────────────────┘
                      │ HTTPS/WSS
┌─────────────────────▼───────────────────────────────────────┐
│                     CAPA DE APLICACIÓN                     │
│  ┌─────────────────────────────────────────────────────┐   │
│  │               Rust API Server                       │   │
│  │    ┌──────────────────────────────────────────┐    │   │
│  │    │  Auth    │  Git     │  Telegram │ Script │    │   │
│  │    │ Service  │ Service  │  Service  │Service │    │   │
│  │    └──────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────┬───────────────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────────────┐
│                    CAPA DE DATOS                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐    │
│  │ PostgreSQL  │  │             │  │  APIs Externas  │    │
│  │ (Principal) │  │             │    Telegram        │    |
│  └─────────────┘  └─────────────┘  └─────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```





