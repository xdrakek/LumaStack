import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Notification } from '@/types'

export const useNotificationsStore = defineStore('notifications', () => {
  // State
  const notifications = ref<Notification[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const unreadCount = computed(() => notifications.value.filter(n => !n.is_read).length)
  const unreadNotifications = computed(() => notifications.value.filter(n => !n.is_read))
  const readNotifications = computed(() => notifications.value.filter(n => n.is_read))

  // Actions
  async function fetchNotifications(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      // const data = await notificationService.getAll()
      // notifications.value = data

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar notificaciones'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function markAsRead(notificationId: number): Promise<void> {
    try {
      // TODO: Implementar llamada al API
      // await notificationService.markAsRead(notificationId)

      // Actualizar estado local
      const notification = notifications.value.find(n => n.id === notificationId)
      if (notification) {
        notification.is_read = true
      }

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al marcar notificación como leída'
      throw err
    }
  }

  async function markAllAsRead(): Promise<void> {
    try {
      // TODO: Implementar llamada al API
      // await notificationService.markAllAsRead()

      // Actualizar estado local
      notifications.value.forEach(n => n.is_read = true)

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al marcar todas como leídas'
      throw err
    }
  }

  async function deleteNotification(notificationId: number): Promise<void> {
    try {
      // TODO: Implementar llamada al API
      // await notificationService.delete(notificationId)

      // Actualizar estado local
      const index = notifications.value.findIndex(n => n.id === notificationId)
      if (index !== -1) {
        notifications.value.splice(index, 1)
      }

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al eliminar notificación'
      throw err
    }
  }

  function addNotification(notification: Notification): void {
    notifications.value.unshift(notification)
  }

  function clearError(): void {
    error.value = null
  }

  return {
    // State
    notifications,
    loading,
    error,

    // Getters
    unreadCount,
    unreadNotifications,
    readNotifications,

    // Actions
    fetchNotifications,
    markAsRead,
    markAllAsRead,
    deleteNotification,
    addNotification,
    clearError
  }
})
