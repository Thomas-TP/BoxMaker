import {
  ArrowUpRight,
  CheckCircle2,
  Download,
  RefreshCw,
  Save,
} from "lucide-react";
import { useState } from "react";

interface UpdateView {
  state: "unavailable" | "available" | "current" | "empty";
  message: string;
  version: string | null;
  notes: string | null;
}

export function Updates({
  saveProject,
  canSave,
}: {
  saveProject: () => Promise<boolean>;
  canSave: boolean;
}) {
  const [result, setResult] = useState<UpdateView | null>(null);
  const [busy, setBusy] = useState(false);
  const [downloaded, setDownloaded] = useState(false);
  const [error, setError] = useState("");
  async function check() {
    setBusy(true);
    setError("");
    try {
      if (!("__TAURI_INTERNALS__" in window)) {
        setResult({
          state: "unavailable",
          message:
            "Les mises à jour intégrées sont disponibles dans l’application Windows installée.",
          version: null,
          notes: null,
        });
        return;
      }
      const { invoke } = await import("@tauri-apps/api/core");
      setResult(await invoke<UpdateView>("check_update"));
      setDownloaded(false);
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  async function update(saveFirst = false) {
    setBusy(true);
    setError("");
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      if (!downloaded) {
        await invoke("download_update");
        setDownloaded(true);
      } else {
        if (saveFirst && !(await saveProject())) return;
        await invoke("install_update");
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setBusy(false);
    }
  }
  return (
    <section className="update-panel">
      <div className="eyebrow">TOUJOURS À JOUR</div>
      <h2>Mises à jour</h2>
      <p className="dialog-intro">
        Les nouveautés de Boxmaker, quand vous le décidez.
      </p>
      <div className="installed-version">
        <span className="update-icon">
          <RefreshCw size={24} />
        </span>
        <div>
          <strong>Boxmaker {__APP_VERSION__}</strong>
          <span>Version installée · préversion</span>
        </div>
        <span className="version">Windows</span>
      </div>
      <p className="update-source">
        <span>Versions publiées par Swiss3Design</span>
        <a
          href="https://github.com/Thomas-TP/BoxMaker/releases"
          target="_blank"
          rel="noreferrer"
        >
          Notes de version <ArrowUpRight size={13} />
        </a>
      </p>
      <button
        type="button"
        className="outlined update-check"
        disabled={busy}
        onClick={() => void check()}
      >
        <RefreshCw size={14} className={busy ? "spin" : ""} />
        {busy ? "Opération en cours…" : "Vérifier les mises à jour"}
      </button>
      {result && (
        <p
          className={`update-status ${result.state === "current" ? "current" : ""}`}
          role="status"
        >
          {result.state === "current" && <CheckCircle2 size={18} />}
          {result.message}
          {result.version && ` Version ${result.version}.`}
        </p>
      )}
      {result?.notes && (
        <details>
          <summary>Notes de cette version</summary>
          <pre>{result.notes}</pre>
        </details>
      )}
      {result?.state === "available" && (
        <div className="update-actions">
          {downloaded && (
            <p>
              La mise à jour est prête. L’application va redémarrer ; les
              modifications non enregistrées seront perdues.
            </p>
          )}
          <button
            type="button"
            className="primary"
            disabled={busy}
            onClick={() => void update()}
          >
            <Download size={15} />
            {downloaded
              ? "Installer et redémarrer"
              : "Télécharger la mise à jour"}
          </button>
          {downloaded && (
            <button
              type="button"
              className="outlined"
              disabled={busy || !canSave}
              onClick={() => void update(true)}
            >
              <Save size={15} /> Enregistrer le projet puis installer
            </button>
          )}
        </div>
      )}
      {error && (
        <p className="update-error" role="alert">
          {error}
        </p>
      )}
    </section>
  );
}
