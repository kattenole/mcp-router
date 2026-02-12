import { useState } from "react";
import { login } from "../lib/api";

interface LoginPageProps {
  onLogin: (username: string, token: string) => void;
}

export function LoginPage({ onLogin }: LoginPageProps) {
  const [username, setUsername] = useState("admin");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);

    try {
      const result = await login(username, password);
      onLogin(result.username, result.token);
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen flex items-center justify-center bg-space-950 p-4">
      <div className="w-full max-w-sm animate-fade-in">
        {/* Logo / Brand */}
        <div className="text-center mb-8">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-space-800 border border-space-600 mb-4">
            <svg
              className="w-8 h-8 text-accent-cyan"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
              strokeWidth={1.5}
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                d="M7.5 21L3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"
              />
            </svg>
          </div>
          <h1 className="text-2xl font-semibold tracking-tight">MCP Router</h1>
          <p className="text-space-400 text-sm mt-1">
            Unified MCP Server Management
          </p>
        </div>

        {/* Login Form */}
        <form onSubmit={handleSubmit} className="card space-y-4">
          {error && (
            <div className="px-3 py-2 rounded-lg bg-accent-red/10 border border-accent-red/30 text-accent-red text-sm">
              {error}
            </div>
          )}

          <div>
            <label className="block text-sm font-medium text-space-300 mb-1.5">
              Username
            </label>
            <input
              type="text"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              className="input-field"
              placeholder="admin"
              autoFocus
            />
          </div>

          <div>
            <label className="block text-sm font-medium text-space-300 mb-1.5">
              Password
            </label>
            <input
              type="password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              className="input-field"
              placeholder="••••••••"
            />
          </div>

          <button
            type="submit"
            disabled={loading || !username || !password}
            className="btn-primary w-full flex items-center justify-center gap-2"
          >
            {loading ? (
              <div className="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
            ) : (
              "Sign In"
            )}
          </button>

          <p className="text-center text-space-500 text-xs">
            Default: admin / admin
          </p>
        </form>

        <p className="text-center text-space-600 text-xs mt-6">
          MCP Protocol v2025-11-25 · Built with Tauri + Rust
        </p>
      </div>
    </div>
  );
}
