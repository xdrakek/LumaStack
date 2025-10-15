import type { NavigationGuardNext, RouteLocationNormalized } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

/**
 * Guard de autenticación
 * Verifica si el usuario está autenticado antes de acceder a rutas protegidas
 */
export function authGuard(
  to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) {
  const authStore = useAuthStore()

  // Si la ruta requiere autenticación
  if (to.meta.requiresAuth) {
    if (!authStore.isAuthenticated) {
      // Redirigir al login y guardar la ruta destino
      next({
        name: 'login',
        query: { redirect: to.fullPath }
      })
      return
    }

    // Si requiere permisos de admin
    if (to.meta.requiresAdmin && !authStore.isAdmin) {
      // Redirigir al dashboard si no es admin
      next({ name: 'dashboard' })
      return
    }
  }

  // Si la ruta requiere ser invitado (solo para no autenticados)
  if (to.meta.requiresGuest && authStore.isAuthenticated) {
    next({ name: 'dashboard' })
    return
  }

  next()
}

/**
 * Guard para manejar redirecciones después del login
 */
export function redirectAfterLoginGuard(
  to: RouteLocationNormalized,
  _from: RouteLocationNormalized,
  next: NavigationGuardNext
) {
  const authStore = useAuthStore()

  if (to.name === 'login' && authStore.isAuthenticated) {
    const redirect = (to.query.redirect as string) || '/dashboard'
    next(redirect)
    return
  }

  next()
}
