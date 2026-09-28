# Publier une version de Boxmaker

Le dépôt public est [Thomas-TP/BoxMaker](https://github.com/Thomas-TP/BoxMaker). Les binaires sont distribués par GitHub Releases ; Velopack utilise cette même source pour les mises à jour demandées depuis l’application.

## Préparer

1. Mettre à jour `package.json`, `[workspace.package].version` dans `Cargo.toml` et `src-tauri/tauri.conf.json` avec **la même version SemVer**. `cargo check` actualise les versions locales de `Cargo.lock`.
2. Déplacer les changements pertinents de `[Unreleased]` vers une nouvelle section datée dans `CHANGELOG.md`.
3. Créer `docs/releases/vX.Y.Z.md` : problème résolu, fonctionnalités, installation et limites connues. Ces notes sont intégrées au paquet Velopack et affichées dans GitHub et l’application.
4. Exécuter `bun run check`, `bun run release:check`, puis `bun run desktop:build`. Tester les exports et, pour les modifications mécaniques, préciser ce qui a réellement été imprimé.
5. Pour un paquet local : `dotnet tool restore`, puis `bun run package:windows -- -SkipBuild` et `bun run package:msix -- -SkipBuild`. Avant de taguer, comparer `store/identity.json` aux trois valeurs actuelles de **Product identity** dans Partner Center.

## Publier

Après avoir committé les changements sur `main`, créer un **tag annoté** de même version et pousser ce tag. Exemple pour une future version :

```powershell
git tag -a v0.1.1 -m "Boxmaker v0.1.1"
git push origin main
git push origin v0.1.1
```

Le workflow `Desktop · checks & releases` :

1. Installe les outils épinglés et les dépendances verrouillées.
2. Exécute Biome, TypeScript, rustfmt, tests Rust, Clippy et le contrôle versions/changelog/notes.
3. Compile les binaires Windows x64 et ARM64, prépare les paquets de mise à jour Velopack et les MSIX de soumission au Store, puis recalcule les SHA-256. Le runner ARM64 vérifie également l’en-tête machine du binaire.
4. Conserve les fichiers publics et les deux MSIX non signés en artefacts CI pendant 14 jours. Les MSIX ne sont pas téléversés dans GitHub Releases ; récupérer les artefacts `boxmaker-store-submission-x64` et `boxmaker-store-submission-arm64` du run validé pour Partner Center.
5. Construit et vérifie sur un runner macOS un DMG universel à signature ad hoc, sans essai manuel de l’interface.
6. **Pour un tag seulement**, publie les canaux Velopack `win` et `win-arm64`, retire de la release publique les EXE non signés, ZIP portables et fichiers Velopack facultatifs, télécharge et vérifie la signature de l’installateur Web Microsoft, ajoute le DMG, recalcule les SHA-256 et publie les notes lorsque tout a réussi. Les MSIX non signés restent exclus.

Les versions `0.x` et celles avec suffixe sont marquées **préversions**. La source Velopack accepte les préversions pour cette phase de développement. Le téléchargement et le redémarrage restent demandés explicitement par l’utilisateur ; il peut enregistrer le projet avant redémarrage ou installer directement en abandonnant les modifications non enregistrées.

Une release déjà publiée n’est jamais écrasée automatiquement. En cas de correction après publication, augmenter la version. Si un upload échoue en laissant un brouillon, relancer le job permet de le compléter.

## Vérifier après publication

- Contrôler le statut GitHub Actions et les notes affichées.
- Télécharger `Boxmaker-Windows.exe` et `Boxmaker-X.Y.Z-macOS.dmg`, puis vérifier leurs empreintes dans `SHA256SUMS.txt`. Les artefacts CI de soumission Store restent non signés et ne sont pas destinés à une installation directe.
- Tester une installation propre et la migration depuis la version précédente.
- Vérifier `releases.win.json` et `releases.win-arm64.json`, chacun avec son paquet `.nupkg` de même version. Le canal x64 historique doit rester `win` pour préserver les mises à jour des installations existantes.
- Ne pas annoncer un cycle installation/mise à jour comme validé avant de l’avoir réellement testé.

Le workflow produit actuellement des packages complets. Les mises à jour différentielles peuvent être ajoutées plus tard en téléchargeant le paquet précédent avant `vpk pack`.

## Éditeur Windows et signature

`Thomas Prud'homme` est le nom d'éditeur inscrit dans les métadonnées de l'application et du paquet Velopack. Ce champ ne constitue pas une vérification d'identité : tant que les exécutables ne sont pas signés, Windows peut afficher « Éditeur inconnu ». Le nom affiché par Windows pour une signature valide provient de l'identité vérifiée dans le certificat. `ThomasTP` reste un pseudonyme de projet, pas l'identité à faire certifier. Les EXE Velopack non signés restent réservés aux artefacts CI ; la release publique présente l’installateur Web signé par Microsoft.

Velopack doit signer les exécutables de l'application **et** ses propres exécutables d'installation et de mise à jour pendant `vpk pack`. `scripts/package-windows.ps1` accepte deux méthodes exclusives :

- `-SignParams '<paramètres signtool>'` pour un certificat ou service compatible avec SignTool ; utiliser des chemins absolus et l'horodatage SHA-256.
- `-AzureTrustedSignFile <chemin-vers-metadata.json>` pour Microsoft Artifact Signing, après validation de l'identité, attribution du rôle de signataire et authentification Azure sur la machine de build.

Quand une méthode de signature est fournie, le script vérifie que la signature Authenticode de l'installateur final est valide avant de produire les sommes de contrôle. Sans méthode, le paquet reste explicitement non signé. La CI ne possède actuellement ni identité de signature ni accès à un certificat : **les releases automatiques restent non signées** jusqu'à sa configuration et validation. Ne pas stocker de certificat, de mot de passe ou de jeton dans le dépôt, les arguments enregistrés par la CI ou une conversation.

Pour une personne physique établie en Suisse, [Microsoft Artifact Signing Public Trust](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart) n'est actuellement pas disponible ; le service accepte en revanche les organisations suisses vérifiées. Une personne physique doit obtenir une solution de signature de code auprès d'une autorité de certification qui accepte son identité et fournit une clé protégée ou un service de signature. Les offres [« Individual Validated Code Signing » de SSL.com](https://www.ssl.com/fr/products/software-integrity/code-signing/iv/) et [« Standard Code Signing » de Certum](https://www.certum.eu/en/code-signing-certificates/) sont des pistes à confirmer auprès de l'émetteur pour une personne domiciliée en Suisse avant tout achat. L'identité figurant sur le certificat est déterminée par cette vérification, pas par `packAuthors`.

Si le coût doit être nul, le [compte développeur Microsoft Store](https://learn.microsoft.com/en-us/windows/apps/publish/faq/get-started-with-the-microsoft-store) permet de faire signer gratuitement un paquet **MSIX publié via le Store**. Il ne donne pas de certificat personnel et ne signe pas l'EXE Velopack distribué sur GitHub ; la distribution directe du MSIX hors Store demande aussi une signature fiable propre. Une application Store peut rester [introuvable par recherche et accessible seulement par lien direct](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/visibility-options), mais elle doit être soumise et certifiée par Microsoft. La [SignPath Foundation](https://signpath.org/terms.html) propose aussi une signature gratuite aux projets sous licence open source admissibles ; son certificat affiche **SignPath Foundation** comme éditeur. Le dépôt Boxmaker n'a actuellement pas de licence open source, et l'ajout d'une telle licence serait une décision distincte du propriétaire.

## Canal Microsoft Store

**Boxmaker** est réservé comme « MSIX or PWA app », Store ID `9MX7QLK05FJP`. `store/identity.json` contient les trois valeurs exactes de la page « Product identity ». Ces valeurs ne sont pas des secrets ; ne pas inventer le `Publisher` à partir du nom public. Le nom public de l’éditeur du compte a été changé de « Parkour Pixels » à « ThomasTP » le 27 septembre 2026. La page Product identity fournit désormais `PublisherDisplayName=ThomasTP`, tandis que `Package/Identity/Name=ParkourPixels.Boxmaker` et le CN restent inchangés. Vérifier ces trois valeurs avant chaque publication. Le workflow refusera tout nouveau tag sans ce fichier. La version SemVer `X.Y.Z` devient `X.Y.Z.0` dans le manifeste MSIX. Le paquet contient `boxmaker-store.txt` afin que l'application indique que les mises à jour de cette installation passent par la bibliothèque Microsoft Store. La version Velopack continue d'utiliser GitHub.

Les fichiers `Boxmaker-X.Y.Z-x64-Store-submission-UNSIGNED.msix` et `Boxmaker-X.Y.Z-arm64-Store-submission-UNSIGNED.msix` sont destinés à Partner Center. Ils sont disponibles temporairement dans les artefacts Actions, jamais dans GitHub Releases. **Ne pas proposer leur installation directe** : Microsoft signe et distribue les versions installables depuis la [fiche Store Boxmaker](https://apps.microsoft.com/detail/9MX7QLK05FJP), avec sélection automatique de l’architecture. Le [mode direct officiel](https://apps.microsoft.com/detail/9MX7QLK05FJP?mode=direct) télécharge un petit installateur Web signé par Microsoft qui récupère l’application depuis le Store. Cet EXE est joint à la release GitHub sous le nom `Boxmaker-Windows.exe` : le télécharger depuis `https://get.microsoft.com/installer/download/9MX7QLK05FJP`, vérifier `Get-AuthenticodeSignature` (`Status=Valid`, signataire `Microsoft Corporation`) et ajouter son SHA-256 à `SHA256SUMS.txt`. Il nécessite Internet et installe la version Store disponible au moment de l’exécution, pas nécessairement la version historique de la release GitHub. Un MSIX ne peut être ajouté à GitHub que si son fichier précis a une signature Windows valide et que sa redistribution est autorisée ; la signature d'une installation via le Store ne signe pas automatiquement le paquet de soumission.

Dans la soumission Store, choisir « Make this product available but not discoverable in the Store » puis « Direct link only ». Cela crée une fiche accessible par URL mais absente de la recherche ; ce n'est pas une audience privée. Le Store exige une fiche, des ressources et une certification même avec cette visibilité. Ne pas soumettre le paquet comme application MSI/EXE : cette voie exige une signature Authenticode propre, que le compte développeur ne fournit pas.
