import { useState, useEffect } from "react";
import type { McpServer, ApiToken, GenerateTokenRequest } from "../lib/types";
import { generateToken, listTokens, revokeToken } from "../lib/api";

interface TokensViewProps {
  servers: McpServer[];
  onRefresh: () => void;
}

export function TokensView({ servers, onRefresh }: TokensViewProps) {
  const [tokens, setTokens] = useState<ApiToken[]>([]);
  const [showGen, setShowGen] = useState(false);
  const [loading, setLoading] = useState(true);
  const [copied, setCopied] = useState<string | null>(null);

  const fetchTokens = async () => {
    setLoading(true);
    try {
      const t = await listTokens();
      setTokens(t);
    } catch (err) {
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchTokens();
  }, []);

  const handleRevoke = async (id: string) => {
    if (!confirm("Revoke this token?")) return;
    try {
      await revokeToken(id);
      await fetchTokens();
    } catch (err) {
      alert(String(err));
    }
  };

  const handleCopy = (text: string) => {
    navigator.clipboard.writeText(text);
    setCopied(text);
    setTimeout(() => setCopied(null), 2000);
  };

  return (
    <div className="animate-fade-in space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-xl font-semibold tracking-tight">API Tokens</h2>
          <p className="text-sm text-space-400 mt-0.5">
            Manage access tokens for AI tools
          </p>
        </div>
        <button onClick={() => setShowGen(true)} className="btn-primary">
          + Generate Token
        </button>
      </div>

      {showGen && (
        <GenerateTokenForm
          servers={servers}
          onClose={() => setShowGen(false)}
          onCreated={() => {
            setShowGen(false);
            fetchTokens();
          }}
        />
      )}

      {loading ? (
        <div className="card py-12 text-center">
          <div className="w-6 h-6 border-2 border-accent-cyan border-t-transparent rounded-full animate-spin mx-auto" />
        </div>
      ) : tokens.length === 0 ? (
        <div className="card text-center py-12">
          <p className="text-space-400 text-sm">No tokens generated yet.</p>
        </div>
      ) : (
        <div className="space-y-3">
          {tokens.map((token) => (
            <div key={token.id} className="card-hover">
              <div className="flex items-center justify-between">
                <div>
                  <div className="flex items-center gap-2">
                    <span className="font-mono text-sm text-accent-cyan">
                      {token.id.slice(0, 20)}...
                    </span>
                    <button
                      onClick={() => handleCopy(token.id)}
                      className="text-xs text-space-500 hover:text-white transition-colors"
                    >
                      {copied === token.id ? "✓ Copied" : "Copy"}
                    </button>
                  </div>
                  <p className="text-xs text-space-400 mt-1">
                    Client: <span className="text-space-300">{token.client_id}</span>
                    {" · "}
                    Issued: {new Date(token.issued_at * 1000).toLocaleDateString()}
                  </p>
                </div>
                <button
                  onClick={() => handleRevoke(token.id)}
                  className="btn-danger text-xs"
                >
                  Revoke
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}

function GenerateTokenForm({
  servers,
  onClose,
  onCreated,
}: {
  servers: McpServer[];
  onClose: () => void;
  onCreated: () => void;
}) {
  const [clientId, setClientId] = useState("");
  const [access, setAccess] = useState<Record<string, boolean>>({});
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<ApiToken | null>(null);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setLoading(true);
    try {
      const token = await generateToken({ client_id: clientId, server_access: access });
      setResult(token);
    } catch (err) {
      alert(String(err));
    } finally {
      setLoading(false);
    }
  };

  if (result) {
    return (
      <div className="card border-accent-green/30 space-y-3">
        <h3 className="text-sm font-medium text-accent-green">Token Generated!</h3>
        <div className="p-3 rounded bg-space-800 font-mono text-xs break-all text-accent-cyan">
          {result.id}
        </div>
        <p className="text-xs text-accent-orange">
          ⚠ Copy this token now. It won't be shown again.
        </p>
        <button onClick={onCreated} className="btn-primary w-full">
          Done
        </button>
      </div>
    );
  }

  return (
    <form onSubmit={handleSubmit} className="card space-y-4 border-accent-blue/30">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-medium">Generate Token</h3>
        <button type="button" onClick={onClose} className="text-space-500 hover:text-white text-xs">
          ✕
        </button>
      </div>

      <div>
        <label className="block text-xs text-space-400 mb-1">Client ID</label>
        <input
          className="input-field"
          value={clientId}
          onChange={(e) => setClientId(e.target.value)}
          placeholder="claude, cursor, cline, etc."
          required
        />
      </div>

      <div>
        <label className="block text-xs text-space-400 mb-2">Server Access</label>
        {servers.length === 0 ? (
          <p className="text-xs text-space-500">No servers to grant access to.</p>
        ) : (
          <div className="space-y-1.5">
            {servers.map((s) => (
              <label
                key={s.id}
                className="flex items-center gap-2 px-3 py-1.5 rounded bg-space-800/50 cursor-pointer"
              >
                <input
                  type="checkbox"
                  checked={access[s.id] ?? false}
                  onChange={(e) =>
                    setAccess({ ...access, [s.id]: e.target.checked })
                  }
                  className="rounded bg-space-700 border-space-600"
                />
                <span className="text-sm">{s.name}</span>
                <span className="text-xs text-space-500 ml-auto">{s.server_type}</span>
              </label>
            ))}
          </div>
        )}
      </div>

      <div className="flex justify-end gap-2">
        <button type="button" onClick={onClose} className="btn-secondary">
          Cancel
        </button>
        <button type="submit" disabled={loading || !clientId} className="btn-primary">
          {loading ? "Generating..." : "Generate"}
        </button>
      </div>
    </form>
  );
}
