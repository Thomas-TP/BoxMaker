import { copyFileSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const { version } = JSON.parse(readFileSync("package.json", "utf8"));
const source = "target/universal-apple-darwin/release/bundle/macos";
const archives = readdirSync(source).filter((name) => name.endsWith(".app.tar.gz"));
if (archives.length !== 1) throw new Error("Expected one universal macOS updater archive.");
const signature = readFileSync(join(source, `${archives[0]}.sig`), "utf8").trim();
if (!signature || !/^[A-Za-z0-9+/=]+$/.test(signature)) throw new Error("Missing updater signature.");
const filename = `Boxmaker-${version}-macOS-update.tar.gz`;
copyFileSync(join(source, archives[0]), join("artifacts/mac-release", filename));
const platform = { url: `https://github.com/Thomas-TP/BoxMaker/releases/download/v${version}/${filename}`, signature };
writeFileSync("artifacts/mac-release/mac-updater.json", `${JSON.stringify({
  version,
  notes: readFileSync(`docs/releases/v${version}.md`, "utf8"),
  pub_date: new Date().toISOString(),
  platforms: { "darwin-x86_64": platform, "darwin-aarch64": platform },
}, null, 2)}\n`);
