// User types
export interface User {
  id: number
  username: string
  email: string
  role: 'user' | 'admin'
  telegram_user_id?: string
  created_at: string
  updated_at: string
}

// Auth types
export interface LoginCredentials {
  username: string
  password: string
}

export interface AuthResponse {
  user: User
  token: string
  refresh_token: string
}

export interface TelegramAuthPayload {
  magic_link: string
}

// Project types
export interface Project {
  id: number
  name: string
  path: string
  description?: string
  is_blocked: boolean
  last_commit_hash?: string
  last_pull_at?: string
  created_at: string
  updated_at: string
}

export interface ProjectFile {
  name: string
  path: string
  type: 'file' | 'directory'
  size?: number
}

export interface Commit {
  hash: string
  author: string
  date: string
  message: string
}

// Script types
export interface Script {
  id: number
  name: string
  description?: string
  content: string
  is_global: boolean
  project_id?: number
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

// Notification types
export interface Notification {
  id: number
  user_id: number
  project_id?: number
  type: 'commit' | 'comment' | 'mention' | 'script_execution'
  title: string
  message: string
  is_read: boolean
  created_at: string
}

export interface NotificationSubscription {
  id: number
  user_id: number
  project_id: number
  notify_commits: boolean
  notify_comments: boolean
  notify_mentions: boolean
  created_at: string
}

// Comment types
export interface Comment {
  id: number
  project_id: number
  user_id: number
  content: string
  created_at: string
  updated_at: string
  user?: User
}

// API Response types
export interface ApiResponse<T = any> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  page_size: number
  total_pages: number
}

// WebSocket types
export interface WebSocketMessage {
  type: 'commit' | 'script_status' | 'notification'
  data: any
}
