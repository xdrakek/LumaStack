import type { RouteMeta as VueRouteMeta } from 'vue-router'

/**
 * Extensión de RouteMeta con propiedades personalizadas
 */
declare module 'vue-router' {
  interface RouteMeta extends VueRouteMeta {
    /** Requiere que el usuario esté autenticado */
    requiresAuth?: boolean
    /** Requiere que el usuario NO esté autenticado (solo invitados) */
    requiresGuest?: boolean
    /** Requiere permisos de administrador */
    requiresAdmin?: boolean
    /** Título de la página (opcional) */
    title?: string
    /** Breadcrumb para navegación (opcional) */
    breadcrumb?: string
  }
}
