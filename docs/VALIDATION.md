# Boxmaker 1.0 validation record

On 1 October 2026, the maintainer reported the following completed checks:

| Area | Reported result |
| --- | --- |
| Printed press-to-slide closure | Working on actual prints |
| Printed sacrificial seal | Working on actual prints |
| Loaded-box transport | Tested successfully |
| Windows x64 installation and updates | Working |
| macOS installation and app functions | Working; the supplied screenshot shows a VMware guest without graphics acceleration |
| Bambu Studio exports | Imported and validated; a non-Bambu 3MF warning is expected |

These are maintainer-reported validations, not independent laboratory measurements. No measured force, cycle count, filament batch, temperature, payload, or transport protocol was supplied. Calculated forces remain estimates. Results do not guarantee every printer, filament, box size, or shipping condition.

Windows ARM64 is built and checked in CI but has not been tested on a user device. The maintainer explicitly accepted this limitation for 1.0. Native Intel Mac and Apple Silicon testing were not separately identified; do not describe both as independently validated.

The 1.0 viewer adds an SVG-based interactive fallback when WebGL is unavailable or its context is lost. This addresses the VMware preview limitation without requiring GPU acceleration. The same geometry is used for the accelerated preview, fallback, and exports.

The macOS updater is new in 1.0. Existing 0.8.x Mac installations need one manual installation of the 1.0 DMG. Future updates are checked in the app and their cryptographic signatures are verified before installation. The first real Mac upgrade using this updater is a separate check from the older manual installation reported above.
