# Contribuer à Boxmaker

Utiliser le moteur Rust comme source unique des dimensions, des maillages et des tarifs. L’interface présente les résultats ; elle ne doit pas réimplémenter ces calculs.

## Vérifications

```powershell
bun install --frozen-lockfile
bun run format
bun run check
bun run release:check
bun run desktop:build
```

Pour un changement de géométrie, ajouter un cas utile aux tests de volume/fermeture et inspecter les exports dans un slicer. Pour un tarif, citer la page officielle et tester juste avant, au seuil et juste après.

Les messages de commit suivent une convention simple : `feat:`, `fix:`, `docs:`, `test:`, `chore:`. Éviter de mélanger une modification mécanique avec un changement de tarif sans lien.

Les sorties `target`, `dist`, `node_modules`, `artifacts`, les profils locaux des slicers et les captures de tests ne doivent pas être committés. Les installateurs sont des pièces jointes aux releases, pas des fichiers source.

Voir [le guide de release](docs/RELEASING.md) pour publier une version.
