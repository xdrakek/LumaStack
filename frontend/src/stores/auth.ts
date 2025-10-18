import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, AuthResponse, LoginCredentials } from '@/types'

export const useAuthStore = defineStore('auth', () => {
  // State
  const user = ref<User | null>(null)
  const token = ref<string | null>(localStorage.getItem('token'))
  const refreshToken = ref<string | null>(localStorage.getItem('refresh_token'))
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const isAuthenticated = computed(() => !!token.value && !!user.value)
  const isAdmin = computed(() => user.value?.role === 'admin')

  // Actions
  async function login(credentials: LoginCredentials): Promise<void> {
    console.log(credentials) //TODO:Remover
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      // const response = await authService.login(credentials)
      // setAuthData(response)

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al iniciar sesión'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function loginWithTelegram(magicLink: string): Promise<void> {
    console.log(magicLink) //TODO:Remover
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al iniciar sesión con Telegram'
      throw err
    } finally {
      loading.value = false
    }
  }

  function setAuthData(authResponse: AuthResponse): void {
    user.value = authResponse.user
    token.value = authResponse.token
    refreshToken.value = authResponse.refresh_token

    localStorage.setItem('token', authResponse.token)
    localStorage.setItem('refresh_token', authResponse.refresh_token)
  }

  function logout(): void {
    user.value = null
    token.value = null
    refreshToken.value = null

    localStorage.removeItem('token')
    localStorage.removeItem('refresh_token')
  }

  async function refreshAuthToken(): Promise<void> {
    if (!refreshToken.value) {
      logout()
      return
    }

    try {
      // TODO: Implementar llamada al API
      throw new Error('API not implemented yet')
    } catch (err) {
      logout()
      throw err
    }
  }

  async function fetchCurrentUser(): Promise<void> {
    if (!token.value) return

    loading.value = true

    try {
      // TODO: Implementar llamada al API
      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al obtener usuario'
      logout()
    } finally {
      loading.value = false
    }
  }

  return {
    // State
    user,
    token,
    loading,
    error,

    // Getters
    isAuthenticated,
    isAdmin,

    // Actions
    login,
    loginWithTelegram,
    logout,
    refreshAuthToken,
    fetchCurrentUser,
    setAuthData
  }
})
