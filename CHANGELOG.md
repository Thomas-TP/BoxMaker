# Historique des versions

Les changements visibles sont regroupés par version. Chaque tag `vX.Y.Z` possède des notes détaillées dans `docs/releases/vX.Y.Z.md` ; les versions publiées disposent de leurs binaires dans GitHub Releases.

## [Unreleased]

Aucun changement non publié.

## [0.8.1] - 2026-09-27

### Ajouté

- Préversion macOS en DMG universel Intel/Apple Silicon, construite sur macOS par GitHub Actions et publiée à côté des fichiers Windows.
- Signature ad hoc sans certificat Apple ; l’ouverture nécessite une autorisation manuelle dans les réglages de sécurité macOS.

### Adapté

- Le bouton de mise à jour affiche la plateforme utilisée. La version Mac renvoie aux releases GitHub ; l’installation automatique Velopack reste réservée à Windows.

### Limite

- La compilation et l’intégrité du DMG sont vérifiées automatiquement, mais l’interface et les exports macOS n’ont pas été testés en conditions réelles sur un Mac.

## [0.8.0] - 2026-09-27

### Ajouté

- Paquet MSIX pour soumission à Microsoft Store, avec l’identité attribuée à Boxmaker dans Partner Center.
- Fiche Store en français suisse, gratuite et accessible uniquement par lien direct.
- Canal de mise à jour adapté à la provenance de l’installation : GitHub pour Velopack, Microsoft Store pour le MSIX.

### Précisé

- L’EXE téléchargé sur GitHub n’est pas signé ; le MSIX joint à la release est un paquet de soumission non signé. Seul le paquet distribué par Microsoft Store sera signé par Microsoft après certification.

## [0.7.0] - 2026-09-27

### Amélioré

- Scellé redessiné en petit verrou vertical imprimé à plat, affleurant dans le coin arrière : suppression de l’oreille latérale de 8,5 mm et de l’allongement arrière de 14 mm.
- Tête sacrificielle à rompre ; la tige retenue dans la coque n’empêche ensuite plus le couvercle de coulisser grâce à sa fente ouverte vers l’arrière.
- Contrôle géométrique de l’enveloppe et des collisions du scellé avec la coque et le couvercle. Les très petites boîtes réservent seulement la largeur nécessaire pour éloigner le scellé du ressort.
- Aperçu 3D montrant le verrou vertical assemblé et la même pièce à plat pour l’impression.
- Projets v5 : les projets scellés antérieurs perdent leur ancienne pesée, car leur géométrie change ; les projets sans scellé conservent leurs réglages.

### Limites

- Enclenchement, force de rupture, résistance au transport et retrait du fragment restent à valider sur une impression réelle.

## [0.6.0] - 2026-09-26

### Ajouté

- Scellé sacrificiel entièrement imprimable pour la boîte coulissante : deux ancrages à crochets et une languette centrale à rompre avant l’ouverture.
- Logements intégrés à la boîte et au couvercle, troisième pièce dans l’aperçu et l’export 3MF complet, export individuel pour réimprimer le scellé.
- Option activée pour les nouveaux projets ; les anciens projets conservent leur géométrie sans scellé. Format de projet v4.

### Limites

- Mécanisme expérimental à valider par impression : enclenchement, rupture visible, extraction des restes et résistance pendant le transport n’ont pas été mesurés physiquement.

## [0.5.0] - 2026-09-11

### Amélioré

- Nouvelle interface : tons bleu et ardoise, aperçu 3D agrandi, typographie plus lisible et panneaux de configuration et d’export réorganisés.
- Export complet placé en haut, avec raccourci dans la barre d’outils sur les petites fenêtres.
- Détails de cavité repliables, ancien modèle déplacé dans les réglages avancés et sélection du contenu des champs numériques au focus.
- Mises à jour accessibles depuis la barre d’outils, dans une fenêtre dédiée ; état de téléchargement conservé à sa fermeture.
- Installation et redémarrage sans sauvegarde obligatoire. L’enregistrement du projet reste une action facultative distincte.
- Suppression du bloc comparatif « PLA en moins ».
- Invalidation de la pesée après un changement d’imprimante ou de marge plateau, car l’orientation de la boîte peut changer.
- Fenêtres de dialogue avec gestion native du focus et fermeture par Échap.

## [0.4.0] - 2026-09-11

### Ajouté

- Export de la boîte et du couvercle dans un seul fichier 3MF, avec deux objets indépendants posés à plat côte à côte.
- Option « Boîte + couvercle » sélectionnée par défaut ; les exports individuels STL/3MF restent disponibles.
- Export complet des trois pièces pour le modèle historique à clavette.
- Contrôle des limites de chaque pièce avant export complet, tests des maillages et des positions, et vérification d’import dans Bambu Studio.

## [0.3.0] - 2026-09-09

### Amélioré

- Orientation automatique parmi les six permutations : résultat indépendant de l’ordre de saisie, priorité au volume imprimable puis à une faible hauteur.
- Languette à épaisseur progressive, dimensionnée par calcul de flexion pour une pression nominale proche de 4 N ; ancrage renforcé près du rail.
- Jeu d’insertion de 0,3 mm par face par défaut, calage séparé à zéro et minima mécaniques adaptatifs ; détail des espaces dans l’interface.
- Animation d’ouverture cohérente avec le profil de flexion calculé et butée compatible avec le retrait initial.
- Projets v3 ; migration des projets coulissants v2 avec conservation du calage choisi et invalidation de la pesée. Modèle à clavette conservé.
- Tests des permutations, calcul de poutre, collisions et exports ; parcours navigateur étendu aux orientations et migrations.
- Répertoires de compilation et d’exports exclus de la surveillance Vite pour éviter les fichiers verrouillés par Velopack.

## [0.2.1] - 2026-09-08

### Corrigé

- Reconstruction des micro-arêtes avant export pour éviter des triangles dégénérés lors de la conversion STL et 3MF.
- Test de régression sur les coordonnées réellement sérialisées, y compris une petite boîte avec des parois épaisses.
- Première distribution du nouveau mécanisme ; la publication v0.2.0 a été interrompue avant création de la release.

## [0.2.0] - 2026-09-08

### Amélioré

- Fermeture coulissante à languette arrière intégrée : deux pièces au lieu de trois.
- Parois et fond amincis avec nervures, coins arrondis ; 32 à 33 % de matière calculée en moins sur trois formats à cavité utile identique.
- Démonstration du déverrouillage et du retrait, avec conservation de l’angle de vue pendant le mouvement.
- Compatibilité des projets v1 et conservation du modèle à clavette.
- Tests de verrouillage, collisions, volumes fermés, espace objet et gain de matière ; intégration de Manifold et CMake.
- Documentation du mécanisme et du protocole d’essai physique restant à effectuer.

## [0.1.1] - 2026-09-08

### Corrigé

- Fins de ligne CSS cohérentes sur les nouveaux checkouts Windows.
- Publication de tous les fichiers couverts par les empreintes SHA-256.
- Première préversion distribuée ; v0.1.0 avait échoué en CI avant publication.

## [0.1.0] - 2026-09-08

### Ajouté

- Générateur paramétrique Rust : boîte en PLA, couvercle coulissant et clavette rigide.
- Aperçu 3D fermé, éclaté et pièces séparées à plat.
- Profils Bambu Lab P1S et Creality K2 classique, marge de plateau et blocage des exports hors volume utile.
- Export STL binaire et 3MF en millimètres, une pièce par fichier.
- Tarifs publics 2026 de la Poste suisse pour lettres, colis et encombrants, avec comparaison des services et rabais en ligne admissible.
- Poids estimé du PLA, coût matière et saisie du poids total réel ; invalidation d’une ancienne pesée lorsque la boîte change.
- Sauvegarde et ouverture de projets JSON, dialogue Windows « Enregistrer sous ».
- Installateur et archive portable Velopack ; vérification manuelle des mises à jour sur GitHub avec notes de version et sauvegarde avant redémarrage.
- Contrôles automatisés, compilation Windows, notes de release et fichiers SHA-256.

### Limites de cette préversion

- Le mécanisme et la résistance à l’expédition attendent une validation par impression physique.
- Les profils slicer, zones exclues et G-code ne sont pas intégrés aux fichiers 3MF.
- Tarifs suisses uniquement, hors options et surcoûts de traitement ; pesée finale nécessaire.
- Exécutables non signés par un certificat d’éditeur.

[Unreleased]: https://github.com/Thomas-TP/BoxMaker/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/Thomas-TP/BoxMaker/releases/tag/v0.2.1
[0.2.0]: https://github.com/Thomas-TP/BoxMaker/releases/tag/v0.2.0
[0.1.1]: https://github.com/Thomas-TP/BoxMaker/releases/tag/v0.1.1
[0.1.0]: https://github.com/Thomas-TP/BoxMaker/releases/tag/v0.1.0
