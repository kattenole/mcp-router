import { invoke } from "@tauri-apps/api/core";
import type {
  McpServer,
  Project,
  ApiToken,
  ServerStatus,
  McpTool,
  LoginResponse,
  LogQueryResult,
  AppStats,
  CreateServerRequest,
  UpdateServerRequest,
  CreateProjectRequest,
  GenerateTokenRequest,
  LogQueryParams,
} from "./types";

// Auth
export const login = (username: string, password: string) =>
  invoke<LoginResponse>("login", { username, password });

export const changePassword = (
  username: string,
  oldPassword: string,
  newPassword: string
) =>
  invoke<void>("change_password", {
    username,
    oldPassword,
    newPassword,
  });

export const hasUsers = () => invoke<boolean>("has_users");

// Servers
export const createServer = (req: CreateServerRequest) =>
  invoke<McpServer>("create_server", { req });

export const listServers = () => invoke<McpServer[]>("list_servers");

export const getServer = (serverId: string) =>
  invoke<McpServer>("get_server", { serverId });

export const updateServer = (req: UpdateServerRequest) =>
  invoke<McpServer>("update_server", { req });

export const deleteServer = (serverId: string) =>
  invoke<void>("delete_server", { serverId });

export const toggleServer = (serverId: string, disabled: boolean) =>
  invoke<McpServer>("toggle_server", { serverId, disabled });

export const updateToolPermissions = (
  serverId: string,
  permissions: string
) => invoke<void>("update_tool_permissions", { serverId, permissions });

// MCP Connections
export const startServer = (serverId: string) =>
  invoke<ServerStatus>("start_server", { serverId });

export const stopServer = (serverId: string) =>
  invoke<void>("stop_server", { serverId });

export const getServerStatus = (serverId: string) =>
  invoke<ServerStatus>("get_server_status", { serverId });

export const listServerTools = (serverId: string) =>
  invoke<McpTool[]>("list_server_tools", { serverId });

export const callTool = (
  serverId: string,
  toolName: string,
  args?: Record<string, unknown>
) =>
  invoke<unknown>("call_tool", {
    serverId,
    toolName,
    arguments: args,
  });

export const getAllServerStatuses = () =>
  invoke<ServerStatus[]>("get_all_server_statuses");

// Projects
export const createProject = (req: CreateProjectRequest) =>
  invoke<Project>("create_project", { req });

export const listProjects = () => invoke<Project[]>("list_projects");

export const getProject = (projectId: string) =>
  invoke<Project>("get_project", { projectId });

export const deleteProject = (projectId: string) =>
  invoke<void>("delete_project", { projectId });

export const updateProject = (
  projectId: string,
  name?: string,
  optimization?: string
) => invoke<Project>("update_project", { projectId, name, optimization });

// Tokens
export const generateToken = (req: GenerateTokenRequest) =>
  invoke<ApiToken>("generate_token", { req });

export const listTokens = () => invoke<ApiToken[]>("list_tokens");

export const revokeToken = (tokenId: string) =>
  invoke<void>("revoke_token", { tokenId });

// Logging
export const queryLogs = (params: LogQueryParams) =>
  invoke<LogQueryResult>("query_logs", { params });

export const clearLogs = () => invoke<void>("clear_logs");

export const getStats = () => invoke<AppStats>("get_stats");
