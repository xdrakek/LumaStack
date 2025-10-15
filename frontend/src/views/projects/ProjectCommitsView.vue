<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
const projectId = route.params.id
</script>

<template>
  <div class="min-h-screen bg-gray-50 p-8">
    <h1 class="text-3xl font-bold mb-6">Commits - Proyecto {{ projectId }}</h1>
    <p class="text-gray-600">Historial de commits (próximamente)</p>
  </div>
</template>
