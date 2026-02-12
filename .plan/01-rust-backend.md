# Rust Backend Modules

## Status: ✅ Alle moduler implementeret og kompilerer

### auth/ (Autentificering)
- [x] Argon2 password hashing (argon2 v0.5.3)
- [x] Login med username/password
- [x] Default admin bruger (admin/admin)
- [x] Skift password
- [x] Session token generering

### db/ (Database)
- [x] rusqlite v0.32 med WAL journal mode
- [x] Migrations-system med versionering
- [x] 001_initial.sql: users, projects, servers, request_logs, api_tokens
- [x] Indexes for performance

### mcp/ (MCP Protocol)
- [x] JSON-RPC 2.0 protocol typer
- [x] MCP Protocol v2025-11-25
- [x] stdio transport (lokale processer)
- [x] HTTP/HTTPS Streamable transport (remote servere)
- [x] initialize/initialized handshake
- [x] tools/list og tools/call
- [x] Bearer token autentificering for remote servere

### server/ (Server Management)
- [x] CRUD for MCP servere
- [x] Start/Stop med MCP client forbindelse
- [x] Toggle enable/disable
- [x] Tool permissions per server
- [x] Project-tilknytning

### server/projects.rs (Projekter)
- [x] CRUD for projekter
- [x] Server-projekt association
- [x] Kaskade-sletning (unassign servere)

### token/ (API Tokens)
- [x] Token-generering (mcpr_ prefix, base64url)
- [x] Server-adgang per token
- [x] Revoke/list tokens
- [x] Validering

### logging/ (Request Logs)
- [x] Log alle operationer
- [x] Pagineret forespørgsel med filtre
- [x] Statistik (total servers, aktive, requests, projects)
- [x] Ryd logs

### models/ (Data Modeller)
- [x] User, McpServer, Project, RequestLog, ApiToken
- [x] ServerType (Local, Remote, RemoteStreamable)
- [x] Request/Response typer for alle Tauri commands
- [x] McpTool, ServerStatus, AppStats
