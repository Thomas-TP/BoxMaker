import { Download, RefreshCw } from "lucide-react";
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
  async function update() {
    setBusy(true);
    setError("");
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      if (!downloaded) {
        await invoke("download_update");
        setDownloaded(true);
      } else if (await saveProject()) {
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
      <h3>Versions & mises à jour</h3>
      <p>
        Boxmaker {__APP_VERSION__} · Canal de préversion
        <br />
        Les versions proviennent du dépôt{" "}
        <a
          href="https://github.com/Thomas-TP/BoxMaker/releases"
          target="_blank"
          rel="noreferrer"
        >
          Thomas-TP/BoxMaker
        </a>
        . La vérification et l’installation se font à votre demande.
      </p>
      <button
        type="button"
        className="outlined"
        disabled={busy}
        onClick={() => void check()}
      >
        <RefreshCw size={14} className={busy ? "spin" : ""} />
        {busy ? "Opération en cours…" : "Vérifier les mises à jour"}
      </button>
      {result && (
        <p role="status">
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
        <button
          type="button"
          className="primary"
          disabled={busy || (downloaded && !canSave)}
          onClick={() => void update()}
        >
          <Download size={15} />
          {downloaded
            ? "Enregistrer le projet et redémarrer"
            : "Télécharger la mise à jour"}
        </button>
      )}
      {error && (
        <p className="update-error" role="alert">
          {error}
        </p>
      )}
    </section>
  );
}
