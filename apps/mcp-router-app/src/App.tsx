import { useState, useEffect } from "react";
import { LoginPage } from "./pages/LoginPage";
import { Dashboard } from "./pages/Dashboard";
import { validateSession, logoutSession } from "./lib/api";

type AuthState = {
  authenticated: boolean;
  username: string | null;
};

export default function App() {
  const [auth, setAuth] = useState<AuthState>({
    authenticated: false,
    username: null,
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const checkSession = async () => {
      const token = sessionStorage.getItem("mcpr_token");
      if (token) {
        try {
          const result = await validateSession(token);
          if (result.valid && result.username) {
            setAuth({ authenticated: true, username: result.username });
          } else {
            sessionStorage.removeItem("mcpr_token");
            sessionStorage.removeItem("mcpr_username");
          }
        } catch {
          sessionStorage.removeItem("mcpr_token");
          sessionStorage.removeItem("mcpr_username");
        }
      }
      setLoading(false);
    };
    checkSession();
  }, []);

  const handleLogin = (username: string, token: string) => {
    sessionStorage.setItem("mcpr_token", token);
    sessionStorage.setItem("mcpr_username", username);
    setAuth({ authenticated: true, username });
  };

  const handleLogout = async () => {
    const token = sessionStorage.getItem("mcpr_token");
    if (token) {
      try {
        await logoutSession(token);
      } catch {
        // Proceed with local logout even if server-side cleanup fails
      }
    }
    sessionStorage.removeItem("mcpr_token");
    sessionStorage.removeItem("mcpr_username");
    setAuth({ authenticated: false, username: null });
  };

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-space-950">
        <div className="flex flex-col items-center gap-4">
          <div className="w-8 h-8 border-2 border-accent-cyan border-t-transparent rounded-full animate-spin" />
          <p className="text-space-400 text-sm">Loading...</p>
        </div>
      </div>
    );
  }

  if (!auth.authenticated) {
    return <LoginPage onLogin={handleLogin} />;
  }

  return <Dashboard username={auth.username!} onLogout={handleLogout} />;
}
