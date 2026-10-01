# Developing Boxmaker

For app use, start with the [README](../README.md) and [user guide](USER_GUIDE.md).

## Tools and commands

Windows requires Rust/MSVC, Visual Studio C++ tools including CMake, Bun, and WebView2. CI uses Rust 1.98.0 and the Bun version in `.bun-version`. Velopack is pinned in `.config/dotnet-tools.json`; restore it with .NET SDK 8 and `dotnet tool restore`.

macOS requires Xcode command-line tools, Rust, Bun, and CMake. CI builds an Intel/Apple Silicon universal app with ad hoc signing. The updater signing key is supplied only to trusted release-tag builds through GitHub Actions, separately from Apple's OS signing.

```powershell
bun install --frozen-lockfile
bun run dev
bun run desktop
bun run check
bun run release:check
bun run desktop:build
dotnet tool restore
bun run package:windows -- -SkipBuild
bun run package:msix -- -SkipBuild
```

Do not run `dev` and `desktop` simultaneously: both use port 1420. The Vite engine endpoint is a development adapter; the packaged app invokes Rust directly.

## Architecture

- `crates/boxmaker-core`: validation, project migration, parametric geometry, mesh generation, weight, postal rates, binary STL and standard 3MF.
- `crates/boxmaker-cli`: JSON adapter for the local development server.
- `src-tauri`: native app, file-save dialog, Windows Velopack updates and macOS Tauri updates.
- `src`: React interface, English/French translation, local draft recovery, and Three.js previews.
- `scripts`: repeatable validation and packaging commands.

Manifold's pinned C++ kernel is compiled through CMake. The initial rectilinear geometry is retained for legacy projects. The preview and exports use the same meshes. SVGRenderer supplies an interactive fallback when WebGL is unavailable; it simplifies lighting and omits shadows. Rendering is scheduled after view changes rather than continuously. Parameter changes are debounced, with one running calculation and only the most recent pending request.

## Projects and errors

Project format 5 remains current for 1.0. Rust accepts versions 1–5 and validates types and ranges before replacing the current project. Legacy geometry is preserved. Geometry migrations invalidate measured shipment weight. Future formats prompt users to update, and damaged files leave the current project intact. The last successfully calculated project is stored locally for recovery; invalid edits do not overwrite it.

## Updates

Microsoft Store installations use Store updates. Existing Windows Velopack installations use GitHub, preserving `win` and `win-arm64` package channels. macOS uses the Tauri updater and verifies signatures before installation. Stable is the default; opting into beta includes prereleases without allowing a downgrade. Startup checks are configurable. Installation and restart require user action, with optional saving to a project file.

No account, analytics, or cloud project storage is required. See [privacy](PRIVACY.md).

## Validation and limits

Rust tests cover postal thresholds, orientation, printer bounds, meshes, exports, project migration, invalid input and closure geometry. The translation check covers interface and engine messages. The browser smoke script checks core user flows. CI builds all advertised platforms; user-device coverage is recorded separately in [VALIDATION.md](VALIDATION.md).

The maintainer confirmed physical closure, seal and loaded transport checks. Calculated PLA force remains an estimate, not a measured force. Printer exclusions, infill, filament properties, payload protection and postage must still be checked for a user's particular print. The app is not certified packaging.

Boxmaker is licensed under MIT. Preserve third-party licenses and notices when redistributing dependencies. See [releasing](RELEASING.md).
