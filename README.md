# Boxmaker · Swiss3Design

Application Windows locale pour concevoir une boîte d’expédition imprimée en PLA, visualiser ses deux pièces et comparer les tarifs de la Poste suisse. La fermeture coulissante à pression remplace la clavette séparée ; le modèle à clavette reste disponible.

[Télécharger la préversion Windows](https://github.com/Thomas-TP/BoxMaker/releases) · [Notes de version](CHANGELOG.md) · [Compilation et releases](https://github.com/Thomas-TP/BoxMaker/actions)

## Utilisation

L’installateur Velopack se trouve dans `artifacts/releases/Swiss3Design.Boxmaker-win-Setup.exe`. Un ZIP portable est également produit. Le binaire autonome est `target/release/boxmaker.exe` et nécessite le runtime Microsoft WebView2.

1. Saisir les trois dimensions en **mm**, dans n’importe quel ordre : le modèle coulissant choisit automatiquement une orientation imprimable et basse. Renseigner le poids en **g**. Le jeu d’insertion vaut 0,3 mm par face ; ajouter séparément le calage nécessaire.
2. Choisir **Bambu Lab P1S (256³ mm)** ou **Creality K2 classique (260³ mm)**.
3. Ajuster parois, fond et jeu dans les réglages avancés. Le profil est PLA ; le prix au kilo est modifiable.
4. Inspecter les vues fermée, éclatée, pièces à plat et ouverture. Le curseur montre la pression sur la languette arrière puis le retrait du couvercle.
5. Cliquer sur **Exporter les 2 pièces** pour obtenir **un seul 3MF contenant la boîte et le couvercle**, sous forme d’objets indépendants posés à plat. Réorganiser les pièces dans le slicer ou utiliser plusieurs plateaux si nécessaire. Les exports individuels STL et 3MF restent disponibles via le menu « Pièce ». Le 3MF contient une géométrie en millimètres, sans profil machine ou G-code. Imprimer le couvercle face lisse dessous, nervures dessus. L’export complet de l’ancien modèle contient trois pièces, dont la clavette posée sur sa tête.
6. Dans Bambu Studio ou Creality Print, vérifier l’orientation, les surplombs, les zones exclues et le poids calculé. Imprimer une petite boîte d’essai avant un emballage complet.
7. Emballer, sécuriser la fermeture avec un adhésif et peser l’envoi fermé. Saisir ce poids réel dans « J’ai pesé mon envoi fermé ».

Les projets peuvent être enregistrés en `.boxmaker.json` puis rouverts. Un projet coulissant v2 est adapté au mécanisme v0.3, conserve son calage et doit être repesé ; un projet à clavette conserve sa géométrie. Modifier la géométrie ou le contenu efface le poids mesuré pour éviter un tarif calculé sur une ancienne pesée.

## Interface

L’aperçu 3D occupe le centre de l’atelier. Les paramètres sont à gauche, l’export et l’expédition à droite ; les panneaux défilent indépendamment sur les grandes fenêtres. Les détails de la cavité et du ressort se déplient sous l’aperçu. Le choix de l’ancien modèle se trouve dans les réglages avancés. Un raccourci d’export apparaît dans la barre d’outils sur les fenêtres plus étroites.

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

La nouvelle géométrie utilise Manifold via les bindings Rust `manifold-csg` pour les opérations booléennes, coins arrondis, nervures et languette. Son noyau C++ est compilé par CMake ; les scripts cherchent aussi CMake dans Visual Studio. Installer les outils CMake C++ si nécessaire. Le moteur rectiligne initial reste utilisé pour les anciens projets et la comparaison de matière. Aucun import de STL arbitraire n’est proposé.

## Validation et limites

- Les tests Rust couvrent les changements de tranche de poids, dimensions et rotations postales, encombrants, entrées invalides, export hors plateau, fermeture/orientation des maillages, volume et structure des exports.
- `scripts/browser-smoke.js` est un parcours Playwright CLI pour vérifier les limites P1S/K2, les tarifs lettre, le poids absent, les exports individuels et complets, et la sauvegarde/réouverture d’un projet.
- Le contrôle de plateau considère les pièces à plat avec rotation XY à 90° et une marge par bord. Les zones exclues spécifiques du slicer P1S ne sont pas modélisées. La vue « pièces à plat » n’est pas un placement automatique sur un plateau commun.
- La masse est estimée avec une densité de 1,24 g/cm³ et le volume solide. Le slicer, la densité réelle du filament, l’infill et la pesée finale peuvent différer. Le coût matière exclut temps, énergie, calage et affranchissement.
- **Prototype mécanique non homologué.** Cette révision n’a pas encore fait l’objet d’un essai physique. Le jeu, la fatigue de la languette PLA, les rails et la protection du contenu doivent être testés physiquement. Le petit pont arrière et les lèvres des rails sont à examiner dans le slicer. Voir [le choix mécanique et le protocole d’essai](docs/MECHANISM.md).
- Tarifs pour les envois intérieurs en Suisse seulement, datés de 2026 ; détails dans [docs/POSTAL.md](docs/POSTAL.md).

## Velopack

L’application initialise `VelopackApp` avant toute initialisation de l’interface. L’installation Windows et les fichiers de release sont générés par Velopack ; le packaging NSIS de Tauri est désactivé. WebView2 est déclaré comme prérequis de l’installateur.

Les mises à jour utilisent [les releases du dépôt GitHub](https://github.com/Thomas-TP/BoxMaker/releases), sans identifiants embarqués. Le bouton « Mises à jour » dans la barre d’outils permet de vérifier, lire les notes puis télécharger une version. « Installer et redémarrer » fonctionne sans enregistrer le projet ; une seconde action permet d’enregistrer puis d’installer. Les modifications non enregistrées sont perdues au redémarrage. Aucune vérification réseau, aucun téléchargement et aucun redémarrage ne sont déclenchés automatiquement au lancement. Les préversions sont incluses pendant la phase `0.x`.

Le workflow GitHub vérifie et construit chaque push/PR. Un tag de version valide déclenche ensuite la publication de la release avec installateur, portable, fichiers Velopack et SHA-256. Voir [le guide de release](docs/RELEASING.md). Aucun compte externe n’est nécessaire pour concevoir une boîte localement.

Les exécutables de développement ne sont pas signés avec un certificat d’éditeur. La signature et le cycle réel installation → mise à jour restent à valider avant distribution publique.

Documentation : [intégration Rust Velopack](https://docs.velopack.io/getting-started/rust), [packaging](https://docs.velopack.io/packaging/overview).
