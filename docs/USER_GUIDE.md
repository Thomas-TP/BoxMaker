# Boxmaker user guide

Install Boxmaker with the Microsoft Store or macOS download button in the [README](../README.md).

1. Enter your object's three dimensions in **millimetres**, in any order. The sliding-box model chooses a printable, low-profile orientation automatically. Enter the object's weight in **grams**. Clearance defaults to 0.3 mm on each face; add any cushioning separately.
2. Select **Bambu Lab P1S (256³ mm)** or **Creality K2 (260³ mm)**.
3. Adjust wall thickness, base thickness, and clearance in the advanced settings. The material profile is PLA, and you can change the price per kilogram.
4. Inspect the closed, exploded, print-layout, and opening views. The opening slider illustrates pressing the rear latch and sliding the lid off.
5. Use the complete export to save **one 3MF with three independent objects: box, lid, and seal**. Rearrange the objects in your slicer or use multiple print plates if needed. Individual STL and 3MF exports remain available, including a replacement seal. The files contain geometry in millimetres, without printer settings or G-code. Print the lid with its smooth face down and ribs up; print the seal flat. The complete export for the legacy model includes its separate locking key instead of the seal.
6. In Bambu Studio or Creality Print, check orientation, overhangs, excluded areas, and calculated material use. Bambu Studio may warn that this 3MF was not created by Bambu Studio. This is expected: Boxmaker exports standard geometry, not a Bambu project or printer settings. Import the geometry and select your own printer and PLA settings. Print a small test box before committing to a full shipment.
7. Pack and close the box, then push the small printed lock vertically into the rear-right corner until it engages. The recipient lifts and breaks its visible head, then presses the latch and slides the lid. The remaining stem stays in the shell during opening and can be removed afterwards. Weigh the closed parcel and enter its actual weight in Boxmaker. Test the mechanism on a real print before shipping.

You can save projects as `.boxmaker.json` files and reopen them later. A sliding-box project from v2 keeps its padding but adopts the newer mechanism and must be weighed again. A legacy key-based project keeps its geometry. Projects created before v4 do not gain a seal until you enable it. A project with the older seal design moves to the compact lock, clears its saved measured weight, and must be weighed again. Geometry or content changes also clear the measured weight.

## Interface

The 3D preview sits in the middle of the workspace, with dimensions and settings on the left and export and shipping on the right. On wide windows, the side panels scroll independently. Cavity and spring details expand beneath the preview. The legacy model selector lives in the advanced settings. A quick export action appears in the toolbar on narrower windows.

If WebGL or graphics acceleration is unavailable, an interactive software preview appears automatically. Rotation, zoom, and the four views remain available; lighting and shadows are simplified.

## Recovery and updates

The last valid project is recovered automatically after closing or restarting Boxmaker. Invalid edits are not written over the last valid draft. Saving `.boxmaker.json` files remains the best way to keep several projects or a durable backup. If you reset or open another project, that becomes the current recovery draft after validation.

Open **Updates** to choose the stable or beta channel and enable or disable checks on startup. The stable channel is the default. Beta selection is available on macOS and Windows Velopack installations; Microsoft Store installations stay on the Store's stable channel. Leaving beta does not downgrade the app: wait for a newer stable release. Download an available update, then choose **Install and restart** or **Save project then install**.

On Mac, drag Boxmaker to Applications before running it. Older 0.8.x installations need one manual upgrade to the 1.0 DMG to gain in-app updates. Updates are cryptographically verified; the app itself remains without Apple Developer ID or notarization, so first launch may require **Privacy & Security → Open Anyway**.

## License

Boxmaker is free and open source under the [MIT license](../LICENSE). You may use generated model files, print them, modify them, and sell your prints; Boxmaker imposes no requirement to credit it on physical boxes. The software license does not grant rights to third-party trademarks or content you import or add yourself.
