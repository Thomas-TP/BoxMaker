import { Children, cloneElement, isValidElement, type ReactNode } from "react";

export type Language = "fr" | "en";
const stored =
  typeof localStorage === "undefined"
    ? null
    : localStorage.getItem("boxmaker-language");
let language: Language =
  stored === "fr" || stored === "en"
    ? stored
    : navigator.language.toLowerCase().startsWith("fr")
      ? "fr"
      : "en";
if (typeof document !== "undefined") document.documentElement.lang = language;

export function getLanguage(): Language {
  return language;
}
export function setLanguage(value: Language) {
  language = value;
  localStorage.setItem("boxmaker-language", value);
  document.documentElement.lang = value;
}

const english: Record<string, string> = {
  Inconnu: "Unknown",
  "Atelier de création": "Design studio",
  Exporter: "Export",
  "Mises à jour": "Updates",
  Ouvrir: "Open",
  Enregistrer: "Save",
  "Guide d’utilisation": "User guide",
  "Réinitialiser les paramètres": "Reset settings",
  "VOTRE ATELIER D’EMBALLAGE": "YOUR PACKAGING STUDIO",
  "Votre boîte, sur mesure.": "Your custom-fit box.",
  "Vos dimensions. Trois pièces dans un seul export 3MF.":
    "Your dimensions. Three parts in one 3MF export.",
  "Expédition en Suisse": "Shipping in Switzerland",
  Configuration: "Configuration",
  "L’objet à protéger": "The object to protect",
  "Dimension A": "Dimension A",
  "Dimension B": "Dimension B",
  "Dimension C": "Dimension C",
  Largeur: "Width",
  Longueur: "Length",
  Hauteur: "Height",
  "Poids de l’objet": "Object weight",
  "Calage par face": "Padding per side",
  "Changer l’orientation de l’objet": "Rotate object orientation",
  "Orientation automatique": "Automatic orientation",
  "Calcul…": "Calculating…",
  "· largeur × longueur × hauteur": "· width × length × height",
  "Les dimensions peuvent être saisies dans n’importe quel ordre.":
    "Enter the dimensions in any order.",
  "Jeu autour de l’objet": "Clearance around the object",
  "Espace libre par face, en plus du calage.":
    "Free space on each side, in addition to padding.",
  "Votre imprimante": "Your printer",
  "Matériau du projet": "Project material",
  "La boîte": "The box",
  "2 pièces": "2 parts",
  "3 pièces": "3 parts",
  "Épaisseur parois": "Wall thickness",
  "Épaisseur fond": "Base thickness",
  "Couvercle coulissant": "Sliding lid",
  "Scellé à rompre, puis appuyer et tirer":
    "Break the seal, then press and pull",
  "Appuyer, puis tirer · verrou intégré": "Press, then pull · integrated latch",
  "Ancien modèle · clavette séparée": "Older model · separate locking key",
  "Scellé imprimé pour l’expédition": "Printable shipping seal",
  "Affleurant dans le coin arrière · pose après fermeture":
    "Flush with the rear corner · insert after closing",
  "Réglages avancés": "Advanced settings",
  "Modèle de fermeture": "Closure model",
  "Coulissant à pression · nervuré": "Press-to-slide · ribbed",
  "Ancienne boîte · clavette": "Older box · locking key",
  "Jeu par côté": "Clearance per side",
  "Marge plateau": "Bed margin",
  "Poids calage": "Padding weight",
  Filament: "Filament",
  "Charger une petite boîte d’essai": "Load a small test box",
  "Vos fichiers restent sur votre ordinateur.":
    "Your files stay on your computer.",
  "Calcul en cours…": "Calculating…",
  "Paramètres à corriger": "Check the settings",
  "Aperçu de votre boîte": "Preview your box",
  "APERÇU 3D": "3D PREVIEW",
  Fermée: "Closed",
  "Vue éclatée": "Exploded view",
  "Pièces à plat": "Print layout",
  Ouverture: "Opening",
  "Construction de votre boîte…": "Building your box…",
  "Afficher l’objet": "Show the object",
  "Recentrer la vue": "Recenter the view",
  "Glisser pour tourner · Molette pour zoomer":
    "Drag to rotate · Scroll to zoom",
  "1 · Appuyer sur la languette": "1 · Press the tab",
  "2 · Tirer le couvercle": "2 · Pull the lid",
  "3 · Relâcher et faire glisser": "3 · Release and slide",
  "Démonstration du mouvement": "Opening demonstration",
  "Les rails retiennent le couvercle ; la languette bloque uniquement le glissement. Mouvement illustratif, à confirmer par impression.":
    "The rails hold the lid; the tab only stops it from sliding. This motion is illustrative and needs print testing.",
  "DIMENSIONS EXTÉRIEURES": "OUTER DIMENSIONS",
  "PLA ESTIMÉ": "ESTIMATED PLA",
  "COÛT MATIÈRE": "MATERIAL COST",
  "Espace autour de l’objet": "Space around the object",
  "Jeu total :": "Total clearance:",
  "mm de calage et": "mm of padding and",
  "mm de jeu par face.": "mm of clearance per side.",
  "mm à l’arrière et": "mm at the rear and",
  "mm au-dessus pour le mouvement.": "mm above for movement.",
  "mm · largeur": "mm · width",
  "mm · épaisseur": "mm · thickness",
  "(largeur × longueur × hauteur), dont":
    "(width × length × height), including",
  "de calage et": "of padding and",
  "de jeu par face.": "of clearance per side.",
  "Minimum nécessaire au mécanisme :":
    "Minimum space required by the mechanism:",
  "L’objet est centré dans cet espace.":
    "The object is centered in this space.",
  "Hors cavité utile : zone du verrou de":
    "Outside the usable cavity: a latch area of",
  "à l’arrière et": "at the rear and",
  "au-dessus pour le mouvement.": "above for movement.",
  "Pression calculée : ≈": "Estimated press force: ≈",
  "N · languette adaptée": "N · adapted tab",
  "Plage indicative selon la rigidité du PLA :":
    "Indicative range for PLA stiffness:",
  "N. À vérifier sur une impression d’essai.": "N. Check with a test print.",
  "· largeur": "· width",
  "· épaisseur": "· thickness",
  "Déformation nominale calculée :": "Calculated nominal strain:",
  "% ; sur butée :": "%; at the stop:",
  "%. La durée de vie en cycles n’est pas prédite.":
    "%. Cycle life has not been predicted.",
  Boîte: "Box",
  Couvercle: "Lid",
  Clavette: "Locking key",
  "Boîte nervurée": "Ribbed box",
  "Couvercle à pression": "Press-fit lid",
  "Scellé imprimé · usage unique": "Printable seal · single use",
  "Pièces présentées côte à côte, comme dans l’export 3MF complet. Réorganisez-les dans le slicer ou répartissez-les sur plusieurs plateaux si nécessaire.":
    "Parts are placed side by side, as in the complete 3MF export. Rearrange them in your slicer or use multiple build plates if needed.",
  "Exporter votre boîte": "Export your box",
  "Chaque pièce tient sur le plateau utile":
    "Every part fits on the usable bed",
  "Vérifiez les dimensions des pièces": "Check the part dimensions",
  Pièce: "Part",
  "Toutes les pièces (3)": "All parts (3)",
  "Boîte + couvercle + scellé": "Box + lid + seal",
  "Boîte + couvercle": "Box + lid",
  "Scellé à réimprimer": "Replacement seal",
  Format: "Format",
  "les 3 pièces": "all 3 parts",
  "les 2 pièces": "both parts",
  "la boîte": "the box",
  "le couvercle": "the lid",
  "le scellé": "the seal",
  "la clavette": "the locking key",
  "Un fichier 3MF · pièces séparées, posées à plat.":
    "One 3MF file · separate parts laid flat.",
  "Réorganisez-les ou répartissez-les sur plusieurs plateaux dans le slicer selon la place disponible.":
    "Rearrange them in your slicer or place them on separate build plates as needed.",
  "Géométrie en mm · une pièce par fichier":
    "Geometry in mm · one part per file",
  "Réglages d’impression à choisir dans le slicer.":
    "Choose print settings in your slicer.",
  "Votre expédition": "Your shipment",
  Destination: "Destination",
  Suisse: "Switzerland",
  "Étiquette colis en ligne": "Online parcel label",
  "Rabais de CHF 1.50 sur les colis admissibles":
    "CHF 1.50 discount on eligible parcels",
  "Poids total": "Total weight",
  estimé: "estimated",
  mesuré: "measured",
  "À renseigner": "Enter a weight",
  "LE MOINS CHER": "LOWEST PRICE",
  "/ envoi": "/ shipment",
  "Avec suivi": "With tracking",
  "Sans suivi": "No tracking",
  "Tarifs à actualiser": "Rates need updating",
  "Ajoutez le poids": "Add the weight",
  "Aucun tarif disponible": "No rate available",
  "Cette grille est limitée à 2026. Vérifiez les prix sur poste.ch.":
    "This rate table covers 2026. Check current prices on post.ch.",
  "Le poids total et le format fermé déterminent les services admissibles.":
    "The total weight and closed box size determine eligible services.",
  "AUTRES POSSIBILITÉS": "OTHER OPTIONS",
  suivi: "tracking",
  "sans suivi": "no tracking",
  "J’ai pesé mon envoi fermé": "I weighed my closed parcel",
  "Poids total réel, boîte et calage inclus":
    "Actual total weight, including box and padding",
  "Tarifs publics 2026 · vérifiés le 07.09.2026":
    "Public 2026 rates · checked 7 September 2026",
  "Sous réserve du poids réel et d’un emballage conforme. Options et traitement manuel non inclus. A Plus et recommandé : prêts à l’envoi.":
    "Subject to actual weight and suitable packaging. Options and manual handling are excluded. A Plus and registered mail must be ready to ship.",
  "Consulter les tarifs de la Poste": "View Swiss Post rates",
  "Contrôles et conseils d’impression": "Print checks and advice",
  "Recommandations d’emballage de la Poste": "Swiss Post packaging advice",
  "Conçu à la bonne taille.": "Designed to fit.",
  "Guide & formats postaux": "Guide & postal sizes",
  "Fermer la notification": "Dismiss notification",
  "MODE D’EMPLOI": "HOW IT WORKS",
  "De votre objet à sa boîte.": "From your object to its box.",
  "Mesurez l’objet et renseignez son poids. Le calage s’ajoute sur les six faces, en plus du jeu d’insertion. Les trois dimensions peuvent être saisies dans n’importe quel ordre pour le nouveau modèle.":
    "Measure your object and enter its weight. Padding is added on all six sides, beyond the insertion clearance. For the new model, enter the three dimensions in any order.",
  "Choisissez la P1S ou la K2 classique. La marge du plateau est réglable.":
    "Choose the P1S or standard K2. You can adjust the build-bed margin.",
  "Exportez toutes les pièces dans un seul 3MF, ou choisissez une pièce seule en 3MF ou STL. L’ancien modèle inclut aussi sa clavette dans l’export complet.":
    "Export all parts in one 3MF, or export a single part as 3MF or STL. The older model includes its locking key in the complete export.",
  "Dans le slicer, vérifiez la géométrie, les zones exclues, l’adhérence et les petits surplombs des rails. Imprimez d’abord la petite boîte d’essai des réglages avancés.":
    "In your slicer, check the geometry, excluded areas, adhesion, and small rail overhangs. Print the small test box from Advanced settings first.",
  "Testez le clic et le déverrouillage sans forcer, protégez l’objet, puis insérez le scellé imprimé dans le coin arrière si cette option est active. Pesez l’envoi fermé. Le destinataire rompt sa tête avant d’appuyer et de faire glisser le couvercle.":
    "Test the latch and release gently, protect the object, then insert the printable seal in the rear corner if enabled. Weigh the closed parcel. The recipient breaks the seal head before pressing and sliding the lid.",
  "Formats extérieurs · Suisse": "Outer sizes · Switzerland",
  Service: "Service",
  Limite: "Limit",
  "B5 épaisse": "Thick B5",
  "Jusqu’à 50 mm · + CHF 2 en A/B/A Plus":
    "Up to 50 mm · + CHF 2 for A/B/A Plus",
  "Colis standard": "Standard parcel",
  Encombrant: "Bulky parcel",
  "Longueur ≤ 2000 mm / 30 kg ; ≤ 2500 mm / 10 kg. L + 2l + 2h ≤ 4000 mm.":
    "Length ≤ 2000 mm / 30 kg; ≤ 2500 mm / 10 kg. L + 2w + 2h ≤ 4000 mm.",
  "La boîte est un prototype, pas un emballage homologué. Les dimensions intérieures annoncées correspondent à l’espace libre sous le couvercle, entre les renforts.":
    "The box is a prototype, not certified packaging. The stated internal dimensions describe the free space beneath the lid, between reinforcements.",
  "Tarifs lettres": "Letter rates",
  "Limites encombrants": "Bulky parcel limits",
  "Créer ma boîte": "Create my box",
  "TOUJOURS À JOUR": "STAY UP TO DATE",
  "Les nouveautés de Boxmaker, quand vous le décidez.":
    "Get Boxmaker updates when you choose.",
  "Version installée · préversion": "Installed version · preview",
  Ordinateur: "Computer",
  "Mises à jour via Microsoft Store": "Updates via Microsoft Store",
  "Versions publiées par Thomas Prud'homme":
    "Versions published by Thomas Prud'homme",
  "Notes de version": "Release notes",
  "Opération en cours…": "Working…",
  "Vérifier les mises à jour": "Check for updates",
  "Notes de cette version": "Notes for this version",
  "Ouvrir la bibliothèque Microsoft Store": "Open the Microsoft Store library",
  "La mise à jour est prête. L’application va redémarrer ; les modifications non enregistrées seront perdues.":
    "The update is ready. The app will restart and unsaved changes will be lost.",
  "Installer et redémarrer": "Install and restart",
  "Télécharger la mise à jour": "Download update",
  "Enregistrer le projet puis installer": "Save the project, then install",
  "Fermer la fenêtre": "Close window",
  "Aperçu 3D interactif de la boîte": "Interactive 3D box preview",
  "L’aperçu nécessite WebGL. Les calculs et exports restent disponibles.":
    "The preview requires WebGL. Calculations and exports remain available.",
  "Les mises à jour intégrées sont disponibles dans l’application Windows installée.":
    "Built-in updates are available in the installed Windows app.",
  "Fichier exporté en millimètres. Ouvrez-le dans votre slicer.":
    "File exported in millimeters. Open it in your slicer.",
  "Export annulé.": "Export canceled.",
  "Projet enregistré.": "Project saved.",
  "Enregistrement annulé.": "Save canceled.",
  "Fichier trop volumineux": "File is too large",
  "Projet Boxmaker incompatible": "Incompatible Boxmaker project",
  "Projet adapté à la nouvelle fermeture et orienté automatiquement. Repesez l’envoi.":
    "Project updated for the new closure and automatically oriented. Weigh the parcel again.",
  "Scellé compact actualisé. Repesez l’envoi avant expédition.":
    "Compact seal updated. Weigh the parcel again before shipping.",
  "Projet chargé.": "Project opened.",
  "Le moteur ne répond pas.": "The engine is not responding.",
  "Sur macOS, téléchargez les nouvelles versions depuis les releases GitHub. Cette version ne dispose pas de mise à jour intégrée.":
    "On macOS, download new versions from GitHub Releases. This build has no built-in updates.",
  "Installez Boxmaker avec l’installateur Velopack pour utiliser les mises à jour intégrées.":
    "Install Boxmaker with the Velopack installer to use built-in updates.",
  "Cette installation reçoit ses mises à jour dans la bibliothèque du Microsoft Store.":
    "This installation receives updates through the Microsoft Store library.",
  "Une nouvelle version est disponible.": "A new version is available.",
  "Vous utilisez la dernière version disponible.":
    "You have the latest available version.",
  "Aucune version téléchargeable n’est encore publiée.":
    "No downloadable version has been published yet.",
  "Une version plus ancienne ne sera pas installée.":
    "An older version will not be installed.",
  "Mettez Boxmaker à jour depuis le Microsoft Store.":
    "Update Boxmaker through Microsoft Store.",
  "Vérifiez d’abord les mises à jour.": "Check for updates first.",
  "Téléchargez la mise à jour avant de redémarrer.":
    "Download the update before restarting.",
  "Aucune mise à jour en attente.": "No update is pending.",
  "Téléchargez les nouvelles versions depuis les releases GitHub.":
    "Download new versions from GitHub Releases.",
  "Fichier non valide": "Invalid file",
  "Courrier B": "B Mail",
  "Courrier A": "A Mail",
  "Courrier A Plus": "A Mail Plus",
  Recommandé: "Registered mail",
  "Lettre standard B5": "Standard B5 letter",
  "Midilettre B5": "B5 mid-size letter",
  "Grande lettre B4": "Large B4 letter",
  "B5 · prêt à l’envoi": "B5 · ready to ship",
  "B4 · prêt à l’envoi": "B4 · ready to ship",
  "Contre signature": "Signature required",
  "Jusqu’à 3 jours ouvrables": "Up to 3 working days",
  "Jour ouvrable suivant": "Next working day",
  "2 jours ouvrables": "2 working days",
  "Avant 9 h le jour suivant": "Before 9 a.m. next day",
  "Colis ≤ 2 kg": "Parcel ≤ 2 kg",
  "Colis ≤ 10 kg": "Parcel ≤ 10 kg",
  "Colis ≤ 30 kg": "Parcel ≤ 30 kg",
  "Une pièce dépasse le volume utile du plateau. Réduisez l’objet ou le calage avant d’exporter.":
    "A part exceeds the usable build volume. Reduce the object size or padding before exporting.",
  "Calage inférieur aux 30 mm environ recommandés par la Poste sur chaque face : adaptez la protection à la fragilité de l’objet.":
    "Padding is below the roughly 30 mm per side recommended by Swiss Post. Adjust protection to the object's fragility.",
  "Renseignez un poids pour obtenir un tarif. Les dimensions seules ne déterminent pas le prix.":
    "Enter a weight to see shipping rates. Dimensions alone do not determine the price.",
  "Hors limites des services postaux usuels intégrés. Transport à organiser avec la Poste.":
    "Outside the limits of the included postal services. Arrange transport with Swiss Post.",
  "Petit colis : la Poste recommande au moins 148 × 105 × 10 mm. Une lettre peut rester possible selon son format.":
    "Small parcel: Swiss Post recommends at least 148 × 105 × 10 mm. Letter mail may still be possible depending on its size.",
  "Poids estimé à partir du volume de PLA (1,24 g/cm³). Remplacez-le par le poids de l’envoi fermé avant affranchissement.":
    "Weight is estimated from PLA volume (1.24 g/cm³). Use the closed parcel's measured weight before buying postage.",
  "P1S : contrôlez aussi les zones exclues et la ligne de purge du profil Bambu Studio ; la marge rectangulaire ne les modélise pas.":
    "P1S: also check the excluded zones and purge line in Bambu Studio; the rectangular margin does not model them.",
  "Scellé imprimé expérimental : insérez le petit verrou vertical dans le coin arrière après avoir chargé et fermé la boîte. Rompez sa tête avant d’appuyer puis de faire glisser le couvercle. Vérifiez l’enclenchement, la rupture et la tenue sur une impression réelle avant tout envoi.":
    "Experimental printable seal: insert the small vertical latch in the rear corner after filling and closing the box. Break its head before pressing and sliding the lid. Test engagement, breakage and retention on a real print before shipping.",
  "Le scellé gêne l’ouverture discrète par la fermeture normale ; il ne garantit pas l’inviolabilité et peut être remplacé ou contourné en endommageant la boîte.":
    "The seal makes discreet opening through the normal closure harder; it does not guarantee tamper resistance and can be replaced or bypassed by damaging the box.",
  "Fermeture à pression : imprimez d’abord l’essai, vérifiez le clic et l’ouverture sans forcer. La durée de vie du ressort PLA et la résistance au transport restent à tester ; scellez l’envoi avec un adhésif.":
    "Press latch: print a test first and check the snap and opening without force. PLA spring life and shipping resistance still need testing; secure the parcel with tape.",
  "Boîte : fond posé au plateau. Couvercle : face lisse dessous, nervures et bouton dessus. Contrôlez le petit pont du verrou et les lèvres des rails dans le slicer.":
    "Box: place the base on the bed. Lid: smooth face down, ribs and button up. Check the latch bridge and rail lips in your slicer.",
  "Petit objet : la cavité est agrandie uniquement selon l’espace nécessaire à la languette calculée. Le supplément est détaillé sous l’aperçu.":
    "Small object: the cavity grows only as much as the calculated tab needs. The extra space appears beneath the preview.",
  "Effort de pression estimé avec un modèle de poutre et une plage de rigidité du PLA. Filament, couches, température et flexion de l’ancrage peuvent modifier le résultat ; validez sur un essai imprimé.":
    "Press force is estimated from a beam model and a PLA stiffness range. Filament, layer direction, temperature and anchor flex may change it; validate with a test print.",
  "Ancien modèle à clavette : prototype PLA à tester. Sécurisez la fermeture avec de l’adhésif.":
    "Older locking-key model: test the PLA prototype and secure the closure with tape.",
  "Modèle de boîte inconnu": "Unknown box model",
  "Le scellé imprimé nécessite le modèle coulissant à pression.":
    "The printable seal requires the press-to-slide model.",
  "Imprimante inconnue": "Unknown printer",
  "Pièce manquante": "Missing part",
  "Format manquant": "Missing format",
  "L’export de toutes les pièces utilise le format 3MF.":
    "Exporting all parts requires 3MF format.",
  "Une pièce dépasse le plateau sélectionné. Export bloqué.":
    "A part exceeds the selected build plate. Export blocked.",
  "Pièce inconnue": "Unknown part",
  "Cette pièce dépasse le plateau sélectionné. Export bloqué.":
    "This part exceeds the selected build plate. Export blocked.",
  "Format inconnu": "Unknown format",
  "Action inconnue": "Unknown action",
  "Dimension de l’objet (mm)": "Object dimension (mm)",
  "Calage par face (mm)": "Padding per side (mm)",
  "Jeu autour de l’objet (mm)": "Object clearance (mm)",
  "Jeu mécanique (mm)": "Mechanical clearance (mm)",
  "Marge du plateau (mm)": "Build-bed margin (mm)",
  "Poids du calage (g)": "Padding weight (g)",
  "Prix du filament": "Filament price",
  "Poids de l’objet (g)": "Object weight (g)",
  "Poids total mesuré (g)": "Measured total weight (g)",
};

const normalize = (text: string) => text.replace(/\s+/g, " ").trim();
export function tr(text: string): string {
  if (language === "fr") return text;
  const key = normalize(text);
  let translated = english[key];
  if (!translated) {
    const range = key.match(/^(.+) doit être entre (.+) et (.+)\.$/);
    if (range)
      translated = `${english[range[1]] ?? range[1]} must be between ${range[2]} and ${range[3]}.`;
    else {
      for (const [prefix, replacement] of Object.entries({
        "Error:": "Error:",
        "Export impossible :": "Export failed:",
        "Enregistrement impossible :": "Save failed:",
        "Ouverture impossible :": "Could not open project:",
        "Vérification impossible :": "Could not check for updates:",
        "Téléchargement impossible :": "Download failed:",
        "Installation impossible :": "Installation failed:",
        "Paramètres invalides :": "Invalid parameters:",
      }))
        if (key.startsWith(prefix)) {
          translated = `${replacement} ${tr(key.slice(prefix.length).trim())}`;
          break;
        }
    }
  }
  if (!translated) return text;
  return (
    (text.match(/^\s*/)?.[0] ?? "") +
    translated +
    (text.match(/\s*$/)?.[0] ?? "")
  );
}

const textProps = ["aria-label", "placeholder", "title", "label", "alt"];
export function localizeTree<T extends ReactNode>(node: T): T {
  if (typeof node === "string") return tr(node) as T;
  if (Array.isArray(node)) return node.map(localizeTree) as unknown as T;
  if (!isValidElement(node)) return node;
  const props = node.props as Record<string, unknown>;
  const changes: Record<string, unknown> = {};
  for (const key of textProps)
    if (typeof props[key] === "string") changes[key] = tr(props[key]);
  if ("children" in props)
    changes.children = Children.map(props.children as ReactNode, localizeTree);
  return cloneElement(node, changes) as T;
}
