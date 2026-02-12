-- Users table for authentication
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- Projects table
CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    optimization TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- MCP Servers table
CREATE TABLE IF NOT EXISTS servers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    server_type TEXT NOT NULL DEFAULT 'local',
    command TEXT,
    args TEXT,
    env TEXT,
    remote_url TEXT,
    bearer_token TEXT,
    project_id TEXT,
    tool_permissions TEXT,
    auto_start INTEGER NOT NULL DEFAULT 0,
    disabled INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE SET NULL
);

-- Request logs table
CREATE TABLE IF NOT EXISTS request_logs (
    id TEXT PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    client_id TEXT,
    client_name TEXT,
    server_id TEXT,
    server_name TEXT,
    request_type TEXT NOT NULL,
    request_params TEXT,
    response_status TEXT NOT NULL DEFAULT 'success',
    duration INTEGER,
    error_message TEXT
);

-- API tokens table
CREATE TABLE IF NOT EXISTS api_tokens (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    server_access TEXT NOT NULL DEFAULT '{}',
    issued_at INTEGER NOT NULL,
    expires_at INTEGER
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_request_logs_timestamp ON request_logs(timestamp);
CREATE INDEX IF NOT EXISTS idx_request_logs_server_id ON request_logs(server_id);
CREATE INDEX IF NOT EXISTS idx_request_logs_request_type ON request_logs(request_type);
CREATE INDEX IF NOT EXISTS idx_servers_project_id ON servers(project_id);
CREATE INDEX IF NOT EXISTS idx_api_tokens_client_id ON api_tokens(client_id);
