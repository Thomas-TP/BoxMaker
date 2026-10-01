# Release history

Each published version has [detailed release notes](docs/releases). The latest installers and checksums are on [GitHub Releases](https://github.com/Thomas-TP/BoxMaker/releases).

## [Unreleased]

No unreleased changes.

## [1.0.0] - 2026-10-01

- First stable release, with maintainer-reported print, transport, x64, Mac, and Bambu Studio validation.
- Interactive SVG preview fallback for machines without WebGL, and rendering only when needed.
- Automatic recovery of the last valid project; Rust validation and migration for project formats 1–5.
- Stable updates by default, optional beta channel, and configurable checks on startup.
- Signed macOS updater packages with in-app download, installation, and restart.
- Clearer bilingual errors and a Bambu Studio 3MF import note.
- Coalesced calculations during rapid input changes.
- MIT open-source license and updated English user/developer documentation.

## [0.8.3] - 2026-09-28

- English and French app interface and Microsoft Store listings, with language selection and automatic language detection.
- English README, user guide, release notes, screenshot filenames, and release asset labels.
- English default filenames for saved projects and complete 3MF exports.
- Updated macOS download badge using the Boxmaker logo.

## [0.8.2] - 2026-09-28

- Native Windows ARM64 build and separate update channel.
- Windows x64, Windows ARM64, and universal macOS downloads in GitHub Releases.

## [0.8.1] - 2026-09-27

- Universal macOS DMG preview for Intel and Apple Silicon Macs.
- Platform-specific update options. The Mac build still requires testing on a physical Mac.

## [0.8.0] - 2026-09-27

- Microsoft Store packaging and direct-link Store listing.
- Updates follow the installation source: Microsoft Store or GitHub/Velopack.

## [0.7.0] - 2026-09-27

- Compact, vertically inserted printable seal without the large side tab.
- Updated seal geometry, 3D preview, and project format v5.

## [0.6.0] - 2026-09-26

- Added a printable, breakable seal as a third part, with individual and combined exports.
- Project format v4; the mechanism still needs physical validation.

## [0.5.0] - 2026-09-11

- Refreshed interface and export layout, improved update controls, and removed the unnecessary PLA comparison.
- Installing an update no longer requires saving the current project first.

## [0.4.0] - 2026-09-11

- Combined 3MF export for the box and lid, with separate printable objects.

## [0.3.0] - 2026-09-09

- Automatic orientation across all six dimension permutations and refined latch geometry.
- Project format v3 and expanded geometry and export coverage.

## [0.2.1] - 2026-09-08

- Fixed tiny degenerate edges during STL and 3MF serialization.
- First published version of the new two-part mechanism.

## [0.2.0] - 2026-09-08

- Designed a two-part sliding latch and lighter box structure. This version was not published.

## [0.1.1] - 2026-09-08

- Fixed CSS line endings and release checksum coverage; first distributed preview.

## [0.1.0] - 2026-09-08

- Initial Rust box generator, 3D preview, STL/3MF exports, printer profiles, and Swiss Post estimates. This version was not published.
