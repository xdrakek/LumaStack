-- ============================================
-- LumaStack Database Schema
-- PostgreSQL 13+
-- ============================================

-- ============================================
-- FASE 1 - MVP TABLES
-- ============================================

-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'user', -- 'user' | 'admin'
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for users
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_active ON users(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE users IS 'System users with authentication credentials';
COMMENT ON COLUMN users.role IS 'Global role: user (default) or admin';
COMMENT ON COLUMN users.is_active IS 'Soft delete flag - inactive users cannot login';

-- Projects table
CREATE TABLE projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    repository_path VARCHAR(500) NOT NULL UNIQUE,
    description TEXT,
    is_public BOOLEAN DEFAULT FALSE,
    last_scanned_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for projects
CREATE INDEX idx_projects_path ON projects(repository_path);
CREATE INDEX idx_projects_public ON projects(is_public) WHERE is_public = TRUE;
CREATE INDEX idx_projects_name ON projects(name);

COMMENT ON TABLE projects IS 'Git repositories detected and tracked by the system';
COMMENT ON COLUMN projects.repository_path IS 'Absolute path to the .git directory';
COMMENT ON COLUMN projects.is_public IS 'If true, all users can view (no need to be a member)';
COMMENT ON COLUMN projects.last_scanned_at IS 'Last time repository was scanned for commits';

-- Project members (many-to-many: users <-> projects)
CREATE TABLE project_members (
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    role VARCHAR(20) NOT NULL DEFAULT 'viewer', -- 'viewer' | 'contributor' | 'admin'
    created_at TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (project_id, user_id)
);

-- Indexes for project_members
CREATE INDEX idx_project_members_user ON project_members(user_id);
CREATE INDEX idx_project_members_project ON project_members(project_id);

COMMENT ON TABLE project_members IS 'User access control per project';
COMMENT ON COLUMN project_members.role IS 'viewer: read-only, contributor: can comment (Phase 2), admin: can manage members';

-- Commits cache (performance optimization)
CREATE TABLE commits (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    commit_hash VARCHAR(40) UNIQUE NOT NULL,
    author_name VARCHAR(255) NOT NULL,
    author_email VARCHAR(255),
    message TEXT NOT NULL,
    committed_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for commits
CREATE INDEX idx_commits_project_date ON commits(project_id, committed_at DESC);
CREATE INDEX idx_commits_hash ON commits(commit_hash);
CREATE INDEX idx_commits_author ON commits(author_email) WHERE author_email IS NOT NULL;

COMMENT ON TABLE commits IS 'Cached Git commits for faster queries and historical tracking';
COMMENT ON COLUMN commits.commit_hash IS 'Git SHA-1 hash (40 hex characters)';
COMMENT ON COLUMN commits.committed_at IS 'Commit timestamp from Git metadata';

-- ============================================
-- FASE 2 - COLLABORATION TABLES
-- ============================================

-- Comments (added in Phase 2)
CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    user_id INTEGER REFERENCES users(id) ON DELETE SET NULL,
    commit_hash VARCHAR(40),
    content TEXT NOT NULL,
    parent_comment_id INTEGER REFERENCES comments(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for comments
CREATE INDEX idx_comments_project ON comments(project_id, created_at DESC);
CREATE INDEX idx_comments_parent ON comments(parent_comment_id) WHERE parent_comment_id IS NOT NULL;
CREATE INDEX idx_comments_commit ON comments(commit_hash) WHERE commit_hash IS NOT NULL;

COMMENT ON TABLE comments IS 'User comments on projects or specific commits';
COMMENT ON COLUMN comments.commit_hash IS 'If set, comment is specific to this commit';
COMMENT ON COLUMN comments.parent_comment_id IS 'For threaded comments (1 level deep)';

-- Notifications (added in Phase 2)
CREATE TABLE notifications (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL, -- 'commit' | 'comment' | 'mention'
    title VARCHAR(255) NOT NULL,
    message TEXT NOT NULL,
    link VARCHAR(500),
    is_read BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for notifications
CREATE INDEX idx_notifications_user_unread ON notifications(user_id, is_read, created_at DESC);
CREATE INDEX idx_notifications_type ON notifications(type);

COMMENT ON TABLE notifications IS 'In-app notifications for users';
COMMENT ON COLUMN notifications.type IS 'Notification category for filtering and icons';
COMMENT ON COLUMN notifications.link IS 'Deep link to relevant content in the app';

-- ============================================
-- FASE 3 - TELEGRAM INTEGRATION
-- ============================================

-- Add telegram_chat_id to users (migration in Phase 3)
-- ALTER TABLE users ADD COLUMN telegram_chat_id BIGINT UNIQUE;
-- CREATE INDEX idx_users_telegram ON users(telegram_chat_id) WHERE telegram_chat_id IS NOT NULL;

-- ============================================
-- FASE 4 - SCRIPTS SYSTEM
-- ============================================

-- Scripts table (added in Phase 4)
CREATE TABLE scripts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    content TEXT NOT NULL,
    project_id INTEGER REFERENCES projects(id) ON DELETE CASCADE, -- NULL for global scripts
    created_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for scripts
CREATE INDEX idx_scripts_project ON scripts(project_id) WHERE project_id IS NOT NULL;
CREATE INDEX idx_scripts_global ON scripts(project_id) WHERE project_id IS NULL;
CREATE INDEX idx_scripts_active ON scripts(is_active) WHERE is_active = TRUE;

COMMENT ON TABLE scripts IS 'Custom scripts for execution';
COMMENT ON COLUMN scripts.project_id IS 'NULL = global script (admin only), otherwise project-specific';
COMMENT ON COLUMN scripts.is_active IS 'Soft delete - inactive scripts cannot be executed';

-- Script executions log
CREATE TABLE script_executions (
    id SERIAL PRIMARY KEY,
    script_id INTEGER REFERENCES scripts(id) ON DELETE CASCADE,
    executed_by INTEGER REFERENCES users(id) ON DELETE SET NULL,
    status VARCHAR(20) NOT NULL, -- 'running' | 'success' | 'failed' | 'timeout'
    exit_code INTEGER,
    output TEXT,
    started_at TIMESTAMP NOT NULL,
    finished_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for script_executions
CREATE INDEX idx_script_executions_script ON script_executions(script_id, started_at DESC);
CREATE INDEX idx_script_executions_user ON script_executions(executed_by, started_at DESC);
CREATE INDEX idx_script_executions_status ON script_executions(status);

COMMENT ON TABLE script_executions IS 'Execution history and logs for scripts';
COMMENT ON COLUMN script_executions.status IS 'Execution state machine: running -> success/failed/timeout';
COMMENT ON COLUMN script_executions.output IS 'Combined stdout and stderr from script execution';

-- ============================================
-- HELPER FUNCTIONS
-- ============================================

-- Function to update updated_at timestamp automatically
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Triggers for updated_at
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_projects_updated_at BEFORE UPDATE ON projects
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_comments_updated_at BEFORE UPDATE ON comments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_scripts_updated_at BEFORE UPDATE ON scripts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();