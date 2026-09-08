# Fermeture coulissante à pression

La v0.2.0 comporte deux pièces : une coque nervurée fermée et un couvercle coulissant avec languette intégrée à l’arrière. Les rails retiennent le couvercle verticalement ; la dent de la languette bloque son glissement. Appuyer sur la zone striée, tirer de quelques millimètres, puis relâcher et retirer le couvercle.

## Choix et références

Le [guide Protolabs Network sur les encliquetages](https://www.hubs.com/knowledge-base/how-design-snap-fit-joints-3d-printing/) décrit les languettes en porte-à-faux et l’importance de limiter les concentrations de contraintes. La [fiche PLA de Prusa](https://help.prusa3d.com/article/pla_2062) indique que ce matériau est cassant et peu flexible. Ces références orientent le choix ; elles ne valident pas cette géométrie particulière.

| Solution examinée | Décision pour ce prototype PLA |
| --- | --- |
| Charnière souple monobloc | Écartée : impose des flexions répétées importantes. |
| Charnière avec axe | Possible, mais ajoute un assemblage et des volumes latéraux. |
| Emboîtement par friction | Simple, mais rétention dépendante du jeu et de l’usure. |
| Clips périphériques | Demande de déformer plusieurs zones pour ouvrir. |
| Rails + languette à faible course | Retenue : deux pièces et commande d’ouverture accessible. |

La languette a une longueur utile de 31 mm, une épaisseur de 1,2 mm et une course de démonstration de 0,85 mm. Elle revient au repos une fois verrouillée. Une butée limite l’enfoncement. Les extrémités de fentes sont arrondies. La cavité minimale de 30 × 40 mm préserve la longueur du ressort pour les petits objets.

## Matière calculée

Comparaison à cavité utile identique, avec 5 mm de calage par face et densité de 1,24 g/cm³. Référence v0.1 : parois 1,6 mm et fond 2 mm ; nouvelle configuration : parois 1,2 mm et fond 0,8 mm, nervures et couvercle 1,2 mm. Ce sont des volumes solides, pas des mesures issues d’un slicer ni une équivalence de résistance.

| Objet (mm) | Ancienne boîte | Nouvelle boîte | Réduction |
| --- | ---: | ---: | ---: |
| 100 × 70 × 30 | 83,30 g | 55,45 g | 33,4 % |
| 150 × 100 × 50 | 162,87 g | 109,28 g | 32,9 % |
| 220 × 180 × 100 | 411,60 g | 279,30 g | 32,1 % |

## Validation

Les tests automatisés vérifient : volumes uniques fermés et orientés, espace réservé à l’objet, absence d’intersection à l’état fermé, verrouillage au retrait, dégagement après pression et retrait libre. Plusieurs jeux et dimensions sont testés. La déformation illustrée est une approximation cinématique, pas une simulation mécanique de contraintes.

Essai physique à effectuer :

1. Commencer par un objet de 30 × 40 × 10 mm, sans calage, jeu de 0,3 mm. Exporter les deux pièces ; garder la buse et le profil PLA réellement utilisés pour les grandes boîtes.
2. Dans le slicer, vérifier que les peaux restent continues, que la languette n’est pas soudée aux côtés et que le petit pont arrière et les rails s’impriment correctement. Fond de boîte et face lisse du couvercle au plateau.
3. Fermer sans forcer. Vérifier le clic, le blocage au retrait et le déverrouillage par pression. Ajuster le jeu dans l’application si nécessaire.
4. Répéter des ouvertures en surveillant fissures, blanchiment, jeu et perte de rétention. Consigner filament, réglages et nombre de cycles.
5. Imprimer le format final, comparer sa masse au calcul et tester la protection avec un contenu représentatif. Les panneaux minces peuvent nécessiter une épaisseur supérieure pour un objet lourd ou fragile.
6. Sceller l’envoi avec de l’adhésif et peser l’ensemble fermé avant affranchissement.

Aucune impression ni certification de transport n’a été réalisée pendant le développement.
