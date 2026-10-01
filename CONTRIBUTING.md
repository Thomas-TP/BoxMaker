# Contributing to Boxmaker

Boxmaker is MIT-licensed. Keep Rust as the single source for geometry, validation, projects, exports, and shipping estimates. The interface presents engine results rather than duplicating calculations.

## Checks

```powershell
bun install --frozen-lockfile
bun run format
bun run check
bun run release:check
bun run desktop:build
```

For geometry changes, add a useful boundary or regression case and inspect the exports in a slicer. For postal rates, cite the official source, effective date, and expiry, and test values immediately below, at, and above thresholds.

Keep English and French interface messages in sync; `bun run check:translations` checks covered strings. Update Rust and JavaScript Tauri packages together: their major and minor versions must match. The release check enforces this.

Use descriptive commits (`feat:`, `fix:`, `docs:`, `test:`, `chore:`). Keep unrelated changes separate. Do not commit build outputs, local slicer profiles, browser profiles, screenshots of tests, signing keys, or credentials. Installers belong in GitHub Releases.

See [development](docs/DEVELOPMENT.md), [validation](docs/VALIDATION.md), and [releasing](docs/RELEASING.md).
