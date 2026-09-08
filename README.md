# Boxmaker · Swiss3Design

Application Windows locale pour concevoir une boîte d’expédition imprimée en PLA, visualiser ses trois pièces et comparer les tarifs de la Poste suisse.

[Télécharger la préversion Windows](https://github.com/Thomas-TP/BoxMaker/releases) · [Notes de version](CHANGELOG.md) · [Compilation et releases](https://github.com/Thomas-TP/BoxMaker/actions)

## Utilisation

L’installateur Velopack se trouve dans `artifacts/releases/Swiss3Design.Boxmaker-win-Setup.exe`. Un ZIP portable est également produit. Le binaire autonome est `target/release/boxmaker.exe` et nécessite le runtime Microsoft WebView2.

1. Saisir les dimensions en **mm**, le poids de l’objet en **g**, et le calage par face.
2. Choisir **Bambu Lab P1S (256³ mm)** ou **Creality K2 classique (260³ mm)**.
3. Ajuster parois, fond et jeu dans les réglages avancés. Le profil est PLA ; le prix au kilo est modifiable.
4. Inspecter les vues fermée, éclatée et pièces à plat.
5. Exporter **chaque pièce séparément** en STL ou 3MF : boîte, couvercle, clavette. Le 3MF contient une géométrie en millimètres, sans profil machine ou G-code. La clavette se pose sur sa tête pour l’impression.
6. Dans Bambu Studio ou Creality Print, vérifier l’orientation, les surplombs, les zones exclues et le poids calculé. Imprimer une petite boîte d’essai avant un emballage complet.
7. Emballer, sécuriser la clavette avec un adhésif et peser l’envoi fermé. Saisir ce poids réel dans « J’ai pesé mon envoi fermé ».

Les projets peuvent être enregistrés en `.boxmaker.json` puis rouverts. Modifier la géométrie ou le contenu efface le poids mesuré pour éviter un tarif calculé sur une ancienne pesée.

## Développement

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
```

Ne pas lancer `dev` et `desktop` simultanément : le port 1420 est partagé.

## Architecture

- `crates/boxmaker-core` : validation, géométrie paramétrique, maillages, masse, tarifs, STL binaire et 3MF.
- `crates/boxmaker-cli` : adaptateur JSON utilisé uniquement par le serveur Vite local. Aucun calcul métier dupliqué dans l’interface.
- `src-tauri` : application native, commandes Rust, dialogue « Enregistrer sous » et initialisation Velopack avant Tauri.
- `src` : React/TypeScript strict et Three.js. Le rendu utilise les mêmes triangles que les exports.
- `scripts/package-windows.ps1` : création du package Velopack depuis un répertoire de staging dédié.

Le moteur Rust réalise des unions/soustractions exactes de volumes rectilignes via une grille construite sur les frontières des primitives. Il ne s’agit pas d’une approximation par voxels de taille fixe. Les faces internes sont supprimées et les sommets partagés. Ce moteur est volontairement limité à cette famille de boîtes ; il n’importe pas de modèles STL arbitraires et ne remplace pas un noyau CAO généraliste. Manifold n’est pas nécessaire à cette première géométrie.

## Validation et limites

- Les tests Rust couvrent les changements de tranche de poids, dimensions et rotations postales, encombrants, entrées invalides, export hors plateau, fermeture/orientation des maillages, volume et structure des exports.
- `scripts/browser-smoke.js` est un parcours Playwright CLI pour vérifier les limites P1S/K2, les tarifs lettre, le poids absent, les téléchargements et la sauvegarde/réouverture d’un projet.
- Le contrôle de plateau considère les pièces à plat avec rotation XY à 90° et une marge par bord. Les zones exclues spécifiques du slicer P1S ne sont pas modélisées. La vue « pièces à plat » n’est pas un placement automatique sur un plateau commun.
- La masse est estimée avec une densité de 1,24 g/cm³ et le volume solide. Le slicer, la densité réelle du filament, l’infill et la pesée finale peuvent différer. Le coût matière exclut temps, énergie, calage et affranchissement.
- **Prototype mécanique non homologué et non imprimé lors du développement.** Le jeu, la résistance du PLA, les rails, la clavette et la protection du contenu doivent être testés physiquement. Les arêtes de cette première version sont rectilignes. Les petits surplombs des rails sont à examiner dans le slicer.
- Tarifs pour les envois intérieurs en Suisse seulement, datés de 2026 ; détails dans [docs/POSTAL.md](docs/POSTAL.md).

## Velopack

L’application initialise `VelopackApp` avant toute initialisation de l’interface. L’installation Windows et les fichiers de release sont générés par Velopack ; le packaging NSIS de Tauri est désactivé. WebView2 est déclaré comme prérequis de l’installateur.

Les mises à jour utilisent [les releases du dépôt GitHub](https://github.com/Thomas-TP/BoxMaker/releases), sans identifiants embarqués. Dans le guide, « Versions & mises à jour » permet de vérifier, lire les notes puis télécharger une version. Le bouton d’installation enregistre d’abord le projet et redémarre ensuite l’application. Aucune vérification réseau, aucun téléchargement et aucun redémarrage ne sont déclenchés automatiquement au lancement. Les préversions sont incluses pendant la phase `0.x`.

Le workflow GitHub vérifie et construit chaque push/PR. Un tag de version valide déclenche ensuite la publication de la release avec installateur, portable, fichiers Velopack et SHA-256. Voir [le guide de release](docs/RELEASING.md). Aucun compte externe n’est nécessaire pour concevoir une boîte localement.

Les exécutables de développement ne sont pas signés avec un certificat d’éditeur. La signature et le cycle réel installation → mise à jour restent à valider avant distribution publique.

Documentation : [intégration Rust Velopack](https://docs.velopack.io/getting-started/rust), [packaging](https://docs.velopack.io/packaging/overview).
