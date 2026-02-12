# MCP Router — Tauri/Rust Rewrite Plan

## Mål
Omskriv MCP Router fra Electron/React/TypeScript til Tauri/Rust/React som macOS DMG.

## Arkitektur
- **Backend**: Rust (Tauri commands)
- **Frontend**: React + Tailwind CSS (SpaceX-inspireret minimalistisk design)
- **Database**: rusqlite (SQLite)
- **Auth**: argon2 (password hashing) + session tokens
- **MCP Protocol**: Version 2025-11-25, JSON-RPC 2.0
- **Transports**: stdio + Streamable HTTP/HTTPS
- **Platform**: macOS kun (DMG)
- **Struktur**: Monorepo med pnpm workspaces

## Monorepo Struktur
```
mcp-router/
├── .plan/                    # Planer og status
├── apps/
│   └── mcp-router-app/      # Tauri desktop app
│       ├── src/              # React frontend
│       ├── src-tauri/        # Rust backend
│       └── package.json
├── packages/
│   └── shared/               # Shared types & utils
├── Cargo.toml                # Workspace root
├── package.json              # Root monorepo config
└── pnpm-workspace.yaml
```

## MVP Features
1. [x] Projekt-setup (Tauri + React + Rust)
2. [x] Auth-system (login/password med argon2)
3. [x] Server-styring (tilføj/fjern/start/stop MCP servere)
4. [x] Tool-styring per server
5. [x] Projekter og Workspaces
6. [x] Request logs og analytics
7. [x] Token-generering til AI-værktøjer
8. [x] MCP Protocol 2025-11-25 (stdio + HTTP/HTTPS)
9. [x] SpaceX-inspireret UI
10. [x] macOS DMG build konfigureret

## Build Status
- ✅ Rust backend kompilerer
- ✅ TypeScript kompilerer fejlfrit
- ✅ Vite frontend build OK
- ✅ Ingen kendte sikkerhedssårbarheder i dependencies
- ℹ️ DMG build kræver macOS (kører i Linux CI/sandbox)
