export interface McpServer {
  id: string;
  name: string;
  description?: string;
  server_type: "local" | "remote" | "remote-streamable";
  command?: string;
  args?: string;
  env?: string;
  remote_url?: string;
  bearer_token?: string;
  project_id?: string;
  tool_permissions?: string;
  auto_start: boolean;
  disabled: boolean;
  created_at: string;
  updated_at: string;
}

export interface Project {
  id: string;
  name: string;
  optimization?: string;
  created_at: string;
  updated_at: string;
}

export interface RequestLog {
  id: string;
  timestamp: number;
  client_id?: string;
  client_name?: string;
  server_id?: string;
  server_name?: string;
  request_type: string;
  request_params?: string;
  response_status: string;
  duration?: number;
  error_message?: string;
}

export interface ApiToken {
  id: string;
  client_id: string;
  server_access: string;
  issued_at: number;
}

export interface McpTool {
  name: string;
  description?: string;
  input_schema?: unknown;
}

export interface ServerStatus {
  server_id: string;
  is_running: boolean;
  tools: McpTool[];
  error?: string;
}

export interface LoginResponse {
  token: string;
  username: string;
}

export interface AppStats {
  total_servers: number;
  active_servers: number;
  total_requests: number;
  total_projects: number;
}

export interface LogQueryResult {
  items: RequestLog[];
  total: number;
  has_more: boolean;
}

// Request types
export interface CreateServerRequest {
  name: string;
  description?: string;
  server_type: "local" | "remote" | "remote-streamable";
  command?: string;
  args?: string;
  env?: string;
  remote_url?: string;
  bearer_token?: string;
  project_id?: string;
  auto_start?: boolean;
}

export interface UpdateServerRequest {
  id: string;
  name?: string;
  description?: string;
  command?: string;
  args?: string;
  env?: string;
  remote_url?: string;
  bearer_token?: string;
  project_id?: string;
  tool_permissions?: string;
  auto_start?: boolean;
  disabled?: boolean;
}

export interface CreateProjectRequest {
  name: string;
  optimization?: string;
}

export interface GenerateTokenRequest {
  client_id: string;
  server_access: Record<string, boolean>;
}

export interface LogQueryParams {
  limit?: number;
  offset?: number;
  server_id?: string;
  request_type?: string;
  response_status?: string;
}
