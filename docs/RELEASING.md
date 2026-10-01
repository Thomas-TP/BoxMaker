# Releasing Boxmaker

## Prepare a version

Keep `package.json`, workspace `Cargo.toml`, `Cargo.lock`, and `src-tauri/tauri.conf.json` in sync. Add an English `docs/releases/vX.Y.Z.md` and a changelog entry. Update README download links. Run `bun run check`, `bun run release:check`, and the relevant user-flow checks before tagging.

Tauri Rust and JavaScript API major/minor versions must match. Keep dependency updates paired, including Dependabot proposals. Do not merge an isolated Rust Tauri upgrade until its frontend counterpart and platform builds pass.

## Builds and publication

The desktop workflow checks source, translations, Rust tests, Clippy, versions and notes; builds Windows x64, Windows ARM64, and a universal Mac app; and verifies executable architecture and the Mac bundle. A `vX.Y.Z` tag publishes a GitHub release only after all builds succeed. Versions starting with `0.` or carrying a prerelease suffix are marked prerelease; stable tags are explicitly published as stable/latest.

Windows public installation uses `Boxmaker-Windows.exe`, downloaded from the official Microsoft Store Web Installer endpoint and checked for a valid Microsoft signature. Preserve Velopack feeds and package names for installed users. Unsigned standalone setup EXEs and unsigned Store MSIX files are excluded from GitHub Releases.

The Mac manual installer is `Boxmaker-X.Y.Z-macOS.dmg`. Tauri also creates an updater archive; `scripts/prepare-macos-updater.ts` copies it and builds `mac-updater.json` using its signature and universal platform entries. Only updater-enabled tag builds require `TAURI_SIGNING_PRIVATE_KEY`. Non-tag and PR builds disable updater artifacts and never receive that secret.

Mac update signatures use a private key retained locally and stored as a GitHub Actions secret. Its public key is embedded in `src-tauri/tauri.macos.conf.json`. Never commit the private key or expose it in logs. Losing it prevents signing updates accepted by existing installations. OS signing remains ad hoc, without Developer ID/notarization. The update signature does not remove Gatekeeper's first-launch checks.

Before publication, the workflow checks required assets, restores both Windows feeds after Velopack uploads, adds clear labels, and regenerates SHA-256 checksums. Use the two prominent installer links in release notes; mark other assets as update internals.

## Microsoft Store

Store ID: `9MX7QLK05FJP`. Public publisher: `ThomasTP`. Keep the existing identity from `store/identity.json`: `ParkourPixels.Boxmaker` and its assigned publisher CN do not change when the public publisher name changes. SemVer `X.Y.Z` becomes MSIX `X.Y.Z.0`.

Download both CI Store-submission artifacts and upload their unsigned MSIX files to the existing Partner Center product. Keep English and French listings and the current unlisted availability. Microsoft signs packages after certification. Do not publish a GitHub MSIX unless a signed, verifiable package has actually been recovered. The GitHub Store web installer always installs the version currently live in the Store, so disclose any certification delay.

## Release verification

Check published asset names and hashes, the official Windows installer signature, update manifests, and stable/prerelease status. Verify clean install, reopening existing project files, saving/exporting, and upgrade from a previous installation. A compiled build or a passed CI job is not proof of a successful user-device update. Record remaining platform limitations in the notes and [validation record](VALIDATION.md).

## Postal maintenance

Keep official sources, verification/effective dates and expiry in [POSTAL.md](POSTAL.md). Refresh `postal.rs` and the dates in the engine with boundary tests before extending validity. Expired prices are hidden by the interface; do not apply a future rate table before its effective date.
