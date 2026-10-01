import { readFileSync, readdirSync } from "node:fs";
import { english, setLanguage, tr } from "../src/i18n";

const normalize = (text: string) => text.replace(/\s+/g, " ").trim();
const french = /[àâçéèêëîïôùûüœ]|\b(?:Votre|Vos|Projet|Fichier|Calage|Poids|Paroi|Fond|Jeu|Marge|Imprimante|Scellé|Courrier|Enregistrer|Ouvrir|Fermer|Vérifier|Réinitialiser|Rejoindre|Paramètres|Exporter|Aperçu|chargé|inconnue|manquants)\b/;
const missing = new Set<string>();
for (const file of readdirSync("src").filter((name) => /\.(?:tsx|ts)$/.test(name) && name !== "i18n.ts")) {
  const source = readFileSync(`src/${file}`, "utf8");
  for (const match of source.matchAll(/"([^"\n]+)"|>([^<>{}]+)</g)) {
    const text = normalize(match[1] ?? match[2]);
    if (match[2] && /\b(?:throw|await|const|return)\b/.test(text)) continue;
    if (french.test(text) && !english[text] && !/^[A-Z_]+$/.test(text)) missing.add(text);
  }
}
for (const file of ["crates/boxmaker-core/src/lib.rs", "crates/boxmaker-core/src/project.rs", "src-tauri/src/main.rs", "src-tauri/src/updates.rs", "src-tauri/src/updates_macos.rs"]) {
  const text = readFileSync(file, "utf8").split("#[cfg(test)]")[0];
  for (const match of text.matchAll(/"([^"\n]+)"/g)) {
    const message = normalize(match[1]);
    if (french.test(message) && !english[message] && !message.includes("{label}") && !message.includes("github.com") && !message.includes("Impossible de démarrer")) missing.add(message);
  }
}
if (missing.size) throw new Error(`Missing English translations:\n${[...missing].sort().join("\n")}`);
setLanguage("en");
for (const [source, target] of Object.entries(english)) {
  if (!target.trim() || tr(source) !== target) throw new Error(`Invalid translation: ${source}`);
}
if (tr("Paroi (mm) doit être entre 1.2 et 5.") !== "Wall thickness (mm) must be between 1.2 and 5.") throw new Error("Dynamic validation translation failed.");
setLanguage("fr");
if (tr("Projet chargé.") !== "Projet chargé.") throw new Error("French translation failed.");
console.log(`PASS: ${Object.keys(english).length} translations, visible strings and Rust messages, dynamic validation, English/French switching.`);
