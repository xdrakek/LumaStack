import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { Project, Commit } from '@/types'

export const useProjectsStore = defineStore('projects', () => {
  // State
  const projects = ref<Project[]>([])
  const selectedProject = ref<Project | null>(null)
  const recentCommits = ref<Commit[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  // Getters
  const projectsCount = computed(() => projects.value.length)
  const blockedProjects = computed(() => projects.value.filter(p => p.is_blocked))

  // Actions
  async function fetchProjects(): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      // const data = await projectService.getAll()
      // projects.value = data

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar proyectos'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchProjectById(id: number): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar proyecto'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function fetchCommits(projectId: number, limit: number = 10): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cargar commits'
      throw err
    } finally {
      loading.value = false
    }
  }

  async function toggleProjectLock(projectId: number): Promise<void> {
    loading.value = true
    error.value = null

    try {
      // TODO: Implementar llamada al API
      // await projectService.toggleLock(projectId)
      // await fetchProjects()

      throw new Error('API not implemented yet')
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Error al cambiar estado del proyecto'
      throw err
    } finally {
      loading.value = false
    }
  }

  function setSelectedProject(project: Project | null): void {
    selectedProject.value = project
  }

  function clearError(): void {
    error.value = null
  }

  return {
    // State
    projects,
    selectedProject,
    recentCommits,
    loading,
    error,

    // Getters
    projectsCount,
    blockedProjects,

    // Actions
    fetchProjects,
    fetchProjectById,
    fetchCommits,
    toggleProjectLock,
    setSelectedProject,
    clearError
  }
})
