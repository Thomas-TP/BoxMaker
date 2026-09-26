# Scellé affleurant v0.7.0 — verrou arrière à tête sacrificielle

Le scellé précédent ajoutait une oreille sur le côté, 14 mm à l’arrière et une surépaisseur au-dessus du couvercle. Le nouveau verrou occupe un coin arrière qui ne fait pas partie de l’espace garanti à l’objet. Pour une boîte ordinaire, l’enveloppe extérieure est **identique à celle sans scellé**. Une très petite boîte peut être élargie juste assez pour séparer le verrou du ressort de fermeture. Aucune pièce achetée n’est nécessaire.

## Fonctionnement

La troisième pièce est une petite tige imprimée à plat, avec deux bras souples et une tête asymétrique reliée par deux attaches minces. Après chargement et fermeture, enfoncer la tige **verticalement** dans la fente du couvercle, au coin arrière droit, jusqu’à ce que les crochets s’enclenchent dans la coque. La tête chevauche la fente du couvercle : tant qu’elle est intacte, le couvercle ne peut pas coulisser normalement.

Pour ouvrir, soulever la grande aile de la tête avec l’ongle et la rompre. La tige reste retenue dans la coque ; la fente du couvercle est ouverte vers l’arrière, ce qui lui permet de coulisser autour de la tige sans arracher le logement. Le manque de tête laisse une trace visible. Une fois la boîte ouverte, pousser la tige restante vers l’intérieur du logement pour la retirer et réimprimer seulement cette petite pièce avant un nouvel envoi.

La coque porte un logement court relié à ses parois arrière et latérale, sous le couvercle. Le couvercle et le scellé n’entrent pas en contact avec ce logement hors des surfaces de retenue prévues. Le moteur vérifie que les trois volumes sont fermés, que le scellé ne coupe ni la coque ni le couvercle et qu’il reste entièrement dans l’enveloppe extérieure annoncée.

## Cotes CAO et impression

| Élément | Cote nominale |
| --- | ---: |
| Scellé imprimé à plat | 6,8 × 11 × 1,2 mm |
| Tête au-dessus du couvercle | 0,85 mm, sous le bord de la boîte |
| Jeu latéral des bras dans le passage | 0,2 mm par côté |
| Déflexion de chaque crochet à l’insertion | environ 0,2 mm |
| Attaches de rupture sous la tête | deux sections de 0,7 × 0,8 mm |

Ces valeurs sont des dimensions du modèle, pas des tolérances obtenues sur toutes les imprimantes. La tige doit rester à plat dans le slicer pour que les bras fléchissent dans le plan des couches. Contrôler que les deux attaches et les pointes des crochets sont réellement imprimées avec le profil PLA choisi ; les détails sont petits pour une buse de 0,4 mm. Ne pas ajouter de supports dans le logement fermé.

Pour l’objet de référence de 100 × 70 × 30 mm avec 5 mm de calage par face, l’enveloppe v0.6.0 était de 93,9 × 133,4 × 49,1566 mm. La v0.7.0 donne 85,4 × 119,4 × 46,5139 mm. La cavité reste 80,6 × 110,6 × 40,6 mm. Le gain concerne l’encombrement du scellé, pas la résistance démontrée.

## Essai physique indispensable

1. Imprimer une petite boîte et plusieurs scellés à plat. Vérifier dans le slicer la continuité des bras, des attaches et du passage du couvercle.
2. Fermer la boîte vide, puis enfoncer le scellé sans le faire basculer. Vérifier le clic et l’impossibilité de retirer la pièce intacte avec les doigts.
3. Soulever la tête avec l’ongle. Mesurer si possible l’effort nécessaire et vérifier que les deux attaches rompent sans éclats agressifs. Le couvercle doit ensuite coulisser sans forcer.
4. Sortir le fragment restant de la coque, répéter avec une nouvelle pièce et tester le format final chargé pendant le transport.

Les efforts de fermeture, de rupture et de retenue n’ont pas été mesurés sur une impression réelle. Le scellé est un témoin d’ouverture normale, pas une garantie d’inviolabilité : une boîte peut être endommagée, imitée ou contournée. Les indications générales sur l’orientation et les jeux FDM proviennent des guides [Formlabs sur les encliquetages](https://formlabs.com/blog/designing-3d-printed-snap-fit-enclosures/) et [Prusa sur la modélisation pour l’impression](https://help.prusa3d.com/article/modeling-with-3d-printing-in-mind_164135) ; elles ne valident pas les cotes de ce modèle.
