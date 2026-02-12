import { useState } from "react";
import type { McpServer, Project, ServerStatus, CreateServerRequest } from "../lib/types";
import {
  createServer,
  deleteServer,
  startServer,
  stopServer,
  toggleServer,
  updateToolPermissions,
} from "../lib/api";

interface ServersViewProps {
  servers: McpServer[];
  projects: Project[];
  statuses: Record<string, ServerStatus>;
  onRefresh: () => void;
}

export function ServersView({ servers, projects, statuses, onRefresh }: ServersViewProps) {
  const [showAdd, setShowAdd] = useState(false);
  const [actionLoading, setActionLoading] = useState<string | null>(null);

  const handleStartStop = async (serverId: string, isRunning: boolean) => {
    setActionLoading(serverId);
    try {
      if (isRunning) {
        await stopServer(serverId);
      } else {
        await startServer(serverId);
      }
      await onRefresh();
    } catch (err) {
      alert(String(err));
    } finally {
      setActionLoading(null);
    }
  };

  const handleDelete = async (serverId: string, name: string) => {
    if (!confirm(`Delete server "${name}"?`)) return;
    try {
      await deleteServer(serverId);
      await onRefresh();
    } catch (err) {
      alert(String(err));
    }
  };

  const handleToggle = async (serverId: string, disabled: boolean) => {
    try {
      await toggleServer(serverId, disabled);
      await onRefresh();
    } catch (err) {
      alert(String(err));
    }
  };

  return (
    <div className="animate-fade-in space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">Servers</h2>
          <p className="text-sm text-space-400 mt-0.5">
            Manage MCP server connections
          </p>
        </div>
        <button onClick={() => setShowAdd(true)} className="btn-primary">
          + Add Server
        </button>
      </div>

      {showAdd && (
        <AddServerForm
          projects={projects}
          onClose={() => setShowAdd(false)}
          onCreated={() => {
            setShowAdd(false);
            onRefresh();
          }}
        />
      )}

      {servers.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-space-400 text-sm">No servers configured yet.</p>
          <button
            onClick={() => setShowAdd(true)}
            className="btn-primary mt-4"
          >
            Add Your First Server
          </button>
        </div>
      ) : (
        <div className="space-y-3">
          {servers.map((server) => {
            const status = statuses[server.id];
            const running = status?.is_running ?? false;
            const loading = actionLoading === server.id;

            return (
              <div key={server.id} className="card-hover">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div
                      className={`w-3 h-3 rounded-full transition-colors ${
                        running
                          ? "bg-accent-green animate-pulse-slow"
                          : server.disabled
                          ? "bg-space-600"
                          : "bg-accent-orange"
                      }`}
                    />
                    <div>
                      <h3 className="font-medium text-sm">{server.name}</h3>
                      <div className="flex items-center gap-2 mt-0.5">
                        <span className="badge-info">{server.server_type}</span>
                        {server.description && (
                          <span className="text-xs text-space-500">
                            {server.description}
                          </span>
                        )}
                      </div>
                    </div>
                  </div>

                  <div className="flex items-center gap-2">
                    {running && status.tools.length > 0 && (
                      <span className="text-xs text-space-400">
                        {status.tools.length} tools
                      </span>
                    )}

                    <button
                      onClick={() => handleStartStop(server.id, running)}
                      disabled={loading || server.disabled}
                      className={`px-3 py-1 rounded text-xs font-medium transition-all ${
                        running
                          ? "bg-accent-red/10 text-accent-red hover:bg-accent-red/20"
                          : "bg-accent-green/10 text-accent-green hover:bg-accent-green/20"
                      } disabled:opacity-50`}
                    >
                      {loading ? "..." : running ? "Stop" : "Start"}
                    </button>

                    <button
                      onClick={() => handleToggle(server.id, !server.disabled)}
                      className={`px-2 py-1 rounded text-xs transition-all ${
                        server.disabled
                          ? "text-space-500 hover:text-white"
                          : "text-accent-cyan hover:text-accent-cyan/80"
                      }`}
                      title={server.disabled ? "Enable" : "Disable"}
                    >
                      {server.disabled ? "Enable" : "Disable"}
                    </button>

                    <button
                      onClick={() => handleDelete(server.id, server.name)}
                      className="px-2 py-1 rounded text-xs text-space-500 hover:text-accent-red transition-colors"
                    >
                      ✕
                    </button>
                  </div>
                </div>

                {/* Connection details */}
                <div className="mt-3 pt-3 border-t border-space-800/50">
                  <div className="flex gap-4 text-xs text-space-500">
                    {server.command && (
                      <span className="font-mono">
                        $ {server.command}
                        {server.args ? ` ${JSON.parse(server.args).join(" ")}` : ""}
                      </span>
                    )}
                    {server.remote_url && (
                      <span className="font-mono">{server.remote_url}</span>
                    )}
                    {server.auto_start && (
                      <span className="badge-info">auto-start</span>
                    )}
                  </div>
                </div>

                {/* Tools list */}
                {running && status.tools.length > 0 && (
                  <div className="mt-3 pt-3 border-t border-space-800/50">
                    <p className="text-xs text-space-400 mb-2">Available Tools:</p>
                    <div className="flex flex-wrap gap-1.5">
                      {status.tools.map((tool) => (
                        <span
                          key={tool.name}
                          className="px-2 py-0.5 rounded bg-space-800 text-xs text-space-300 font-mono"
                          title={tool.description ?? ""}
                        >
                          {tool.name}
                        </span>
                      ))}
                    </div>
                  </div>
                )}
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
}

function AddServerForm({
  projects,
  onClose,
  onCreated,
}: {
  projects: Project[];
  onClose: () => void;
  onCreated: () => void;
}) {
  const [form, setForm] = useState<CreateServerRequest>({
    name: "",
    server_type: "local",
  });
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");
    setLoading(true);
    try {
      await createServer(form);
      onCreated();
    } catch (err) {
      setError(String(err));
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="card space-y-4 border-accent-blue/30">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium">Add New Server</h3>
        <button type="button" onClick={onClose} className="text-space-500 hover:text-white text-xs">
          ✕ Close
        </button>
      </div>

      {error && (
        <div className="px-3 py-2 rounded-lg bg-accent-red/10 border border-accent-red/30 text-accent-red text-sm">
          {error}
        </div>
      )}

      <div className="grid grid-cols-2 gap-4">
        <div>
          <label className="block text-xs text-space-400 mb-1">Name</label>
          <input
            className="input-field"
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
            placeholder="My MCP Server"
            required
          />
        </div>
        <div>
          <label className="block text-xs text-space-400 mb-1">Type</label>
          <select
            className="input-field"
            value={form.server_type}
            onChange={(e) =>
              setForm({
                ...form,
                server_type: e.target.value as CreateServerRequest["server_type"],
              })
            }
          >
            <option value="local">Local (stdio)</option>
            <option value="remote">Remote (SSE)</option>
            <option value="remote-streamable">Remote (HTTP)</option>
          </select>
        </div>
      </div>

      <div>
        <label className="block text-xs text-space-400 mb-1">Description</label>
        <input
          className="input-field"
          value={form.description ?? ""}
          onChange={(e) => setForm({ ...form, description: e.target.value })}
          placeholder="Optional description"
        />
      </div>

      {form.server_type === "local" ? (
        <>
          <div>
            <label className="block text-xs text-space-400 mb-1">Command</label>
            <input
              className="input-field font-mono"
              value={form.command ?? ""}
              onChange={(e) => setForm({ ...form, command: e.target.value })}
              placeholder="npx, node, python, etc."
              required
            />
          </div>
          <div>
            <label className="block text-xs text-space-400 mb-1">Arguments (JSON array)</label>
            <input
              className="input-field font-mono"
              value={form.args ?? ""}
              onChange={(e) => setForm({ ...form, args: e.target.value })}
              placeholder='["-y", "@modelcontextprotocol/server-everything"]'
            />
          </div>
        </>
      ) : (
        <>
          <div>
            <label className="block text-xs text-space-400 mb-1">Remote URL</label>
            <input
              className="input-field font-mono"
              value={form.remote_url ?? ""}
              onChange={(e) => setForm({ ...form, remote_url: e.target.value })}
              placeholder="https://mcp-server.example.com/mcp"
              required
            />
          </div>
          <div>
            <label className="block text-xs text-space-400 mb-1">Bearer Token</label>
            <input
              className="input-field font-mono"
              value={form.bearer_token ?? ""}
              onChange={(e) => setForm({ ...form, bearer_token: e.target.value })}
              placeholder="Optional authentication token"
              type="password"
            />
          </div>
        </>
      )}

      {projects.length > 0 && (
        <div>
          <label className="block text-xs text-space-400 mb-1">Project</label>
          <select
            className="input-field"
            value={form.project_id ?? ""}
            onChange={(e) => setForm({ ...form, project_id: e.target.value || undefined })}
          >
            <option value="">No project</option>
            {projects.map((p) => (
              <option key={p.id} value={p.id}>
                {p.name}
              </option>
            ))}
          </select>
        </div>
      )}

      <div className="flex items-center gap-2">
        <input
          type="checkbox"
          id="auto_start"
          checked={form.auto_start ?? false}
          onChange={(e) => setForm({ ...form, auto_start: e.target.checked })}
          className="rounded bg-space-800 border-space-600"
        />
        <label htmlFor="auto_start" className="text-xs text-space-400">
          Auto-start on launch
        </label>
      </div>

      <div className="flex justify-end gap-2">
        <button type="button" onClick={onClose} className="btn-secondary">
          Cancel
        </button>
        <button type="submit" disabled={loading || !form.name} className="btn-primary">
          {loading ? "Creating..." : "Create Server"}
        </button>
      </div>
    </form>
  );
}
