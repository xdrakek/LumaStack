<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50">
    <div class="text-center">
      <h1 class="text-6xl font-bold text-gray-800 mb-4">404</h1>
      <p class="text-xl text-gray-600 mb-8">Página no encontrada</p>
      <button
        @click="router.push('/')"
        class="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700"
      >
        Volver al inicio
      </button>
    </div>
  </div>
</template>
