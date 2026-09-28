# Guide d’utilisation de Boxmaker

Pour installer Boxmaker, utilisez le badge Microsoft Store dans le [README](../README.md).

1. Saisir les trois dimensions en **mm**, dans n’importe quel ordre : le modèle coulissant choisit automatiquement une orientation imprimable et basse. Renseigner le poids en **g**. Le jeu d’insertion vaut 0,3 mm par face ; ajouter séparément le calage nécessaire.
2. Choisir **Bambu Lab P1S (256³ mm)** ou **Creality K2 classique (260³ mm)**.
3. Ajuster parois, fond et jeu dans les réglages avancés. Le profil est PLA ; le prix au kilo est modifiable.
4. Inspecter les vues fermée, éclatée, pièces à plat et ouverture. Le curseur montre la pression sur la languette arrière puis le retrait du couvercle.
5. Cliquer sur **Exporter les 3 pièces** pour obtenir **un seul 3MF contenant la boîte, le couvercle et le scellé**, sous forme d’objets indépendants posés à plat. Réorganiser les pièces dans le slicer ou utiliser plusieurs plateaux si nécessaire. Les exports individuels STL et 3MF restent disponibles via le menu « Pièce », notamment pour réimprimer le scellé. Le 3MF contient une géométrie en millimètres, sans profil machine ou G-code. Imprimer le couvercle face lisse dessous, nervures dessus, et le scellé à plat. L’export complet de l’ancien modèle contient trois pièces, dont la clavette posée sur sa tête.
6. Dans Bambu Studio ou Creality Print, vérifier l’orientation, les surplombs, les zones exclues et le poids calculé. Imprimer une petite boîte d’essai avant un emballage complet.
7. Emballer et fermer la boîte, puis insérer verticalement le petit verrou imprimé dans le coin arrière droit jusqu’à l’enclenchement. Le destinataire soulève et rompt sa tête, puis appuie et fait glisser le couvercle. Le reste du verrou demeure dans la coque et peut être retiré après ouverture. Peser l’envoi fermé et saisir ce poids réel dans « J’ai pesé mon envoi fermé ». Faire un essai physique avant tout envoi.

Les projets peuvent être enregistrés en `.boxmaker.json` puis rouverts. Un projet coulissant v2 est adapté au mécanisme v0.3, conserve son calage et doit être repesé ; un projet à clavette conserve sa géométrie. Les projets antérieurs à la v4 restent sans scellé jusqu’à activation explicite. Un ancien projet avec scellé passe au nouveau verrou compact et perd sa pesée enregistrée ; il faut peser l’envoi à nouveau. Modifier la géométrie ou le contenu efface également le poids mesuré.

## Interface

L’aperçu 3D occupe le centre de l’atelier. Les paramètres sont à gauche, l’export et l’expédition à droite ; les panneaux défilent indépendamment sur les grandes fenêtres. Les détails de la cavité et du ressort se déplient sous l’aperçu. Le choix de l’ancien modèle se trouve dans les réglages avancés. Un raccourci d’export apparaît dans la barre d’outils sur les fenêtres plus étroites.
