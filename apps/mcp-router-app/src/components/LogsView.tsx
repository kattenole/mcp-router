import { useState, useEffect } from "react";
import type { RequestLog, LogQueryResult } from "../lib/types";
import { queryLogs, clearLogs } from "../lib/api";

export function LogsView() {
  const [logs, setLogs] = useState<RequestLog[]>([]);
  const [total, setTotal] = useState(0);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(0);
  const pageSize = 25;

  const fetchLogs = async (offset = 0) => {
    setLoading(true);
    try {
      const result = await queryLogs({ limit: pageSize, offset });
      setLogs(result.items);
      setTotal(result.total);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchLogs(page * pageSize);
  }, [page]);

  const handleClear = async () => {
    if (!confirm("Clear all logs?")) return;
    try {
      await clearLogs();
      setLogs([]);
      setTotal(0);
      setPage(0);
    } catch (err) {
      alert(String(err));
    }
  };

  return (
    <div className="animate-fade-in space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">Request Logs</h2>
          <p className="text-sm text-space-400 mt-0.5">
            {total} total log entries
          </p>
        </div>
        <div className="flex gap-2">
          <button onClick={() => fetchLogs(page * pageSize)} className="btn-secondary text-xs">
            ↻ Refresh
          </button>
          <button onClick={handleClear} className="btn-danger text-xs">
            Clear All
          </button>
        </div>
      </div>

      <div className="card overflow-hidden p-0">
        {loading ? (
          <div className="py-12 text-center">
            <div className="w-6 h-6 border-2 border-accent-cyan border-t-transparent rounded-full animate-spin mx-auto" />
          </div>
        ) : logs.length === 0 ? (
          <div className="py-12 text-center text-space-500 text-sm">
            No log entries yet.
          </div>
        ) : (
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-space-700 text-space-400 text-xs">
                <th className="px-4 py-3 text-left font-medium">Time</th>
                <th className="px-4 py-3 text-left font-medium">Type</th>
                <th className="px-4 py-3 text-left font-medium">Server</th>
                <th className="px-4 py-3 text-left font-medium">Status</th>
                <th className="px-4 py-3 text-right font-medium">Duration</th>
              </tr>
            </thead>
            <tbody>
              {logs.map((log) => (
                <tr
                  key={log.id}
                  className="border-b border-space-800/50 hover:bg-space-800/30 transition-colors"
                >
                  <td className="px-4 py-2.5 font-mono text-xs text-space-400">
                    {new Date(log.timestamp).toLocaleTimeString()}
                  </td>
                  <td className="px-4 py-2.5">
                    <span className="badge-info">{log.request_type}</span>
                  </td>
                  <td className="px-4 py-2.5 text-space-300">
                    {log.server_name ?? "-"}
                  </td>
                  <td className="px-4 py-2.5">
                    <span
                      className={
                        log.response_status === "success"
                          ? "badge-success"
                          : "badge-error"
                      }
                    >
                      {log.response_status}
                    </span>
                  </td>
                  <td className="px-4 py-2.5 text-right font-mono text-xs text-space-400">
                    {log.duration != null ? `${log.duration}ms` : "-"}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        )}
      </div>

      {/* Pagination */}
      {total > pageSize && (
        <div className="flex items-center justify-center gap-2">
          <button
            onClick={() => setPage(Math.max(0, page - 1))}
            disabled={page === 0}
            className="btn-secondary text-xs disabled:opacity-30"
          >
            ← Previous
          </button>
          <span className="text-xs text-space-400">
            Page {page + 1} of {Math.ceil(total / pageSize)}
          </span>
          <button
            onClick={() => setPage(page + 1)}
            disabled={(page + 1) * pageSize >= total}
            className="btn-secondary text-xs disabled:opacity-30"
          >
            Next →
          </button>
        </div>
      )}
    </div>
  );
}
