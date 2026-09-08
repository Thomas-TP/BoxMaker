# Historique des versions

Les changements visibles sont regroupés par version. Chaque tag `vX.Y.Z` possède des notes détaillées dans `docs/releases/vX.Y.Z.md` et une release GitHub avec ses binaires Windows.

## [Unreleased]

Aucun changement non publié.

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

[Unreleased]: https://github.com/Thomas-TP/BoxMaker/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Thomas-TP/BoxMaker/releases/tag/v0.1.0
