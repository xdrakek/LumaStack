<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const router = useRouter()
const authStore = useAuthStore()

const username = ref('')
const password = ref('')
const loading = ref(false)
const error = ref('')

async function handleLogin() {
  loading.value = true
  error.value = ''

  try {
    // TODO: Implementar llamada al API
    console.log('Login:', { username: username.value, password: password.value })

    // Mock de login exitoso
    authStore.setAuth('mock-token', {
      id: 1,
      username: username.value,
      email: `${username.value}@example.com`,
      role: 'user'
    })

    router.push('/dashboard')
  } catch (err) {
    error.value = 'Error al iniciar sesión'
    console.error(err)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <div class="max-w-md w-full space-y-8 p-8">
      <div>
        <h2 class="text-center text-3xl font-bold">Iniciar Sesión</h2>
      </div>

      <form @submit.prevent="handleLogin" class="mt-8 space-y-6">
        <div>
          <label for="username" class="block text-sm font-medium">Usuario</label>
          <input
            id="username"
            v-model="username"
            type="text"
            required
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md"
          />
        </div>

        <div>
          <label for="password" class="block text-sm font-medium">Contraseña</label>
          <input
            id="password"
            v-model="password"
            type="password"
            required
            class="mt-1 block w-full px-3 py-2 border border-gray-300 rounded-md"
          />
        </div>

        <div v-if="error" class="text-red-600 text-sm">
          {{ error }}
        </div>

        <button
          type="submit"
          :disabled="loading"
          class="w-full py-2 px-4 bg-blue-600 text-white rounded-md hover:bg-blue-700 disabled:opacity-50"
        >
          {{ loading ? 'Cargando...' : 'Ingresar' }}
        </button>
      </form>
    </div>
  </div>
</template>
