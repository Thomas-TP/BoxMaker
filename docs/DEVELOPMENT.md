# Développement de Boxmaker

Les commandes ci-dessous se lancent depuis la racine du dépôt. Pour découvrir l’application, voir le [README](../README.md) et le [guide d’utilisation](UTILISATION.md).

## Installer les outils

Prérequis : Windows, Rust stable/MSVC et outils C++ de Visual Studio, Bun, WebView2. Outils testés : Rust 1.98.0, Bun 1.3.11, Velopack 1.2.0. Pour l’installateur : .NET SDK 8 ou compatible et `dotnet tool restore`, qui installe la version de Velopack verrouillée dans `.config/dotnet-tools.json`.

```powershell
bun install --frozen-lockfile
bun run dev             # aperçu navigateur sur http://127.0.0.1:1420
bun run desktop         # application native en développement
bun run check           # Biome, TypeScript, tests Rust et Clippy
bun run release:check   # cohérence des versions, changelog et notes
bun run desktop:build   # ressources web embarquées, binaire Windows
dotnet tool restore     # CLI Velopack verrouillée
bun run package:windows -- -SkipBuild  # installateur et portable Velopack
bun run package:msix -- -SkipBuild     # MSIX non signé pour Partner Center
```

Ne pas lancer `dev` et `desktop` simultanément : le port 1420 est partagé.

## Architecture

- `crates/boxmaker-core` : validation, géométrie paramétrique, maillages, masse, tarifs, STL binaire et 3MF.
- `crates/boxmaker-cli` : adaptateur JSON utilisé uniquement par le serveur Vite local. Aucun calcul métier dupliqué dans l’interface.
- `src-tauri` : application native, commandes Rust, dialogue « Enregistrer sous » et initialisation Velopack avant Tauri.
- `src` : React/TypeScript strict et Three.js. Le rendu utilise les mêmes triangles que les exports.
- `scripts/package-windows.ps1` : création du package Velopack depuis un répertoire de staging dédié.

La nouvelle géométrie utilise Manifold via les bindings Rust `manifold-csg` pour les opérations booléennes, coins arrondis, nervures et languette. Son noyau C++ est compilé par CMake ; les scripts cherchent aussi CMake dans Visual Studio. Installer les outils CMake C++ si nécessaire. Le moteur rectiligne initial reste utilisé pour les anciens projets et la comparaison de matière. Aucun import de STL arbitraire n’est proposé.

## Validation et limites

- Les tests Rust couvrent les changements de tranche de poids, dimensions et rotations postales, encombrants, entrées invalides, export hors plateau, fermeture/orientation des maillages, volume et structure des exports.
- `scripts/browser-smoke.js` est un parcours Playwright CLI pour vérifier les limites P1S/K2, les tarifs lettre, le poids absent, les exports individuels et complets, et la sauvegarde/réouverture d’un projet.
- Le contrôle de plateau considère les pièces à plat avec rotation XY à 90° et une marge par bord. Les zones exclues spécifiques du slicer P1S ne sont pas modélisées. La vue « pièces à plat » n’est pas un placement automatique sur un plateau commun.
- La masse est estimée avec une densité de 1,24 g/cm³ et le volume solide. Le slicer, la densité réelle du filament, l’infill et la pesée finale peuvent différer. Le coût matière exclut temps, énergie, calage et affranchissement.
- **Prototype mécanique non homologué.** Cette révision n’a pas encore fait l’objet d’un essai physique. Le jeu, la fatigue de la languette PLA, les rails, la rupture du scellé et la protection du contenu doivent être testés physiquement. Le petit pont arrière, les lèvres des rails et les deux attaches du scellé sont à examiner dans le slicer. Voir [le choix mécanique](MECHANISM.md) et [le protocole du scellé affleurant](SEALING-v0.7.md).
- Tarifs pour les envois intérieurs en Suisse seulement, datés de 2026 ; détails dans [docs/POSTAL.md](POSTAL.md).

## Velopack

L’application initialise `VelopackApp` avant toute initialisation de l’interface. L’installation Windows et les fichiers de release sont générés par Velopack ; le packaging NSIS de Tauri est désactivé. WebView2 est déclaré comme prérequis de l’installateur.

Les mises à jour utilisent [les releases du dépôt GitHub](https://github.com/Thomas-TP/BoxMaker/releases), sans identifiants embarqués. Le bouton « Mises à jour » dans la barre d’outils permet de vérifier, lire les notes puis télécharger une version. « Installer et redémarrer » fonctionne sans enregistrer le projet ; une seconde action permet d’enregistrer puis d’installer. Les modifications non enregistrées sont perdues au redémarrage. Aucune vérification réseau, aucun téléchargement et aucun redémarrage ne sont déclenchés automatiquement au lancement. Les préversions sont incluses pendant la phase `0.x`.

Le workflow GitHub vérifie et construit chaque push/PR. Un tag de version valide déclenche ensuite la publication de la release avec installateur, portable, fichiers Velopack et SHA-256. Le MSIX non signé sert à la soumission Store locale et n’est pas envoyé à GitHub. Voir [le guide de release](RELEASING.md). Aucun compte externe n’est nécessaire pour concevoir une boîte localement.

Thomas Prud'homme figure dans les métadonnées de l’EXE Windows, sans que cela constitue une signature. L’EXE publié sur GitHub reste non signé. Le compte Microsoft Store affiche « ThomasTP » ; le MSIX de soumission porte exactement l’identité attribuée par Partner Center et n’est pas destiné à une installation directe. Le cycle réel installation → mise à jour depuis le Store reste à valider sur Windows. Voir [la procédure de publication](RELEASING.md#éditeur-windows-et-signature) et la [confidentialité](PRIVACY.md).

Documentation : [intégration Rust Velopack](https://docs.velopack.io/getting-started/rust), [packaging](https://docs.velopack.io/packaging/overview).
