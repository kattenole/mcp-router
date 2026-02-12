import type { AppStats } from "../lib/types";

type View = "overview" | "servers" | "projects" | "logs" | "tokens";

interface SidebarProps {
  activeView: View;
  onViewChange: (view: View) => void;
  username: string;
  onLogout: () => void;
  stats: AppStats | null;
}

const navItems: { id: View; label: string; icon: string }[] = [
  { id: "overview", label: "Overview", icon: "◎" },
  { id: "servers", label: "Servers", icon: "⬡" },
  { id: "projects", label: "Projects", icon: "◫" },
  { id: "logs", label: "Logs", icon: "☰" },
  { id: "tokens", label: "Tokens", icon: "⚿" },
];

export function Sidebar({
  activeView,
  onViewChange,
  username,
  onLogout,
  stats,
}: SidebarProps) {
  return (
    <aside className="w-56 min-h-screen bg-space-900/30 border-r border-space-800 flex flex-col">
      {/* Brand */}
      <div className="p-4 border-b border-space-800">
        <div className="flex items-center gap-2.5">
          <div className="w-8 h-8 rounded-lg bg-accent-cyan/10 border border-accent-cyan/30 flex items-center justify-center">
            <span className="text-accent-cyan text-sm font-bold">⟡</span>
          </div>
          <div>
            <h1 className="text-sm font-semibold tracking-tight">MCP Router</h1>
            <p className="text-[10px] text-space-500 uppercase tracking-widest">v0.1.0</p>
          </div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 p-2 space-y-0.5">
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onViewChange(item.id)}
            className={`w-full flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm transition-all duration-150 ${
              activeView === item.id
                ? "bg-accent-blue/10 text-accent-cyan border border-accent-blue/30"
                : "text-space-400 hover:text-white hover:bg-space-800/50"
            }`}
          >
            <span className="text-base w-5 text-center opacity-70">{item.icon}</span>
            <span>{item.label}</span>
            {item.id === "servers" && stats && (
              <span className="ml-auto text-xs text-space-500">{stats.total_servers}</span>
            )}
          </button>
        ))}
      </nav>

      {/* User */}
      <div className="p-3 border-t border-space-800">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-2">
            <div className="w-6 h-6 rounded-full bg-space-700 flex items-center justify-center">
              <span className="text-xs">{username[0].toUpperCase()}</span>
            </div>
            <span className="text-xs text-space-400">{username}</span>
          </div>
          <button
            onClick={onLogout}
            className="text-xs text-space-500 hover:text-accent-red transition-colors"
            title="Sign out"
          >
            ✕
          </button>
        </div>
      </div>
    </aside>
  );
}
