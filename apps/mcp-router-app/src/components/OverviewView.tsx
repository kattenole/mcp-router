import type { AppStats, McpServer, ServerStatus } from "../lib/types";

interface OverviewViewProps {
  stats: AppStats | null;
  servers: McpServer[];
  statuses: Record<string, ServerStatus>;
  onRefresh: () => void;
}

export function OverviewView({ stats, servers, statuses, onRefresh }: OverviewViewProps) {
  const runningServers = Object.values(statuses).filter((s) => s.is_running).length;
  const totalTools = Object.values(statuses).reduce(
    (acc, s) => acc + s.tools.length,
    0
  );

  return (
    <div className="animate-fade-in space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">Dashboard</h2>
          <p className="text-sm text-space-400 mt-0.5">
            MCP Router system overview
          </p>
        </div>
        <button onClick={onRefresh} className="btn-secondary text-xs">
          ↻ Refresh
        </button>
      </div>

      {/* Stats Grid */}
      <div className="grid grid-cols-4 gap-4">
        <StatCard label="Total Servers" value={stats?.total_servers ?? 0} color="cyan" />
        <StatCard label="Active" value={runningServers} color="green" />
        <StatCard label="Total Tools" value={totalTools} color="blue" />
        <StatCard label="Requests" value={stats?.total_requests ?? 0} color="orange" />
      </div>

      {/* Quick Server Status */}
      <div className="card">
        <h3 className="text-sm font-medium text-space-300 mb-3">Server Status</h3>
        {servers.length === 0 ? (
          <p className="text-sm text-space-500 py-4 text-center">
            No servers configured. Add a server to get started.
          </p>
        ) : (
          <div className="space-y-2">
            {servers.map((server) => {
              const status = statuses[server.id];
              const running = status?.is_running ?? false;
              return (
                <div
                  key={server.id}
                  className="flex items-center justify-between py-2 px-3 rounded-lg bg-space-800/30"
                >
                  <div className="flex items-center gap-3">
                    <div
                      className={`w-2 h-2 rounded-full ${
                        running ? "bg-accent-green animate-pulse-slow" : "bg-space-600"
                      }`}
                    />
                    <div>
                      <p className="text-sm font-medium">{server.name}</p>
                      <p className="text-xs text-space-500">{server.server_type}</p>
                    </div>
                  </div>
                  <div className="flex items-center gap-2">
                    {running && (
                      <span className="text-xs text-space-400">
                        {status.tools.length} tools
                      </span>
                    )}
                    <span
                      className={running ? "badge-success" : "badge-error"}
                    >
                      {running ? "Running" : "Stopped"}
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        )}
      </div>

      {/* Protocol Info */}
      <div className="card">
        <h3 className="text-sm font-medium text-space-300 mb-2">System Info</h3>
        <div className="grid grid-cols-2 gap-4 text-sm">
          <InfoRow label="MCP Protocol" value="v2025-11-25" />
          <InfoRow label="Transport" value="stdio + HTTP/HTTPS" />
          <InfoRow label="Backend" value="Rust + Tauri" />
          <InfoRow label="Database" value="SQLite (rusqlite)" />
        </div>
      </div>
    </div>
  );
}

function StatCard({
  label,
  value,
  color,
}: {
  label: string;
  value: number;
  color: string;
}) {
  const colorMap: Record<string, string> = {
    cyan: "text-accent-cyan border-accent-cyan/20 bg-accent-cyan/5",
    green: "text-accent-green border-accent-green/20 bg-accent-green/5",
    blue: "text-accent-blue border-accent-blue/20 bg-accent-blue/5",
    orange: "text-accent-orange border-accent-orange/20 bg-accent-orange/5",
  };

  return (
    <div className={`rounded-xl border p-4 ${colorMap[color] ?? ""}`}>
      <p className="text-2xl font-bold">{value}</p>
      <p className="text-xs opacity-60 mt-1">{label}</p>
    </div>
  );
}

function InfoRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between items-center py-1.5 px-3 rounded bg-space-800/30">
      <span className="text-space-400">{label}</span>
      <span className="font-mono text-xs">{value}</span>
    </div>
  );
}
