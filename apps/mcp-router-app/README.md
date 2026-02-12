# MCP Router (Tauri/Rust)

A unified MCP (Model Context Protocol) server management desktop application built with **Tauri + Rust + React**.

## Architecture

- **Backend**: Rust
  - `rusqlite` — SQLite database with WAL mode
  - `argon2` — Password hashing for authentication
  - `reqwest` — HTTP client for remote MCP servers
  - `tokio` — Async runtime for MCP connections
  - MCP Protocol v2025-11-25 (JSON-RPC 2.0)

- **Frontend**: React 19 + Tailwind CSS
  - SpaceX-inspired minimalist dark design
  - Tauri API for Rust ↔ JS bridge

- **Distribution**: macOS DMG (via Tauri bundler)

## Features (MVP)

- 🔐 **Login/Password** — Argon2 password hashing
- ⬡ **Server Management** — Add, remove, start, stop MCP servers
- 🔧 **Tool Control** — Per-server tool permissions
- 📁 **Projects** — Organize servers into projects
- 📊 **Request Logs** — Full audit trail with pagination
- 🔑 **API Tokens** — Generate tokens for Claude, Cursor, Cline, etc.
- 🌐 **Dual Transport** — stdio (local) + HTTP/HTTPS (remote)

## Development

### Prerequisites

- Rust (1.70+)
- Node.js (18+)
- Tauri CLI (`cargo install tauri-cli`)
- macOS system libs (for Linux dev: `libwebkit2gtk-4.1-dev libgtk-3-dev`)

### Setup

```bash
cd apps/mcp-router-app
npm install
```

### Development Mode

```bash
cargo tauri dev
```

### Production Build (macOS DMG)

```bash
cargo tauri build
```

The DMG will be in `src-tauri/target/release/bundle/dmg/`.

## MCP Protocol Support

- **Protocol Version**: 2025-11-25
- **Transport**: stdio, Streamable HTTP/HTTPS
- **Features**: initialize, tools/list, tools/call
- **Auth**: Bearer token for remote servers

## Default Credentials

- Username: `admin`
- Password: `admin`

⚠️ Change the default password after first login!
