import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { authGuard, redirectAfterLoginGuard } from './guards'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    redirect: '/dashboard'
  },
  {
    path: '/login',
    name: 'login',
    component: () => import('@/views/auth/LoginView.vue'),
    meta: { requiresGuest: true }
  },
  {
    path: '/auth/telegram/:token',
    name: 'telegram-auth',
    component: () => import('@/views/auth/TelegramAuthView.vue'),
    meta: { requiresGuest: true }
  },
  {
    path: '/dashboard',
    name: 'dashboard',
    component: () => import('@/views/DashboardView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/projects',
    name: 'projects',
    component: () => import('@/views/projects/ProjectsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/projects/:id',
    name: 'project-detail',
    component: () => import('@/views/projects/ProjectDetailView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/projects/:id/tree/:path(.*)*',
    name: 'project-tree',
    component: () => import('@/views/projects/ProjectTreeView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/projects/:id/commits',
    name: 'project-commits',
    component: () => import('@/views/projects/ProjectCommitsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/scripts',
    name: 'scripts',
    component: () => import('@/views/scripts/ScriptsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/scripts/:id/executions',
    name: 'script-executions',
    component: () => import('@/views/scripts/ScriptExecutionsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/notifications',
    name: 'notifications',
    component: () => import('@/views/NotificationsView.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/admin',
    name: 'admin',
    component: () => import('@/views/admin/AdminView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true }
  },
  {
    path: '/:pathMatch(.*)*',
    name: 'not-found',
    component: () => import('@/views/NotFoundView.vue')
  }
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
})

// Navigation guards
router.beforeEach(authGuard)
router.beforeEach(redirectAfterLoginGuard)

export default router
