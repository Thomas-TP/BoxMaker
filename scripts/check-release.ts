import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const root = resolve(import.meta.dir, "..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const pkg = JSON.parse(read("package.json"));
const version: string = pkg.version;
if (!/^\d+\.\d+\.\d+(?:-[a-z0-9.-]+)?$/.test(version)) throw new Error("Version SemVer invalide.");
const cargo = read("Cargo.toml").match(/\[workspace\.package\][\s\S]*?version\s*=\s*"([^"]+)"/)?.[1];
const tauri = JSON.parse(read("src-tauri/tauri.conf.json")).version;
if (cargo !== version || tauri !== version) throw new Error("Les versions package.json, Cargo.toml et tauri.conf.json doivent correspondre.");
for (const name of ["boxmaker", "boxmaker-core", "boxmaker-cli"]) {
  const lockVersion = read("Cargo.lock").match(new RegExp(`name = "${name}"\\r?\\nversion = "([^"]+)"`))?.[1];
  if (lockVersion !== version) throw new Error(`Cargo.lock est obsolète pour ${name}. Lancez cargo check.`);
}
const tag = `v${version}`;
if (process.env.GITHUB_REF_TYPE === "tag" && process.env.GITHUB_REF_NAME !== tag) throw new Error(`Le tag doit être ${tag}.`);
const notes = read(`docs/releases/${tag}.md`);
if (notes.trim().length < 150 || /\bTODO\b|À compléter/.test(notes)) throw new Error("Les notes de version doivent être complètes.");
if (!read("CHANGELOG.md").includes(`## [${version}]`)) throw new Error("Version absente du CHANGELOG.");
console.log(`Version ${tag} cohérente ; changelog et notes de release présents.`);
