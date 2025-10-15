<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const loading = ref(true)
const error = ref('')

onMounted(async () => {
  const token = route.params.token as string

  try {
    // TODO: Implementar autenticación con Telegram
    console.log('Telegram token:', token)

    // Mock de autenticación exitosa
    await new Promise(resolve => setTimeout(resolve, 1000))

    authStore.setAuth('telegram-token', {
      id: 1,
      username: 'telegram-user',
      email: 'user@telegram.com',
      role: 'user',
      telegramUserId: '123456789'
    })

    router.push('/dashboard')
  } catch (err) {
    error.value = 'Token inválido o expirado'
    console.error(err)
  } finally {
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <div class="max-w-md w-full p-8 text-center">
      <div v-if="loading">
        <h2 class="text-2xl font-bold mb-4">Autenticando con Telegram...</h2>
        <div class="animate-spin h-8 w-8 border-4 border-blue-500 border-t-transparent rounded-full mx-auto"></div>
      </div>

      <div v-else-if="error" class="text-red-600">
        <h2 class="text-2xl font-bold mb-4">Error</h2>
        <p>{{ error }}</p>
        <button
          @click="router.push('/login')"
          class="mt-4 px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700"
        >
          Volver al login
        </button>
      </div>
    </div>
  </div>
</template>
