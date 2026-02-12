import { useState } from "react";
import type { Project, McpServer, CreateProjectRequest } from "../lib/types";
import { createProject, deleteProject, updateProject } from "../lib/api";

interface ProjectsViewProps {
  projects: Project[];
  servers: McpServer[];
  onRefresh: () => void;
}

export function ProjectsView({ projects, servers, onRefresh }: ProjectsViewProps) {
  const [showAdd, setShowAdd] = useState(false);
  const [form, setForm] = useState<CreateProjectRequest>({ name: "" });
  const [loading, setLoading] = useState(false);

  const handleCreate = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      await createProject(form);
      setForm({ name: "" });
      setShowAdd(false);
      await onRefresh();
    } catch (err) {
      alert(String(err));
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async (id: string, name: string) => {
    if (!confirm(`Delete project "${name}"?`)) return;
    try {
      await deleteProject(id);
      await onRefresh();
    } catch (err) {
      alert(String(err));
    }
  };

  return (
    <div className="animate-fade-in space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">Projects</h2>
          <p className="text-sm text-space-400 mt-0.5">
            Organize servers into projects
          </p>
        </div>
        <button onClick={() => setShowAdd(true)} className="btn-primary">
          + New Project
        </button>
      </div>

      {showAdd && (
        <form onSubmit={handleCreate} className="card space-y-3 border-accent-blue/30">
          <div className="flex items-center justify-between">
            <h3 className="text-sm font-medium">New Project</h3>
            <button type="button" onClick={() => setShowAdd(false)} className="text-space-500 hover:text-white text-xs">
              ✕
            </button>
          </div>
          <input
            className="input-field"
            value={form.name}
            onChange={(e) => setForm({ ...form, name: e.target.value })}
            placeholder="Project name"
            required
            autoFocus
          />
          <div className="flex justify-end gap-2">
            <button type="button" onClick={() => setShowAdd(false)} className="btn-secondary">
              Cancel
            </button>
            <button type="submit" disabled={loading || !form.name} className="btn-primary">
              {loading ? "Creating..." : "Create"}
            </button>
          </div>
        </form>
      )}

      {projects.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-space-400 text-sm">No projects yet.</p>
        </div>
      ) : (
        <div className="space-y-3">
          {projects.map((project) => {
            const projectServers = servers.filter((s) => s.project_id === project.id);
            return (
              <div key={project.id} className="card-hover">
                <div className="flex items-center justify-between">
                  <div>
                    <h3 className="font-medium text-sm">{project.name}</h3>
                    <p className="text-xs text-space-500 mt-0.5">
                      {projectServers.length} server{projectServers.length !== 1 ? "s" : ""}
                    </p>
                  </div>
                  <div className="flex items-center gap-2">
                    {project.optimization && (
                      <span className="badge-info">{project.optimization}</span>
                    )}
                    <button
                      onClick={() => handleDelete(project.id, project.name)}
                      className="text-xs text-space-500 hover:text-accent-red transition-colors"
                    >
                      Delete
                    </button>
                  </div>
                </div>
                {projectServers.length > 0 && (
                  <div className="mt-3 pt-3 border-t border-space-800/50 space-y-1">
                    {projectServers.map((s) => (
                      <div key={s.id} className="flex items-center gap-2 text-xs text-space-400">
                        <span className={`w-1.5 h-1.5 rounded-full ${s.disabled ? "bg-space-600" : "bg-accent-green"}`} />
                        {s.name}
                      </div>
                    ))}
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
