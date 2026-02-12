import { useState, useEffect, useCallback } from "react";
import {
  listServers,
  listProjects,
  getStats,
  getAllServerStatuses,
} from "../lib/api";
import type { McpServer, Project, AppStats, ServerStatus } from "../lib/types";
import { Sidebar } from "../components/Sidebar";
import { ServersView } from "../components/ServersView";
import { ProjectsView } from "../components/ProjectsView";
import { LogsView } from "../components/LogsView";
import { TokensView } from "../components/TokensView";
import { OverviewView } from "../components/OverviewView";

type View = "overview" | "servers" | "projects" | "logs" | "tokens";

interface DashboardProps {
  username: string;
  onLogout: () => void;
}

export function Dashboard({ username, onLogout }: DashboardProps) {
  const [activeView, setActiveView] = useState<View>("overview");
  const [servers, setServers] = useState<McpServer[]>([]);
  const [projects, setProjects] = useState<Project[]>([]);
  const [stats, setStats] = useState<AppStats | null>(null);
  const [statuses, setStatuses] = useState<Record<string, ServerStatus>>({});

  const refresh = useCallback(async () => {
    try {
      const [srvs, projs, st, sts] = await Promise.all([
        listServers(),
        listProjects(),
        getStats(),
        getAllServerStatuses(),
      ]);
      setServers(srvs);
      setProjects(projs);
      setStats(st);
      const statusMap: Record<string, ServerStatus> = {};
      sts.forEach((s) => (statusMap[s.server_id] = s));
      setStatuses(statusMap);
    } catch (err) {
      console.error("Failed to refresh:", err);
    }
  }, []);

  useEffect(() => {
    refresh();
    const interval = setInterval(refresh, 5000);
    return () => clearInterval(interval);
  }, [refresh]);

  return (
    <div className="min-h-screen flex bg-space-950">
      <Sidebar
        activeView={activeView}
        onViewChange={setActiveView}
        username={username}
        onLogout={onLogout}
        stats={stats}
      />
      <main className="flex-1 overflow-y-auto">
        <div className="p-6 max-w-6xl mx-auto">
          {activeView === "overview" && (
            <OverviewView stats={stats} servers={servers} statuses={statuses} onRefresh={refresh} />
          )}
          {activeView === "servers" && (
            <ServersView
              servers={servers}
              projects={projects}
              statuses={statuses}
              onRefresh={refresh}
            />
          )}
          {activeView === "projects" && (
            <ProjectsView
              projects={projects}
              servers={servers}
              onRefresh={refresh}
            />
          )}
          {activeView === "logs" && <LogsView />}
          {activeView === "tokens" && (
            <TokensView servers={servers} onRefresh={refresh} />
          )}
        </div>
      </main>
    </div>
  );
}
