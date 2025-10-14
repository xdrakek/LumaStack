// User types
export interface User {
  id: number
  username: string
  email: string
  role: 'user' | 'admin'
  telegram_user_id?: string
  created_at: string
}

// Auth types
export interface LoginCredentials {
  username: string
  password: string
}

export interface AuthResponse {
  token: string
  refresh_token: string
  user: User
}

// Project types
export interface Project {
  id: number
  name: string
  path: string
  description?: string
  is_blocked: boolean
  created_at: string
  updated_at: string
}

// Commit types
export interface Commit {
  hash: string
  author: string
  email: string
  date: string
  message: string
  project_id: number
}

// Notification types
export type NotificationType = 'commit' | 'comment' | 'mention' | 'script'

export interface Notification {
  id: number
  user_id: number
  project_id?: number
  type: NotificationType
  title: string
  message: string
  is_read: boolean
  created_at: string
}

// Script types
export interface Script {
  id: number
  name: string
  description?: string
  content: string
  project_id?: number
  is_global: boolean
  created_by: number
  created_at: string
  updated_at: string
}

export interface ScriptExecution {
  id: number
  script_id: number
  executed_by: number
  status: 'running' | 'completed' | 'failed'
  output?: string
  error?: string
  started_at: string
  finished_at?: string
}
