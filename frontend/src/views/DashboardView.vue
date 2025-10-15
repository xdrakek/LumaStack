<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'

const authStore = useAuthStore()
</script>

<template>
  <div class="min-h-screen bg-gray-50 p-8">
    <div class="max-w-7xl mx-auto">
      <h1 class="text-3xl font-bold mb-6">Dashboard</h1>

      <div v-if="authStore.user" class="bg-white rounded-lg shadow p-6">
        <h2 class="text-xl font-semibold mb-4">Bienvenido, {{ authStore.user.username }}</h2>
        <p class="text-gray-600">Email: {{ authStore.user.email }}</p>
        <p class="text-gray-600">Rol: {{ authStore.user.role }}</p>

        <button
          @click="authStore.clearAuth"
          class="mt-4 px-4 py-2 bg-red-600 text-white rounded-md hover:bg-red-700"
        >
          Cerrar Sesión
        </button>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mt-8">
        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold mb-2">Proyectos</h3>
          <p class="text-3xl font-bold text-blue-600">0</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold mb-2">Scripts</h3>
          <p class="text-3xl font-bold text-green-600">0</p>
        </div>

        <div class="bg-white rounded-lg shadow p-6">
          <h3 class="text-lg font-semibold mb-2">Notificaciones</h3>
          <p class="text-3xl font-bold text-purple-600">0</p>
        </div>
      </div>
    </div>
  </div>
</template>
