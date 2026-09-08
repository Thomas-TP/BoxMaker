# Poste suisse — grille intégrée

Vérifiée le **7 septembre 2026**. CHF TTC. Grille utilisable jusqu’au 31 décembre 2026 ; l’interface masque les prix à partir de 2027. Ceci n’est pas une synchronisation automatique avec la Poste.

Le moteur compare les dimensions **extérieures fermées** en millimètres après classement des axes ; le poids est celui de l’envoi complet. Les grammes fractionnaires sont arrondis vers le haut pour les seuils tarifaires. Sans poids, aucun prix n’est proposé.

## Lettres

| Format | Maximum mm | Poids | B | A |
|---|---|---|---|---|
| Standard B5 | 250 × 176 × 20 | 100 g | 1.00 | 1.20 |
| Midilettre B5 | 250 × 176 × 20 | 500 g | 1.40 | 1.70 |
| B5 épaisse | 250 × 176 × 50 | 500 g | prix de base + 2.00 | prix de base + 2.00 |
| Grande lettre B4 | 353 × 250 × 20 | 1000 g | 2.00 | 2.50 |

Le supplément B5 s’applique seulement au-delà de 20 mm. Le minimum de face utilisé est 140 × 90 mm. A Plus prêt à l’envoi : B5 2.90 (+2.00 si épaisse), B4 4.70. Recommandé prêt à l’envoi : 5.80, épaisseur B5 jusqu’à 50 mm incluse. Le moteur conserve le format admissible le moins cher pour chaque service. Les prix non prêts à l’envoi pour A Plus et Recommandé sont supérieurs de 1.00 et figurent dans le résultat moteur `counterCents` ; l’interface indique le tarif prêt à l’envoi.

## Colis et encombrants

Standard : 1000 × 600 × 600 mm, maximum 30 kg. Minimum **recommandé** : 148 × 105 × 10 mm, 100 g. Une taille plus petite n’entraîne pas un rejet automatique du tarif colis.

| Poids | Economy | Priority | Express Lune |
|---|---|---|---|
| ≤ 2 kg | 9.00 | 10.50 | 17.00 |
| > 2 à 10 kg | 12.00 | 13.50 | 23.00 |
| > 10 à 30 kg | 21.00 | 22.50 | 29.00 |
| Encombrant admissible | 31.00 | 32.50 | 38.00 |

Rabais de 1.50 pour les étiquettes colis créées dans les services en ligne admissibles de la Poste : Economy/Priority/encombrants, **pas Express Lune**.

Encombrants : longueur ≤ 2000 mm et ≤ 30 kg, ou longueur > 2000 à 2500 mm et ≤ 10 kg. Dans tous les cas, L + 2l + 2h ≤ 4000 mm. Au-delà, aucun tarif normal n’est retourné. Les surcoûts exceptionnels de transport hors limites ne sont pas proposés comme un service normal.

## Conditions non déduites des dimensions

La forme, le matériau, la fermeture, le calage, l’étiquette et les conditions de traitement restent à contrôler. Le moteur suppose une boîte rectangulaire correctement emballée. Les suppléments de traitement manuel (4 CHF pour colis), prestations Fragile, Signature, Assurance, marchandises dangereuses, samedi, retours et contrats commerciaux ne sont pas inclus. Les recommandations d’emballage ne sont pas une certification de cette boîte.

## Sources primaires

- [Lettres Suisse](https://www.post.ch/fr/expedier-des-lettres/lettres-suisse)
- [Colis Suisse](https://www.post.ch/fr/expedier-des-colis/colis-suisse)
- [Encombrants](https://www.post.ch/fr/expedier-des-colis/encombrant)
- [Emballage](https://www.post.ch/fr/expedier-des-colis/emballage-et-adressage/emballage-des-colis)
- [Dimensions lettres et colis](https://www.post.ch/-/media/post/pk/dokumente/das-angebot-im-ueberblick.pdf?hash=2587672E8A35E83B8C4C2F4183F25262&sc_lang=fr&vs=18)

Pour mettre à jour la grille : modifier `crates/boxmaker-core/src/postal.rs`, la date et la validité dans `lib.rs`, les mentions dans l’interface et ce document, puis exécuter les tests de seuils. Ne pas appliquer une grille 2027 avant sa date d’entrée en vigueur.
