# Publier une version de Boxmaker

Le dépôt public est [Thomas-TP/BoxMaker](https://github.com/Thomas-TP/BoxMaker). Les binaires sont distribués par GitHub Releases ; Velopack utilise cette même source pour les mises à jour demandées depuis l’application.

## Préparer

1. Mettre à jour `package.json`, `[workspace.package].version` dans `Cargo.toml` et `src-tauri/tauri.conf.json` avec **la même version SemVer**. `cargo check` actualise les versions locales de `Cargo.lock`.
2. Déplacer les changements pertinents de `[Unreleased]` vers une nouvelle section datée dans `CHANGELOG.md`.
3. Créer `docs/releases/vX.Y.Z.md` : problème résolu, fonctionnalités, installation et limites connues. Ces notes sont intégrées au paquet Velopack et affichées dans GitHub et l’application.
4. Exécuter `bun run check`, `bun run release:check`, puis `bun run desktop:build`. Tester les exports et, pour les modifications mécaniques, préciser ce qui a réellement été imprimé.
5. Pour un paquet local : `dotnet tool restore`, puis `bun run package:windows -- -SkipBuild` et `bun run package:msix -- -SkipBuild`. Avant de taguer, comparer `store/identity.json` aux trois valeurs actuelles de **Product identity** dans Partner Center ; une demande de changement de nom d’éditeur est en cours de vérification.

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
3. Compile le binaire Windows, crée l’installateur et l’archive portable avec Velopack, ainsi qu’un MSIX de soumission au Store avec MakeAppx, puis recalcule les SHA-256.
4. Conserve les fichiers de build en artifacts pendant 14 jours.
5. **Pour un tag seulement**, crée un brouillon de release, joint tous les fichiers et les notes, puis publie lorsque tout a réussi.

Les versions `0.x` et celles avec suffixe sont marquées **préversions**. La source Velopack accepte les préversions pour cette phase de développement. Le téléchargement et le redémarrage restent demandés explicitement par l’utilisateur ; il peut enregistrer le projet avant redémarrage ou installer directement en abandonnant les modifications non enregistrées.

Une release déjà publiée n’est jamais écrasée automatiquement. En cas de correction après publication, augmenter la version. Si un upload échoue en laissant un brouillon, relancer le job permet de le compléter.

## Vérifier après publication

- Contrôler le statut GitHub Actions et les notes affichées.
- Télécharger l’installateur ou le ZIP portable et vérifier son empreinte dans `SHA256SUMS.txt`. Le MSIX joint sert uniquement à la soumission Partner Center ; il n'est pas signé pour une installation directe.
- Tester une installation propre et la migration depuis la version précédente.
- Vérifier `releases.win.json` et la présence du paquet `.nupkg` de même version.
- Ne pas annoncer un cycle installation/mise à jour comme validé avant de l’avoir réellement testé.

Le workflow produit actuellement des packages complets. Les mises à jour différentielles peuvent être ajoutées plus tard en téléchargeant le paquet précédent avant `vpk pack`.

## Éditeur Windows et signature

`Thomas Prud'homme` est le nom d'éditeur inscrit dans les métadonnées de l'application et du paquet Velopack. Ce champ ne constitue pas une vérification d'identité : tant que les exécutables ne sont pas signés, Windows peut afficher « Éditeur inconnu ». Le nom affiché par Windows pour une signature valide provient de l'identité vérifiée dans le certificat. `ThomasTP` reste un pseudonyme de projet, pas l'identité à faire certifier.

Velopack doit signer les exécutables de l'application **et** ses propres exécutables d'installation et de mise à jour pendant `vpk pack`. `scripts/package-windows.ps1` accepte deux méthodes exclusives :

- `-SignParams '<paramètres signtool>'` pour un certificat ou service compatible avec SignTool ; utiliser des chemins absolus et l'horodatage SHA-256.
- `-AzureTrustedSignFile <chemin-vers-metadata.json>` pour Microsoft Artifact Signing, après validation de l'identité, attribution du rôle de signataire et authentification Azure sur la machine de build.

Quand une méthode de signature est fournie, le script vérifie que la signature Authenticode de l'installateur final est valide avant de produire les sommes de contrôle. Sans méthode, le paquet reste explicitement non signé. La CI ne possède actuellement ni identité de signature ni accès à un certificat : **les releases automatiques restent non signées** jusqu'à sa configuration et validation. Ne pas stocker de certificat, de mot de passe ou de jeton dans le dépôt, les arguments enregistrés par la CI ou une conversation.

Pour une personne physique établie en Suisse, [Microsoft Artifact Signing Public Trust](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart) n'est actuellement pas disponible ; le service accepte en revanche les organisations suisses vérifiées. Une personne physique doit obtenir une solution de signature de code auprès d'une autorité de certification qui accepte son identité et fournit une clé protégée ou un service de signature. Les offres [« Individual Validated Code Signing » de SSL.com](https://www.ssl.com/fr/products/software-integrity/code-signing/iv/) et [« Standard Code Signing » de Certum](https://www.certum.eu/en/code-signing-certificates/) sont des pistes à confirmer auprès de l'émetteur pour une personne domiciliée en Suisse avant tout achat. L'identité figurant sur le certificat est déterminée par cette vérification, pas par `packAuthors`.

Si le coût doit être nul, le [compte développeur Microsoft Store](https://learn.microsoft.com/en-us/windows/apps/publish/faq/get-started-with-the-microsoft-store) permet de faire signer gratuitement un paquet **MSIX publié via le Store**. Il ne donne pas de certificat personnel et ne signe pas l'EXE Velopack distribué sur GitHub ; la distribution directe du MSIX hors Store demande aussi une signature fiable propre. Une application Store peut rester [introuvable par recherche et accessible seulement par lien direct](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/visibility-options), mais elle doit être soumise et certifiée par Microsoft. La [SignPath Foundation](https://signpath.org/terms.html) propose aussi une signature gratuite aux projets sous licence open source admissibles ; son certificat affiche **SignPath Foundation** comme éditeur. Le dépôt Boxmaker n'a actuellement pas de licence open source, et l'ajout d'une telle licence serait une décision distincte du propriétaire.

## Canal Microsoft Store

**Boxmaker** est réservé comme « MSIX or PWA app », Store ID `9MX7QLK05FJP`. `store/identity.json` contient les trois valeurs exactes de la page « Product identity ». Ces valeurs ne sont pas des secrets ; ne pas inventer le `Publisher` à partir du nom public. Une demande de renommage de l’éditeur du compte de « Parkour Pixels » vers « ThomasTP » a été déposée le 27 septembre 2026 et est en cours de vérification : **relire et ajuster `store/identity.json` après validation de Microsoft et avant le prochain tag**. Le workflow refusera tout nouveau tag sans ce fichier. La version SemVer `X.Y.Z` devient `X.Y.Z.0` dans le manifeste MSIX. Le paquet contient `boxmaker-store.txt` afin que l'application indique que les mises à jour de cette installation passent par la bibliothèque Microsoft Store. La version Velopack continue d'utiliser GitHub.

Le fichier `Boxmaker-X.Y.Z-Store-submission-UNSIGNED.msix` joint à chaque release GitHub est destiné à Partner Center. **Ne pas proposer son installation directe** : c'est Microsoft qui signe et distribue la version installable après certification. Le fichier `Swiss3Design.Boxmaker-win-Setup.exe` reste non signé tant qu'aucun certificat Authenticode propre n'est configuré ; les notes de chaque release doivent le signaler explicitement et donner, une fois le produit approuvé, le lien direct de la fiche Store comme option signée.

Dans la soumission Store, choisir « Make this product available but not discoverable in the Store » puis « Direct link only ». Cela crée une fiche accessible par URL mais absente de la recherche ; ce n'est pas une audience privée. Le Store exige une fiche, des ressources et une certification même avec cette visibilité. Ne pas soumettre le paquet comme application MSI/EXE : cette voie exige une signature Authenticode propre, que le compte développeur ne fournit pas.
