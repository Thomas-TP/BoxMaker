# Publier une version de Boxmaker

Le dépôt public est [Thomas-TP/BoxMaker](https://github.com/Thomas-TP/BoxMaker). Les binaires sont distribués par GitHub Releases ; Velopack utilise cette même source pour les mises à jour demandées depuis l’application.

## Préparer

1. Mettre à jour `package.json`, `[workspace.package].version` dans `Cargo.toml` et `src-tauri/tauri.conf.json` avec **la même version SemVer**. `cargo check` actualise les versions locales de `Cargo.lock`.
2. Déplacer les changements pertinents de `[Unreleased]` vers une nouvelle section datée dans `CHANGELOG.md`.
3. Créer `docs/releases/vX.Y.Z.md` : problème résolu, fonctionnalités, installation et limites connues. Ces notes sont intégrées au paquet Velopack et affichées dans GitHub et l’application.
4. Exécuter `bun run check`, `bun run release:check`, puis `bun run desktop:build`. Tester les exports et, pour les modifications mécaniques, préciser ce qui a réellement été imprimé.
5. Pour un paquet local : `dotnet tool restore`, puis `bun run package:windows -- -SkipBuild`.

## Publier

Après avoir committé les changements sur `main`, créer un **tag annoté** de même version et pousser ce tag. Exemple pour une future version :

```powershell
git tag -a v0.1.1 -m "Boxmaker v0.1.1"
git push origin main
git push origin v0.1.1
```

Le workflow `Windows · checks & releases` :

1. Installe les outils épinglés et les dépendances verrouillées.
2. Exécute Biome, TypeScript, rustfmt, tests Rust, Clippy et le contrôle versions/changelog/notes.
3. Compile le binaire Windows, crée l’installateur et l’archive portable avec Velopack, calcule les SHA-256.
4. Conserve les fichiers de build en artifacts pendant 14 jours.
5. **Pour un tag seulement**, crée un brouillon de release, joint tous les fichiers et les notes, puis publie lorsque tout a réussi.

Les versions `0.x` et celles avec suffixe sont marquées **préversions**. La source Velopack accepte les préversions pour cette phase de développement. Le téléchargement et le redémarrage restent demandés explicitement par l’utilisateur ; un projet est enregistré avant redémarrage.

Une release déjà publiée n’est jamais écrasée automatiquement. En cas de correction après publication, augmenter la version. Si un upload échoue en laissant un brouillon, relancer le job permet de le compléter.

## Vérifier après publication

- Contrôler le statut GitHub Actions et les notes affichées.
- Télécharger l’installateur ou le ZIP portable et vérifier son empreinte dans `SHA256SUMS.txt`.
- Tester une installation propre et la migration depuis la version précédente.
- Vérifier `releases.win.json` et la présence du paquet `.nupkg` de même version.
- Ne pas annoncer un cycle installation/mise à jour comme validé avant de l’avoir réellement testé.

Le workflow produit actuellement des packages complets. Les mises à jour différentielles peuvent être ajoutées plus tard en téléchargeant le paquet précédent avant `vpk pack`. La signature des exécutables nécessite un certificat ou un service de signature configuré séparément ; aucun secret de signature n’est inclus dans le dépôt.
