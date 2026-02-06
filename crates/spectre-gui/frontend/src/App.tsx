import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface VersionInfo {
  version: string;
  name: string;
  description: string;
}

interface ComponentStatus {
  name: string;
  status: string;
  version: string | null;
  details: string | null;
}

interface SystemStatus {
  components: ComponentStatus[];
  config_loaded: boolean;
}

function App() {
  const [version, setVersion] = useState<VersionInfo | null>(null);
  const [status, setStatus] = useState<SystemStatus | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function loadData() {
      try {
        const ver = await invoke<VersionInfo>("get_version");
        setVersion(ver);

        const st = await invoke<SystemStatus>("get_status");
        setStatus(st);
      } catch (e) {
        setError(String(e));
      }
    }
    loadData();
  }, []);

  if (error) {
    return (
      <div className="app error">
        <h1>SPECTRE</h1>
        <p className="error-text">Error: {error}</p>
      </div>
    );
  }

  return (
    <div className="app">
      <header className="header">
        <h1>{version?.name ?? "SPECTRE"}</h1>
        <span className="version">v{version?.version ?? "..."}</span>
      </header>

      <main className="main">
        <p className="description">{version?.description}</p>

        <section className="status-section">
          <h2>Component Status</h2>
          {status ? (
            <div className="component-grid">
              {status.components.map((c) => (
                <div key={c.name} className="component-card">
                  <div className="component-header">
                    <span className="component-name">{c.name}</span>
                    <span className={`status-badge ${c.status}`}>
                      {c.status}
                    </span>
                  </div>
                  {c.version && (
                    <span className="component-version">v{c.version}</span>
                  )}
                  {c.details && <p className="component-details">{c.details}</p>}
                </div>
              ))}
            </div>
          ) : (
            <p>Loading...</p>
          )}
        </section>
      </main>
    </div>
  );
}

export default App;
