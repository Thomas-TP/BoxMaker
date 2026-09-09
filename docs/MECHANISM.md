# Fermeture coulissante à pression — v0.3

Deux pièces : coque nervurée et couvercle coulissant. Les rails reprennent la retenue verticale ; une dent sur la languette bloque le retrait. Appuyer sur la zone striée à l’arrière, tirer de quelques millimètres, relâcher puis retirer le couvercle.

## Orientation et espace utile

Le moteur compare les six permutations des dimensions de l’objet. Il minimise d’abord le dépassement du volume utile de l’imprimante, puis la hauteur extérieure. À égalité, il préfère le coulissement dans la plus grande dimension horizontale, puis le volume extérieur et un ordre numérique stable. C’est une recherche discrète selon ces critères, pas une optimisation de toutes les formes ou orientations obliques possibles.

L’ordre de saisie ne change donc ni les dimensions finales ni les triangles. Un objet long peut être placé verticalement si c’est nécessaire pour tenir dans l’imprimante. Les rails, la fermeture et les marges de plateau sont inclus dans cette décision.

La cavité demandée est `objet orienté + 2 × (calage + jeu d’insertion)`. Réglages initiaux : calage nul et jeu de 0,3 mm par face. Pour les très petits objets, le dimensionnement de la languette peut imposer une cavité plus grande : ce supplément est annoncé dans l’interface. Les nervures et l’espace de flexion restent hors de l’enveloppe libre annoncée.

Exemple de 100 × 70 × 30 mm : objet orienté 70 × 100 × 30 mm ; cavité 70,6 × 100,6 × 30,6 mm ; extérieur 75,4 × 109,4 × 36,5139 mm. À jeu mécanique de 0,3 mm, le verrou réserve 5,2 mm à l’arrière et son mouvement environ 1,16 mm au-dessus de la cavité. Masse solide calculée : environ 42 g à 1,24 g/cm³. La v0.2.1 donnait 55,45 g avec 5 mm de calage par face ; cette différence inclut donc une réduction du volume utile et n’établit pas une résistance équivalente.

## Sources et hypothèses PLA

Le [guide Bayer sur les encliquetages, copie MIT](https://fab.cba.mit.edu/classes/S62.12/people/vernelle.noel/Plastic_Snap_fit_design.pdf) recommande le retour sans déformation imposée au repos et décrit les sections progressives pour mieux répartir la déformation. Ses valeurs admissibles concernent ses matériaux ; elles ne sont pas reprises comme limites PLA. Il explique aussi que la souplesse des deux pièces doit être considérée lorsque les deux se déforment.

La [fiche technique Prusament PLA, version 1.1](https://www.prusa3d.com/file/370474/prusament-pla-technical-data-sheet.pdf), page 2, donne pour ses éprouvettes horizontales un module de flexion de 3,1 GPa, un module de traction de 2,3 GPa et un allongement à la limite d’élasticité de 2,9 %. La page 3 précise les conditions d’impression et la forte dépendance des résultats aux réglages. Ce sont des données d’un PLA particulier, pas une caractérisation de tous les PLA.

Le calcul utilise **E = 3 100 MPa** comme hypothèse nominale de flexion. La plage **2 000 à 3 500 MPa** affichée sert à explorer la sensibilité de la force ; elle ne garantit pas que tous les filaments y appartiennent. Aucun paramètre Bambu, diamètre de buse ou hauteur de couche propre à l’utilisateur n’entre dans le dimensionnement.

## Calcul de la languette

Modèle de poutre en porte-à-faux, petites déformations, ancrage rigide, section rectangulaire de largeur b et épaisseur h(x) linéairement décroissante. Unités : N, mm, MPa. L est la longueur jusqu’au crochet ; le doigt charge la zone striée à a = L − 4,4 mm.

```text
h(x) = h_racine + (h_bout − h_racine) × x/L
I(x) = b × h(x)^3 / 12
C(y) = intégrale de 0 à min(a,y) de (a−x)(y−x) / (E × I(x)) dx
F = 0,85 / C(L)
déplacement(y) = F × C(y)
épsilon(x) = 6 × F × (a−x) / (E × b × h(x)^2), pour 0 ≤ x ≤ a
```

L’intégration utilise 96 intervalles au point milieu. Un test la compare à la formule exacte d’une poutre uniforme chargée avant son extrémité. Le profil de déplacement calculé alimente également la vue d’ouverture et les tests de dégagement.

Les candidats explorés ont L de 18 à 32 mm, b de 8 à 18 mm et une racine de 1,6 à 2,8 mm. Le bout vaut environ 65 % de la racine, arrondi au pas géométrique de 0,2 mm et au minimum 1,2 mm. Ce pas est un choix de dimensions, pas une consigne de hauteur de couche. Les candidats retenus visent 3,6 à 4,4 N, une déformation nominale maximale de 0,6 % et une déformation à la butée de 0,8 %. Ces deux plafonds sont des choix de conception prudents, **pas des limites de fatigue établies**.

Le choix final combine l’agrandissement de cavité nécessaire, l’écart à 4 N, la déformation et le volume du ressort. La languette est située près du rail et son ancrage possède un renfort local pour réduire l’influence de la flexion du grand panneau.

| Valeur du modèle par défaut | Résultat |
| --- | ---: |
| Longueur / largeur | 22 / 10 mm |
| Épaisseur racine / bout | 1,8 / 1,2 mm |
| Déplacement demandé au crochet | 0,85 mm |
| Force nominale calculée | 3,99 N |
| Sensibilité pour E de 2 à 3,5 GPa | 2,57 à 4,50 N |
| Déformation maximale à l’ouverture | 0,419 % |
| Déformation estimée à la butée | 0,662 % |

Sous la même hypothèse de module et d’ancrage rigide, la languette uniforme v0.2.1 (longueur jusqu’au crochet 33,3 mm, largeur 12 mm, épaisseur 1,2 mm, charge à 25,3 mm) nécessitait environ 0,57 N pour déplacer le crochet de 0,85 mm. Le nouveau calcul vise donc environ sept fois cet effort. Ce rapport est analytique : ce n’est ni une mesure des impressions ni une multiplication démontrée de la résistance du verrou.

Le calcul ne représente pas la souplesse réelle de l’ancrage, les concentrations locales, l’anisotropie des couches, le frottement, la force de retenue en traction, le fluage ou la fatigue. Il ne remplace pas une simulation mécanique complète ni un essai. Une butée plate limite la pression et permet le début du retrait sans heurter le volume réservé à l’objet.

## Vérifications et essai physique

Les tests contrôlent les six permutations, l’espace libre de l’objet, les volumes fermés et orientés, le verrouillage au repos, le dégagement après pression et le retrait initial à plusieurs jeux et tailles. Les fichiers STL/3MF sont relus après sérialisation pour contrôler leurs triangles et arêtes ; la géométrie finale est reconstruite sur une grille de 0,0001 mm avant triangulation. L’aperçu et les exports utilisent les mêmes maillages.

Cette révision reste à imprimer. Le retour utilisateur sur l’impression précédente a motivé la reprise de la languette ; il ne valide pas la nouvelle version.

1. Exporter une petite boîte, par exemple pour 30 × 40 × 10 mm, sans calage et avec les jeux par défaut. Garder son profil habituel de PLA pour que l’essai soit représentatif.
2. Contrôler dans le slicer la continuité des parois, les fentes, les rails et le pont arrière. Boîte fond au plateau ; couvercle face lisse au plateau, nervures vers le haut.
3. Fermer sans forcer. Vérifier clic, retenue et ouverture ; si possible mesurer l’effort au centre de la zone striée. Ajuster le jeu si nécessaire.
4. Répéter les ouvertures ; consigner fissures, blanchiment, jeu, effort et nombre de cycles, ainsi que filament et paramètres d’impression.
5. Tester le format final avec son contenu et comparer la masse au calcul. Une protection supplémentaire ou des parois plus épaisses peuvent être nécessaires selon l’objet.
6. Sceller l’envoi et peser le colis fermé avant affranchissement.
