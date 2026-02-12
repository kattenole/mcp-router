use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServer {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub server_type: ServerType,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,
    pub remote_url: Option<String>,
    pub bearer_token: Option<String>,
    pub project_id: Option<String>,
    pub tool_permissions: Option<String>,
    pub auto_start: bool,
    pub disabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServerType {
    Local,
    Remote,
    RemoteStreamable,
}

impl std::fmt::Display for ServerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerType::Local => write!(f, "local"),
            ServerType::Remote => write!(f, "remote"),
            ServerType::RemoteStreamable => write!(f, "remote-streamable"),
        }
    }
}

impl std::str::FromStr for ServerType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(ServerType::Local),
            "remote" => Ok(ServerType::Remote),
            "remote-streamable" | "remote_streamable" => Ok(ServerType::RemoteStreamable),
            _ => Err(format!("Unknown server type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub optimization: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub id: String,
    pub timestamp: i64,
    pub client_id: Option<String>,
    pub client_name: Option<String>,
    pub server_id: Option<String>,
    pub server_name: Option<String>,
    pub request_type: String,
    pub request_params: Option<String>,
    pub response_status: String,
    pub duration: Option<i64>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiToken {
    pub id: String,
    pub client_id: String,
    pub server_access: String,
    pub issued_at: i64,
    pub expires_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub server_id: String,
    pub is_running: bool,
    pub tools: Vec<McpTool>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerWithStatus {
    pub server: McpServer,
    pub status: ServerStatus,
}

// Request/Response types for Tauri commands
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateServerRequest {
    pub name: String,
    pub description: Option<String>,
    pub server_type: ServerType,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,
    pub remote_url: Option<String>,
    pub bearer_token: Option<String>,
    pub project_id: Option<String>,
    pub auto_start: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateServerRequest {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub command: Option<String>,
    pub args: Option<String>,
    pub env: Option<String>,
    pub remote_url: Option<String>,
    pub bearer_token: Option<String>,
    pub project_id: Option<String>,
    pub tool_permissions: Option<String>,
    pub auto_start: Option<bool>,
    pub disabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub optimization: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateTokenRequest {
    pub client_id: String,
    pub server_access: std::collections::HashMap<String, bool>,
    /// Token TTL in seconds. Defaults to 90 days if not specified.
    pub expires_in_seconds: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct LogQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub server_id: Option<String>,
    pub request_type: Option<String>,
    pub response_status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LogQueryResult {
    pub items: Vec<RequestLog>,
    pub total: i64,
    pub has_more: bool,
}

#[derive(Debug, Serialize)]
pub struct AppStats {
    pub total_servers: i64,
    pub active_servers: i64,
    pub total_requests: i64,
    pub total_projects: i64,
}
