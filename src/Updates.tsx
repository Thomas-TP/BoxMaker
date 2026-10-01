import {
  ArrowUpRight,
  CheckCircle2,
  Download,
  RefreshCw,
  Save,
} from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { localizeTree } from "./i18n";
import { projectFileError, readSetting, writeSetting } from "./storage";

interface UpdateView {
  state: "unavailable" | "available" | "current" | "empty" | "store";
  message: string;
  version: string | null;
  notes: string | null;
}

export function Updates({
  saveProject,
  canSave,
  onAvailable,
}: {
  saveProject: () => Promise<boolean>;
  canSave: boolean;
  onAvailable: () => void;
}) {
  const [result, setResult] = useState<UpdateView | null>(null);
  const [busy, setBusy] = useState(false);
  const [downloaded, setDownloaded] = useState(false);
  const [error, setError] = useState("");
  const [platform, setPlatform] = useState("Ordinateur");
  const [source, setSource] = useState("loading");
  const [beta, setBeta] = useState(
    () => readSetting("boxmaker-update-channel") === "beta",
  );
  const [autoCheck, setAutoCheck] = useState(
    () => readSetting("boxmaker-auto-update-check") !== "false",
  );
  const availableCallback = useRef(onAvailable);
  availableCallback.current = onAvailable;
  useEffect(() => {
    if (!("__TAURI_INTERNALS__" in window)) {
      setSource("unavailable");
      return;
    }
    void import("@tauri-apps/api/core")
      .then(({ invoke }) =>
        Promise.all([
          invoke<string>("app_platform"),
          invoke<string>("update_source"),
        ]),
      )
      .then(([platform, source]) => {
        setPlatform(platform);
        setSource(source);
      })
      .catch(() => {
        setSource("unavailable");
      });
  }, []);
  const check = useCallback(
    async (automatic = false) => {
      setBusy(true);
      setError("");
      try {
        if (!("__TAURI_INTERNALS__" in window)) {
          setResult({
            state: "unavailable",
            message:
              "Les mises à jour intégrées sont disponibles dans l’application installée.",
            version: null,
            notes: null,
          });
          return;
        }
        const { invoke } = await import("@tauri-apps/api/core");
        const view = await invoke<UpdateView>("check_update", { beta });
        setResult(view);
        if (automatic && view.state === "available")
          availableCallback.current();
        setDownloaded(false);
      } catch (e) {
        setError(projectFileError(e));
      } finally {
        setBusy(false);
      }
    },
    [beta],
  );
  useEffect(() => {
    if (autoCheck && source === "github") void check(true);
  }, [autoCheck, source, check]);
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
      setError(projectFileError(e));
    } finally {
      setBusy(false);
    }
  }
  return localizeTree(
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
          <span>
            {__APP_VERSION__.startsWith("0.") || __APP_VERSION__.includes("-")
              ? "Version installée · préversion"
              : "Version installée · stable"}
          </span>
        </div>
        <span className="version">{platform}</span>
      </div>
      <p className="update-source">
        <span>
          {source === "store"
            ? "Mises à jour via Microsoft Store"
            : "Versions publiées par Thomas Prud'homme"}
        </span>
        <a
          href="https://github.com/Thomas-TP/BoxMaker/releases"
          target="_blank"
          rel="noreferrer"
        >
          Notes de version <ArrowUpRight size={13} />
        </a>
      </p>
      <div className="update-preferences">
        <label className="update-option">
          <input
            type="checkbox"
            checked={beta && source !== "store"}
            disabled={busy || source !== "github"}
            onChange={(event) => {
              const value = event.target.checked;
              writeSetting(
                "boxmaker-update-channel",
                value ? "beta" : "stable",
              );
              setBeta(value);
              setResult(null);
              setDownloaded(false);
              setError("");
            }}
          />
          <span>
            <strong>Rejoindre la bêta</strong>
            <small>
              Recevez les nouveautés avant leur sortie stable. Elles peuvent
              contenir des problèmes.
            </small>
          </span>
        </label>
        {source === "store" ? (
          <p>
            Cette installation Microsoft Store reste sur le canal stable. Le
            choix bêta est disponible sur macOS et dans les installations
            Windows Velopack.
          </p>
        ) : (
          <>
            <p>
              {beta
                ? "Canal bêta : versions stables et préversions."
                : "Canal stable : uniquement les versions stables."}
            </p>
            <p>
              Quitter la bêta ne réinstalle pas une ancienne version. Vous
              recevrez la prochaine version stable plus récente.
            </p>
            <label className="update-option">
              <input
                type="checkbox"
                checked={autoCheck}
                disabled={busy || source !== "github"}
                onChange={(event) => {
                  writeSetting(
                    "boxmaker-auto-update-check",
                    String(event.target.checked),
                  );
                  setAutoCheck(event.target.checked);
                }}
              />
              <span>
                <strong>Vérifier au lancement</strong>
                <small>
                  Contacte GitHub pour rechercher une mise à jour.
                  L’installation et le redémarrage restent votre choix.
                </small>
              </span>
            </label>
          </>
        )}
      </div>
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
      {source === "store" && (
        <a
          className="outlined update-check"
          href="ms-windows-store://downloadsandupdates"
        >
          Ouvrir la bibliothèque Microsoft Store <ArrowUpRight size={14} />
        </a>
      )}
      {result?.state === "available" && (
        <div className="update-actions">
          {downloaded && (
            <p>
              La mise à jour est prête. L’application va redémarrer ; le dernier
              brouillon valide sera récupéré automatiquement.
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
    </section>,
  );
}
