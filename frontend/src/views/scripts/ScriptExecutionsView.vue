<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const scriptId = route.params.id
</script>

<template>
  <div class="min-h-screen bg-gray-50 p-8">
    <h1 class="text-3xl font-bold mb-6">Ejecuciones - Script {{ scriptId }}</h1>
    <p class="text-gray-600">Historial de ejecuciones (próximamente)</p>
  </div>
</template>
