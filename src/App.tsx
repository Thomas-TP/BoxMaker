import {
  ArrowDownToLine,
  ArrowRight,
  Box,
  Check,
  ChevronDown,
  CircleHelp,
  Eye,
  FileDown,
  FolderOpen,
  Layers3,
  LoaderCircle,
  Maximize,
  Package,
  Printer,
  RefreshCw,
  RotateCcw,
  Save,
  Settings2,
  ShieldCheck,
  SlidersHorizontal,
  Truck,
  X,
} from "lucide-react";
import { useEffect, useRef, useState } from "react";
import { Modal } from "./Modal";
import type { Design, Params } from "./types";
import { defaults, download, engine } from "./types";
import { Updates } from "./Updates";
import { Viewer } from "./Viewer";

const money = (v: number) =>
  new Intl.NumberFormat("fr-CH", { style: "currency", currency: "CHF" }).format(
    v,
  );
const number = (v: number) =>
  new Intl.NumberFormat("fr-CH", { maximumFractionDigits: 1 }).format(v);
const dimensions = (v: number[]) => v.map(number).join(" × ");

function Field({
  label,
  value,
  onChange,
  unit = "mm",
  min = 0,
  max = 2500,
  step = 1,
  optional = false,
}: {
  label: string;
  value: number | null;
  onChange: (value: number | null) => void;
  unit?: string;
  min?: number;
  max?: number;
  step?: number;
  optional?: boolean;
}) {
  return (
    <label className="field">
      <span>{label}</span>
      <div>
        <input
          onFocus={(e) => e.currentTarget.select()}
          type="number"
          min={min}
          max={max}
          step={step}
          value={value ?? ""}
          placeholder={optional ? "Inconnu" : ""}
          onChange={(e) =>
            onChange(
              e.target.value === "" && optional ? null : Number(e.target.value),
            )
          }
        />
        <small>{unit}</small>
      </div>
    </label>
  );
}

export default function App() {
  const [params, setParams] = useState<Params>(defaults);
  const [design, setDesign] = useState<Design | null>(null);
  const [mode, setMode] = useState<"assembled" | "exploded" | "print" | "open">(
    "exploded",
  );
  const [opening, setOpening] = useState(0);
  const [showObject, setShowObject] = useState(true);
  const [reset, setReset] = useState(0);
  const [busy, setBusy] = useState(true);
  const [error, setError] = useState("");
  const [notice, setNotice] = useState("");
  const [exporting, setExporting] = useState(false);
  const [part, setPart] = useState("all");
  const [format, setFormat] = useState("3mf");
  const [help, setHelp] = useState(false);
  const [updatesOpen, setUpdatesOpen] = useState(false);
  const [advanced, setAdvanced] = useState(false);
  const sequence = useRef(0);
  const upload = useRef<HTMLInputElement>(null);
  const set = <K extends keyof Params>(key: K, value: Params[K]) =>
    setParams((p) => ({
      ...p,
      ...([
        "object",
        "objectWeight",
        "padding",
        "objectClearance",
        "paddingWeight",
        "wall",
        "floor",
        "clearance",
        "model",
        "printer",
        "plateMargin",
      ].includes(key)
        ? { measuredTotal: null }
        : {}),
      [key]: value,
    }));
  useEffect(() => {
    const id = ++sequence.current;
    setBusy(true);
    setError("");
    const timer = setTimeout(() => {
      engine<Design>("calculate", params)
        .then((result) => {
          if (id === sequence.current) {
            setDesign(result);
            setPart((current) =>
              current === "all" || result.parts.some((p) => p.id === current)
                ? current
                : "body",
            );
          }
        })
        .catch((e: unknown) => {
          if (id === sequence.current) setError(String(e));
        })
        .finally(() => {
          if (id === sequence.current) setBusy(false);
        });
    }, 180);
    return () => {
      clearTimeout(timer);
      sequence.current++;
    };
  }, [params]);
  useEffect(() => {
    if (!notice) return;
    const timer = setTimeout(() => setNotice(""), 5000);
    return () => clearTimeout(timer);
  }, [notice]);
  async function exportPart() {
    setExporting(true);
    try {
      const result = await engine<{ bytes: number[]; filename: string }>(
        "export",
        params,
        { part, format },
      );
      const saved = await download(
        new Uint8Array(result.bytes),
        result.filename,
      );
      setNotice(
        saved
          ? "Fichier exporté en millimètres. Ouvrez-le dans votre slicer."
          : "Export annulé.",
      );
    } catch (e) {
      setNotice(`Export impossible : ${String(e)}`);
    } finally {
      setExporting(false);
    }
  }
  async function saveProject() {
    try {
      const saved = await download(
        JSON.stringify({ version: 3, params }, null, 2),
        "ma-boite.boxmaker.json",
        "application/json",
      );
      setNotice(saved ? "Projet enregistré." : "Enregistrement annulé.");
      return saved;
    } catch (e) {
      setNotice(`Enregistrement impossible : ${String(e)}`);
      return false;
    }
  }
  async function load(file: File | undefined) {
    if (!file) return;
    try {
      if (file.size > 32768) throw new Error("Fichier trop volumineux");
      const project = JSON.parse(await file.text());
      if (![1, 2, 3].includes(project.version) || !project.params)
        throw new Error("Projet Boxmaker incompatible");
      const loaded = {
        ...project.params,
        objectClearance:
          project.params.objectClearance ?? defaults.objectClearance,
        model: project.params.model ?? "legacy",
      };
      const migrated = loaded.model === "press-slide" && project.version < 3;
      if (migrated) loaded.measuredTotal = null;
      await engine<Design>("calculate", loaded);
      setParams(loaded);
      setMode("exploded");
      setOpening(0);
      setNotice(
        migrated
          ? "Projet adapté à la nouvelle fermeture et orienté automatiquement. Repesez l’envoi."
          : "Projet chargé.",
      );
    } catch (e) {
      setNotice(`Ouverture impossible : ${String(e)}`);
    }
  }
  const stale = busy || !!error;
  const best = design?.quotes[0];
  const selectedPart = design?.parts.find((p) => p.id === part);
  const allFit = design?.parts.every((p) => p.fits);
  const tariffExpired = design
    ? new Date().toISOString().slice(0, 10) > design.tariffValidUntil
    : false;
  return (
    <div className="app-shell">
      <header className="topbar">
        <div className="brand">
          <div className="brand-mark">
            <Box size={25} strokeWidth={1.6} />
          </div>
          <span>
            boxmaker<span className="brand-by">Swiss3Design</span>
          </span>
        </div>
        <div className="top-center">
          <span className="status-dot" /> Atelier de création{" "}
          <span className="version">v{__APP_VERSION__}</span>
        </div>
        <div className="header-actions">
          <button
            type="button"
            className="primary compact-export"
            disabled={
              stale ||
              exporting ||
              !(part === "all" ? allFit : selectedPart?.fits)
            }
            onClick={() => void exportPart()}
          >
            <ArrowDownToLine size={15} /> Exporter
          </button>
          <button
            type="button"
            className="text-button updates-button"
            onClick={() => setUpdatesOpen(true)}
          >
            <RefreshCw size={15} /> <span>Mises à jour</span>
          </button>
          <span className="header-separator" />
          <button
            type="button"
            className="text-button"
            onClick={() => upload.current?.click()}
          >
            <FolderOpen size={16} /> Ouvrir
          </button>
          <button
            type="button"
            className="outlined"
            disabled={stale}
            onClick={() => void saveProject()}
          >
            <Save size={15} /> Enregistrer
          </button>
          <button
            type="button"
            className="icon-button"
            aria-label="Guide d’utilisation"
            onClick={() => setHelp(true)}
          >
            <CircleHelp size={20} />
          </button>
        </div>
        <input
          ref={upload}
          type="file"
          accept=".json"
          hidden
          onChange={(e) => {
            void load(e.target.files?.[0]);
            e.target.value = "";
          }}
        />
      </header>

      <div className="page-heading">
        <div>
          <div className="eyebrow">VOTRE ATELIER D’EMBALLAGE</div>
          <h1>Votre boîte, sur mesure.</h1>
          <p>Vos dimensions. Deux pièces. Prêt à imprimer.</p>
        </div>
        <div className="swiss-label">
          <span className="swiss-cross">✚</span> Expédition en Suisse
        </div>
      </div>

      <main className="workspace">
        <aside className="parameters panel">
          <div className="panel-heading">
            <SlidersHorizontal size={18} />
            <h2>Configuration</h2>
            <button
              type="button"
              className="icon-button"
              aria-label="Réinitialiser les paramètres"
              onClick={() => setParams(defaults)}
            >
              <RotateCcw size={15} />
            </button>
          </div>
          <section className="form-section">
            <div className="section-title">
              <span className="step">01</span>
              <h3>L’objet à protéger</h3>
            </div>
            <div className="field-grid three">
              {(params.model === "press-slide"
                ? ["Dimension A", "Dimension B", "Dimension C"]
                : ["Largeur", "Longueur", "Hauteur"]
              ).map((label, i) => (
                <Field
                  key={label}
                  label={label}
                  value={params.object[i]}
                  min={10}
                  onChange={(v) => {
                    const object = [...params.object] as Params["object"];
                    object[i] = v ?? 0;
                    set("object", object);
                  }}
                />
              ))}
            </div>
            <div className="field-grid two">
              <Field
                label="Poids de l’objet"
                unit="g"
                value={params.objectWeight}
                optional
                min={0.1}
                max={100000}
                onChange={(v) => set("objectWeight", v)}
              />
              <Field
                label="Calage par face"
                value={params.padding}
                max={100}
                onChange={(v) => set("padding", v ?? 0)}
              />
            </div>
            {params.model === "legacy" ? (
              <button
                type="button"
                className="inline-action"
                onClick={() =>
                  set("object", [
                    params.object[1],
                    params.object[2],
                    params.object[0],
                  ])
                }
              >
                <RotateCcw size={13} /> Changer l’orientation de l’objet
              </button>
            ) : (
              <p className="orientation-note">
                <strong>Orientation automatique</strong>
                <span>
                  {design
                    ? `${dimensions(design.orientedObject)} mm`
                    : "Calcul…"}{" "}
                  · largeur × longueur × hauteur
                </span>
                Les dimensions peuvent être saisies dans n’importe quel ordre.
              </p>
            )}
            {params.model === "press-slide" && (
              <div className="object-fit">
                <Field
                  label="Jeu autour de l’objet"
                  value={params.objectClearance}
                  min={0.1}
                  max={2}
                  step={0.1}
                  onChange={(v) => set("objectClearance", v ?? 0.3)}
                />
                <p className="construction-note">
                  Espace libre par face, en plus du calage.
                </p>
              </div>
            )}
          </section>
          <section className="form-section">
            <div className="section-title">
              <span className="step">02</span>
              <h3>Votre imprimante</h3>
            </div>
            <div className="printer-options">
              {(
                [
                  {
                    id: "p1s",
                    name: "Bambu Lab P1S",
                    size: "256 × 256 × 256 mm",
                  },
                  { id: "k2", name: "Creality K2", size: "260 × 260 × 260 mm" },
                ] as const
              ).map((p) => (
                <button
                  key={p.id}
                  type="button"
                  className={`printer-option ${params.printer === p.id ? "selected" : ""}`}
                  onClick={() => set("printer", p.id)}
                  aria-pressed={params.printer === p.id}
                >
                  <Printer size={24} strokeWidth={1.4} />
                  <span>
                    <strong>{p.name}</strong>
                    <small>{p.size}</small>
                  </span>
                  <span className="radio">
                    {params.printer === p.id && <span />}
                  </span>
                </button>
              ))}
            </div>
            <div className="material-tag">
              <span className="material-dot" />
              <strong>PLA</strong>
              <span>Matériau du projet</span>
              <Check size={14} />
            </div>
          </section>
          <section className="form-section">
            <div className="section-title">
              <span className="step">03</span>
              <h3>La boîte</h3>
              <span className="small-badge">
                {params.model === "press-slide" ? "2 pièces" : "3 pièces"}
              </span>
            </div>
            <div className="field-grid two">
              <Field
                label="Épaisseur parois"
                value={params.wall}
                min={1.2}
                max={5}
                step={0.2}
                onChange={(v) => set("wall", v ?? 0)}
              />
              <Field
                label="Épaisseur fond"
                value={params.floor}
                min={params.model === "press-slide" ? 0.8 : 1.2}
                max={6}
                step={0.2}
                onChange={(v) => set("floor", v ?? 0)}
              />
            </div>
            <div className="closure">
              <Layers3 size={20} />
              <span>
                <strong>Couvercle coulissant</strong>
                <small>
                  {params.model === "press-slide"
                    ? "Appuyer, puis tirer · verrou intégré"
                    : "Ancien modèle · clavette séparée"}
                </small>
              </span>
            </div>
            <button
              type="button"
              className="advanced-toggle"
              aria-expanded={advanced}
              onClick={() => setAdvanced(!advanced)}
            >
              <Settings2 size={14} /> Réglages avancés{" "}
              <ChevronDown size={14} className={advanced ? "rotated" : ""} />
            </button>
            {advanced && (
              <div className="advanced-fields">
                <label className="model-choice">
                  Modèle de fermeture
                  <select
                    value={params.model}
                    onChange={(e) => {
                      const model = e.target.value as Params["model"];
                      setParams((p) => ({
                        ...p,
                        model,
                        wall: model === "legacy" ? 1.6 : 1.2,
                        floor: model === "legacy" ? 2 : 0.8,
                        measuredTotal: null,
                      }));
                      setMode("exploded");
                      setOpening(0);
                    }}
                  >
                    <option value="press-slide">
                      Coulissant à pression · nervuré
                    </option>
                    <option value="legacy">Ancienne boîte · clavette</option>
                  </select>
                </label>

                <div className="field-grid two">
                  <Field
                    label="Jeu par côté"
                    value={params.clearance}
                    min={0.15}
                    max={0.6}
                    step={0.05}
                    onChange={(v) => set("clearance", v ?? 0)}
                  />
                  <Field
                    label="Marge plateau"
                    value={params.plateMargin}
                    max={20}
                    onChange={(v) => set("plateMargin", v ?? 0)}
                  />
                  <Field
                    label="Poids calage"
                    unit="g"
                    value={params.paddingWeight}
                    max={10000}
                    onChange={(v) => set("paddingWeight", v ?? 0)}
                  />
                  <Field
                    label="Filament"
                    unit="CHF/kg"
                    value={params.filamentPrice}
                    max={200}
                    onChange={(v) => set("filamentPrice", v ?? 0)}
                  />
                </div>
                <button
                  type="button"
                  className="inline-action"
                  onClick={() =>
                    setParams({
                      ...params,
                      object:
                        params.model === "press-slide"
                          ? [30, 40, 10]
                          : [20, 20, 10],
                      padding: 0,
                      objectWeight: 5,
                      measuredTotal: null,
                    })
                  }
                >
                  Charger une petite boîte d’essai <ArrowRight size={13} />
                </button>
              </div>
            )}
          </section>
          <div className="local-note">
            <ShieldCheck size={15} />
            <span>Vos fichiers restent sur votre ordinateur.</span>
          </div>
        </aside>

        <section className="preview-column">
          <div className="preview panel">
            <div className="preview-top">
              <span>
                <span className={`status-dot ${busy ? "pulsing" : ""}`} />
                {busy
                  ? "Calcul en cours…"
                  : error
                    ? "Paramètres à corriger"
                    : "Aperçu de votre boîte"}
              </span>
              <span className="prototype-tag">
                <Box size={12} /> APERÇU 3D
              </span>
            </div>
            <div className="view-modes">
              {(
                [
                  { id: "assembled", label: "Fermée" },
                  { id: "exploded", label: "Vue éclatée" },
                  { id: "print", label: "Pièces à plat" },
                ] as const
              ).map((v) => (
                <button
                  type="button"
                  key={v.id}
                  onClick={() => setMode(v.id)}
                  aria-pressed={mode === v.id}
                  className={mode === v.id ? "active" : ""}
                >
                  {v.label}
                </button>
              ))}
              {params.model === "press-slide" && (
                <button
                  type="button"
                  aria-pressed={mode === "open"}
                  className={mode === "open" ? "active" : ""}
                  onClick={() => {
                    setMode("open");
                    setOpening(0);
                  }}
                >
                  Ouverture
                </button>
              )}
            </div>
            <Viewer
              design={design}
              mode={mode}
              showObject={showObject}
              reset={reset}
              opening={opening}
            />
            {!design && !error && (
              <div className="loading-model">
                <LoaderCircle className="spin" /> Construction de votre boîte…
              </div>
            )}
            {error && (
              <div className="error-banner" role="alert">
                {error}
              </div>
            )}
            <div className="view-tools">
              <button
                type="button"
                className={showObject ? "active" : ""}
                aria-label="Afficher l’objet"
                aria-pressed={showObject}
                onClick={() => setShowObject(!showObject)}
              >
                <Eye size={17} />
              </button>
              <button
                type="button"
                aria-label="Recentrer la vue"
                onClick={() => setReset(reset + 1)}
              >
                <Maximize size={17} />
              </button>
            </div>
            <div className="preview-bottom">
              <span>Glisser pour tourner · Molette pour zoomer</span>
              <span>mm</span>
            </div>
          </div>
          {mode === "open" && params.model === "press-slide" && (
            <div className="opening-control panel">
              <label htmlFor="opening-progress">
                <strong>
                  {opening <= 15
                    ? "1 · Appuyer sur la languette"
                    : opening < 30
                      ? "2 · Tirer le couvercle"
                      : "3 · Relâcher et faire glisser"}
                </strong>
                <span>Démonstration du mouvement</span>
              </label>
              <input
                id="opening-progress"
                type="range"
                min="0"
                max="100"
                value={opening}
                onChange={(e) => setOpening(Number(e.target.value))}
              />
              <p>
                Les rails retiennent le couvercle ; la languette bloque
                uniquement le glissement. Mouvement illustratif, à confirmer par
                impression.
              </p>
            </div>
          )}
          <div className={`metrics panel ${stale ? "muted" : ""}`}>
            <div>
              <span>DIMENSIONS EXTÉRIEURES</span>
              <strong>
                {design ? dimensions(design.outer) : "—"}
                <small> mm</small>
              </strong>
            </div>
            <div>
              <span>PLA ESTIMÉ</span>
              <strong>
                {design ? number(design.plasticWeight) : "—"}
                <small> g</small>
              </strong>
            </div>
            <div>
              <span>COÛT MATIÈRE</span>
              <strong>{design ? money(design.materialCost) : "—"}</strong>
            </div>
          </div>
          {design?.mechanism && (
            <details className={`fit-summary panel ${stale ? "muted" : ""}`}>
              <summary className="fit-summary-heading">
                <strong>Espace autour de l’objet</strong>
                <span>
                  {dimensions(design.inner)} mm <ChevronDown size={15} />
                </span>
              </summary>
              <p>
                Jeu total : <strong>{dimensions(design.objectSpace)} mm</strong>{" "}
                (largeur × longueur × hauteur), dont {number(params.padding)} mm
                de calage et {number(params.objectClearance)} mm de jeu par
                face.
              </p>
              {design.mechanismExpansion.some((v) => v > 0.01) && (
                <p>
                  Minimum nécessaire au mécanisme :{" "}
                  <strong>+{dimensions(design.mechanismExpansion)} mm</strong>.
                  L’objet est centré dans cet espace.
                </p>
              )}
              <p className="construction-note">
                Hors cavité utile : zone du verrou de{" "}
                {number(design.mechanism.rearAllowance)} mm à l’arrière et{" "}
                {number(design.mechanism.headroom)} mm au-dessus pour le
                mouvement.
              </p>
              <details className="spring-details">
                <summary>
                  Pression calculée : ≈{" "}
                  {number(design.mechanism.estimatedForce)} N · languette
                  adaptée
                </summary>
                <p>
                  Plage indicative selon la rigidité du PLA :{" "}
                  {number(design.mechanism.forceRange[0])}–
                  {number(design.mechanism.forceRange[1])} N. À vérifier sur une
                  impression d’essai.
                </p>
                <p>
                  Longueur {number(design.mechanism.beamLength)} mm · largeur{" "}
                  {number(2 * design.mechanism.tongueHalfWidth)} mm · épaisseur{" "}
                  {number(design.mechanism.beamThickness)} →{" "}
                  {number(design.mechanism.tipThickness)} mm.
                </p>
                <p>
                  Déformation nominale calculée :{" "}
                  {design.mechanism.strainPercent.toFixed(2)} % ; sur butée :{" "}
                  {design.mechanism.stopStrainPercent.toFixed(2)} %. La durée de
                  vie en cycles n’est pas prédite.
                </p>
              </details>
            </details>
          )}
          <div className="parts-strip panel">
            {design?.parts.map((p, i) => (
              <div key={p.id}>
                <span className={`part-dot part-${i}`} />
                <strong>{p.name}</strong>
                <span>{dimensions(p.size)} mm</span>
                {p.fits ? (
                  <Check size={13} />
                ) : (
                  <X size={13} className="danger" />
                )}
              </div>
            ))}
          </div>
          {mode === "print" && (
            <p className="below-note">
              Pièces présentées côte à côte, comme dans l’export 3MF complet.
              Réorganisez-les dans le slicer ou répartissez-les sur plusieurs
              plateaux si nécessaire.
            </p>
          )}
        </section>

        <aside className="shipping-column">
          <section className="export panel">
            <div className="panel-heading">
              <FileDown size={18} />
              <h2>Exporter votre boîte</h2>
            </div>
            <div className={`fit-status ${allFit ? "" : "invalid"}`}>
              {allFit ? <Check size={15} /> : <X size={15} />}
              <span>
                {allFit
                  ? "Chaque pièce tient sur le plateau utile"
                  : "Vérifiez les dimensions des pièces"}
              </span>
            </div>
            <div className="export-selects">
              <label>
                Pièce
                <select
                  value={part}
                  onChange={(e) => {
                    setPart(e.target.value);
                    if (e.target.value === "all") setFormat("3mf");
                  }}
                >
                  <option value="all">
                    {params.model === "legacy"
                      ? "Toutes les pièces (3)"
                      : "Boîte + couvercle"}
                  </option>
                  <option value="body">Boîte</option>
                  <option value="lid">Couvercle</option>
                  {params.model === "legacy" && (
                    <option value="key">Clavette</option>
                  )}
                </select>
              </label>
              <label>
                Format
                <select
                  value={format}
                  disabled={part === "all"}
                  onChange={(e) => setFormat(e.target.value)}
                >
                  <option value="3mf">3MF</option>
                  <option value="stl">STL</option>
                </select>
              </label>
            </div>
            <button
              type="button"
              className="primary"
              disabled={
                stale ||
                exporting ||
                !(part === "all" ? allFit : selectedPart?.fits)
              }
              onClick={() => void exportPart()}
            >
              {exporting ? (
                <LoaderCircle className="spin" size={17} />
              ) : (
                <ArrowDownToLine size={17} />
              )}{" "}
              Exporter{" "}
              {part === "all"
                ? params.model === "legacy"
                  ? "les 3 pièces"
                  : "les 2 pièces"
                : part === "body"
                  ? "la boîte"
                  : part === "lid"
                    ? "le couvercle"
                    : "la clavette"}
              <ArrowRight size={16} />
            </button>
            <p>
              {part === "all" ? (
                <>
                  Un fichier 3MF · pièces séparées, posées à plat.
                  <br />
                  Réorganisez-les ou répartissez-les sur plusieurs plateaux dans
                  le slicer selon la place disponible.
                </>
              ) : (
                <>
                  Géométrie en mm · une pièce par fichier
                  <br />
                  Réglages d’impression à choisir dans le slicer.
                </>
              )}
            </p>
          </section>
          <section className={`shipping panel ${stale ? "muted" : ""}`}>
            <div className="panel-heading">
              <Truck size={19} />
              <h2>Votre expédition</h2>
              <span className="swiss-cross small">✚</span>
            </div>
            <div className="destination">
              <span>Destination</span>
              <strong>
                Suisse <Check size={13} />
              </strong>
            </div>
            <label className="switch-row">
              <span>
                Étiquette colis en ligne
                <small>Rabais de CHF 1.50 sur les colis admissibles</small>
              </span>
              <input
                type="checkbox"
                checked={params.online}
                onChange={(e) => set("online", e.target.checked)}
              />
              <span className="switch" />
            </label>
            <div className="weight-summary">
              <Package size={15} />
              <span>
                Poids total{" "}
                {params.measuredTotal === null ? "estimé" : "mesuré"}
              </span>
              <strong>
                {design?.totalWeight != null
                  ? `${number(design.totalWeight)} g`
                  : "À renseigner"}
              </strong>
            </div>
            {best && !tariffExpired ? (
              <div className="best-rate">
                <div className="rate-eyebrow">
                  <span>LE MOINS CHER</span>
                  <Check size={14} />
                </div>
                <h3>{best.service}</h3>
                <p>{best.category}</p>
                <div className="price">
                  <strong>{money(best.cents / 100)}</strong>
                  <span>/ envoi</span>
                </div>
                <div className="rate-foot">
                  <span>{best.delay}</span>
                  <span>{best.tracking ? "Avec suivi" : "Sans suivi"}</span>
                </div>
              </div>
            ) : (
              <div className="empty-rate">
                <Truck size={24} />
                <strong>
                  {tariffExpired
                    ? "Tarifs à actualiser"
                    : params.objectWeight === null &&
                        params.measuredTotal === null
                      ? "Ajoutez le poids"
                      : "Aucun tarif disponible"}
                </strong>
                <p>
                  {tariffExpired
                    ? "Cette grille est limitée à 2026. Vérifiez les prix sur poste.ch."
                    : "Le poids total et le format fermé déterminent les services admissibles."}
                </p>
              </div>
            )}
            {!tariffExpired && design && design.quotes.length > 1 && (
              <div className="other-rates">
                <h4>AUTRES POSSIBILITÉS</h4>
                {design.quotes.slice(1).map((q) => (
                  <div key={q.service}>
                    <span>
                      <strong>{q.service}</strong>
                      <small>
                        {q.category} · {q.tracking ? "suivi" : "sans suivi"}
                      </small>
                    </span>
                    <b>{money(q.cents / 100)}</b>
                  </div>
                ))}
              </div>
            )}
            <details className="weigh-details">
              <summary>
                J’ai pesé mon envoi fermé <ChevronDown size={13} />
              </summary>
              <Field
                label="Poids total réel, boîte et calage inclus"
                unit="g"
                optional
                value={params.measuredTotal}
                min={0.1}
                max={100000}
                onChange={(v) => set("measuredTotal", v)}
              />
            </details>
            <p className="tariff-note">
              Tarifs publics 2026 · vérifiés le 07.09.2026
              <br />
              Sous réserve du poids réel et d’un emballage conforme. Options et
              traitement manuel non inclus. A Plus et recommandé : prêts à
              l’envoi.
            </p>
            <a
              className="source-link"
              href="https://www.post.ch/fr/expedier-des-colis/colis-suisse"
              target="_blank"
              rel="noreferrer"
            >
              Consulter les tarifs de la Poste <ArrowRight size={12} />
            </a>
          </section>
        </aside>
      </main>
      <details className="warnings">
        <summary>
          <ShieldCheck size={16} /> Contrôles et conseils d’impression{" "}
          <span>{design?.warnings.length ?? 0}</span>
          <ChevronDown size={15} />
        </summary>
        <ul>
          {design?.warnings.map((w) => (
            <li key={w}>{w}</li>
          ))}
        </ul>
        <a
          href="https://www.post.ch/fr/expedier-des-colis/emballage-et-adressage/emballage-des-colis"
          target="_blank"
          rel="noreferrer"
        >
          Recommandations d’emballage de la Poste
        </a>
      </details>
      <footer>
        <span>
          SWISS3DESIGN <span className="footer-dot">/</span> BOXMAKER
        </span>
        <span>Conçu à la bonne taille.</span>
        <button type="button" onClick={() => setHelp(true)}>
          Guide & formats postaux <ArrowRight size={12} />
        </button>
      </footer>
      {notice && (
        <div className="toast" role="status">
          {notice}
          <button
            type="button"
            aria-label="Fermer la notification"
            onClick={() => setNotice("")}
          >
            <X size={15} />
          </button>
        </div>
      )}
      <Modal
        open={help}
        onClose={() => setHelp(false)}
        title="Guide d’utilisation"
      >
        <section className="help-modal">
          <div className="eyebrow">MODE D’EMPLOI</div>
          <h2 id="help-title">De votre objet à sa boîte.</h2>
          <ol>
            <li>
              Mesurez l’objet et renseignez son poids. Le calage s’ajoute sur
              les six faces, en plus du jeu d’insertion. Les trois dimensions
              peuvent être saisies dans n’importe quel ordre pour le nouveau
              modèle.
            </li>
            <li>
              Choisissez la P1S ou la K2 classique. La marge du plateau est
              réglable.
            </li>
            <li>
              Exportez toutes les pièces dans un seul 3MF, ou choisissez une
              pièce seule en 3MF ou STL. L’ancien modèle inclut aussi sa
              clavette dans l’export complet.
            </li>
            <li>
              Dans le slicer, vérifiez la géométrie, les zones exclues,
              l’adhérence et les petits surplombs des rails. Imprimez d’abord la
              petite boîte d’essai des réglages avancés.
            </li>
            <li>
              Testez le clic et le déverrouillage sans forcer, protégez l’objet,
              scellez le couvercle avec un adhésif et pesez l’envoi fermé.
            </li>
          </ol>
          <h3>Formats extérieurs · Suisse</h3>
          <table>
            <thead>
              <tr>
                <th>Service</th>
                <th>Limite</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>B5</td>
                <td>250 × 176 × 20 mm · 500 g</td>
              </tr>
              <tr>
                <td>B5 épaisse</td>
                <td>Jusqu’à 50 mm · + CHF 2 en A/B/A Plus</td>
              </tr>
              <tr>
                <td>B4</td>
                <td>353 × 250 × 20 mm · 1 kg</td>
              </tr>
              <tr>
                <td>Colis standard</td>
                <td>1000 × 600 × 600 mm · 30 kg</td>
              </tr>
              <tr>
                <td>Encombrant</td>
                <td>
                  Longueur ≤ 2000 mm / 30 kg ; ≤ 2500 mm / 10 kg. L + 2l + 2h ≤
                  4000 mm.
                </td>
              </tr>
            </tbody>
          </table>
          <p>
            La boîte est un prototype, pas un emballage homologué. Les
            dimensions intérieures annoncées correspondent à l’espace libre sous
            le couvercle, entre les renforts.
          </p>
          <p>
            <a
              href="https://www.post.ch/fr/expedier-des-lettres/lettres-suisse"
              target="_blank"
              rel="noreferrer"
            >
              Tarifs lettres
            </a>{" "}
            ·{" "}
            <a
              href="https://www.post.ch/fr/expedier-des-colis/encombrant"
              target="_blank"
              rel="noreferrer"
            >
              Limites encombrants
            </a>
          </p>

          <button
            type="button"
            className="primary"
            onClick={() => setHelp(false)}
          >
            Créer ma boîte <ArrowRight size={16} />
          </button>
        </section>
      </Modal>
      <Modal
        open={updatesOpen}
        onClose={() => setUpdatesOpen(false)}
        title="Mises à jour"
      >
        <Updates saveProject={saveProject} canSave={!stale} />
      </Modal>
    </div>
  );
}
