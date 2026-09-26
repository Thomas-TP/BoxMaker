# Scellé entièrement imprimé — double ancrage et languette de rupture

26 septembre 2026. Scellé intégré à Boxmaker v0.6.0, avec éprouvettes CAO antérieures. Contrainte : **aucune pièce achetée**, sans colle, vis ou aimant. La géométrie intégrée reste expérimentale et demande une impression d’essai.

## Solution retenue

Une petite agrafe imprimée à plat forme une seule pièce : deux ancrages à crochets et une languette centrale reliée à leurs têtes par trois paires de ponts de rupture. Un ancrage se verrouille dans la coque, l'autre dans le couvercle, après chargement et fermeture finale.

La liaison mécanique est : coque → ancrage A → ponts → languette → ponts → ancrage B → couvercle. Appuyer sur le ressort réutilisable ne libère pas cette liaison. Retirer le centre la coupe ; chaque moitié reste sur sa propre pièce et ne relie plus les deux pièces entre elles.

Les bras fléchissent dans le plan d'impression pendant l'enclenchement. Les ponts relient les ancrages ; soulever le centre hors de ce plan vise à provoquer leur rupture progressive. Cette différence de sollicitations est le principe recherché, pas une résistance mesurée. Les ponts peuvent aussi fléchir dans le plan ou céder simultanément : les impressions détermineront les cotes finales.

## Parcours prévu sur la boîte

1. Charger et fermer normalement. Une fermeture d'essai ne consomme pas le scellé.
2. Enfoncer l'agrafe en appuyant sur les deux têtes latérales. Pousser uniquement sur la languette solliciterait prématurément les ponts.
3. Le client soulève le centre, puis appuie et fait glisser le couvercle. Le centre manquant reste visible après refermeture.
4. Une fois ouvert, accéder aux faces de déverrouillage des restes et les retirer. Ces faces doivent être inaccessibles de l'extérieur lorsque la boîte est fermée.
5. Pour un nouvel envoi, réimprimer seulement l'agrafe.

Cela ajoute une troisième petite pièce imprimée. L’application exporte les trois pièces dans un même 3MF et propose un export individuel du scellé pour le réimprimer.

## Prototype fourni

Le générateur Rust `crates/boxmaker-core/examples/printed_seal.rs` crée les fichiers dans `output/printed-seal-v1/`. Le 3MF contient cinq objets indépendants : deux logements d'observation et trois variantes d'agrafe.

**Les logements ont des fenêtres latérales volontairement accessibles pour observer et libérer les crochets. Ce sont des éprouvettes de manipulation et de rupture, pas des logements sécurisés pour un colis.** Ils ne reproduisent pas les rails, la souplesse du couvercle ou les protections à intégrer à la boîte.

| Élément | Dimensions | Masse solide théorique à 1,24 g/cm³ |
| --- | --- | --- |
| Chaque logement | 20 × 21,3 × 4,6 mm | 1,960 g |
| Agrafe 1, un trait | 28,6 × 31,4 × 2 mm ; ponts 0,6 mm | 0,573 g |
| Agrafe 2, deux traits | Même enveloppe ; ponts 0,8 mm | 0,580 g |
| Agrafe 3, trois traits | Même enveloppe ; ponts 1,0 mm | 0,587 g |

Les traits identifient les variantes ; ce ne sont pas des identifiants uniques de sécurité. Les masses viennent des maillages, sans pesée réelle. Les bras ont une section de 1,2 × 1,6 mm ; le jeu de passage nominal est de 0,3 mm par face. Les ponts ont une largeur de 1 mm dans le plan. Ces dimensions sont des points de départ.

## Manipuler les éprouvettes

Préparation proposée : PLA, buse 0,4 mm, couches 0,2 mm, échelle 100 %, orientation à plat du 3MF. La buse réelle n'est pas connue : un autre diamètre nécessite de revoir les petits détails. Examiner les ponts de rupture et les courts plafonds des logements dans le slicer. Les ponts partent du plateau ; aucun assemblage imprimé en place n'est nécessaire. Ne pas remplir les passages de supports inaccessibles.

Commencer avec les deux logements et l'agrafe à deux traits. Garder son profil PLA habituel et examiner la continuité des ponts dans le slicer.

Placer les logements côte à côte, grandes faces planes dessous, entrées étroites vers soi, fronts alignés. Laisser 0,6 mm entre eux : les axes des entrées sont alors espacés de 20,6 mm. Face lisse du scellé dessous, traits dessus, enfiler une patte dans chaque entrée en maintenant les têtes latérales jusqu'à l'enclenchement. Le centre reste à l'extérieur.

Maintenir les logements et soulever progressivement la languette centrale entre les doigts. Éloigner le visage et surveiller les éclats : le PLA peut casser irrégulièrement. Après rupture, les logements doivent être séparables. Les fenêtres latérales permettent de repousser les crochets vers leur centre et d'extraire les restes. Si un bras blanchit ou si l'insertion force, ajuster le jeu plutôt que forcer.

Le générateur a été compilé et les exports produits avec ses contrôles de validité des solides. Aucun essai physique, aucune mesure de force, aucune qualification postale et aucune suite de tests n'ont été exécutés.

## Intégration à Boxmaker v0.6.0

L’option « Scellé imprimé pour l’expédition » ajoute deux logements à crochets : l’un sur le couvercle et l’autre sur une extension latérale de la coque. L’agrafe est imprimée à plat et insérée **après** le chargement et la fermeture. Son export individuel permet de remplacer seulement cette pièce à usage unique. Les anciens projets chargés restent sans scellé jusqu’à activation explicite ; l’option est active pour un nouveau projet.

La coque s’élargit de 8,5 mm pour le logement externe et s’allonge de 14 mm à l’arrière. Une petite cavité peut également être élargie afin de laisser assez de matière autour du ressort et du logement. Les dimensions extérieures, la masse et les tarifs sont recalculés avec ces ajouts. Les ponts de rupture intégrés ont 0,8 mm d’épaisseur et la fente nominale laisse 0,3 mm de jeu vertical par face. Ce sont des cotes CAO, pas des valeurs garanties après impression.

Les ouvertures de dégagement des fragments sont tournées vers l’intérieur de la boîte fermée. Après rupture et ouverture, les deux morceaux doivent pouvoir être extraits par ces ouvertures. Les fenêtres latérales ouvertes des éprouvettes ne sont pas reprises sur la boîte. L’état « Ouverture » dans l’aperçu représente la boîte **après** rupture et retrait du centre ; il masque donc l’agrafe intacte.

À contrôler physiquement avant un premier envoi :

- Enclenchement des deux crochets en poussant les têtes, sans rupture prématurée des ponts ; vérifier dans le slicer que les bras et ponts sont imprimés en continu.
- Résistance à la traction et à la flexion du couvercle fermé, notamment aux coins, et absence d’extraction discrète d’une agrafe intacte.
- Rupture accessible aux doigts du destinataire, visible après refermeture, sans blessure ni éclat dangereux.
- Retrait des deux fragments après ouverture et réinsertion d’un nouveau scellé ; transport réel avec la charge et le profil PLA retenus.

## Limites et choix

L'agrafe vise à imposer une rupture visible pour une ouverture normale. Elle n'empêche pas un vol laissant des dégâts et n'est pas impossible à copier. Un identifiant imprimé et une photo des ponts avant expédition pourraient aider à comparer l'état reçu ; un numéro ou un QR imprimé seul n'est pas une authentification.

Une bande intégrée au couvercle supprime la troisième pièce, mais consomme une partie du couvercle au premier scellement. Un collier imprimé ajoute une longue sangle, des petits crans et un geste de coupe. Une fermeture secrète ou une clé livrée sur la boîte ne distingue pas le client d'un tiers. L'agrafe remplaçable est le compromis retenu pour ce besoin.

## Sources et portée

- [Formlabs — encliquetages imprimés](https://formlabs.com/blog/designing-3d-printed-snap-fit-enclosures/) : bras fléchissant à l'insertion, retour libre dans un logement et orientation XY en FDM. Les indications SLA ne sont pas transposées au PLA.
- [Makers Making Change — Snap Fits](https://makersmakingchange.github.io/OpenAT_Design_Guide/Design_Elements_For_3D_Printed_Parts/Snap_Fits.html) : orientation, tolérances, raccordements et épaulements de retenue. Aucune cote publiée n'est prise comme une validation de notre agrafe.
- [Prusa — modélisation pour l'impression](https://help.prusa3d.com/article/modeling-with-3d-printing-in-mind_164135) : détails minimaux selon la buse et la largeur d'extrusion.
- [Unisto — point de rupture visible](https://www.unisto.com/fileadmin/security_seals/news/bearbeitet/The_true_story_about_securely_sealed_Tote_Boxes.pdf) : intérêt d'un témoin inspectable ; principe consulté sans achat de composant.

Les éprouvettes sont une proposition pour Boxmaker. Aucune nouveauté brevetable ni résistance à l'effraction n'est revendiquée.
