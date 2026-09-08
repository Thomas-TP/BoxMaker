mod export;
mod mesh;
mod postal;
mod press_slide;

pub use mesh::Mesh;
pub use postal::{Quote, quotes};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Params {
    #[serde(default = "legacy_model")]
    pub model: String,
    pub object: [f64; 3],
    pub object_weight: Option<f64>,
    pub padding: f64,
    pub padding_weight: f64,
    pub wall: f64,
    pub floor: f64,
    pub clearance: f64,
    pub printer: String,
    pub plate_margin: f64,
    pub online: bool,
    pub filament_price: f64,
    pub measured_total: Option<f64>,
}

fn legacy_model() -> String {
    "legacy".into()
}

impl Default for Params {
    fn default() -> Self {
        Self {
            model: "press-slide".into(),
            object: [100., 70., 30.],
            object_weight: Some(80.),
            padding: 5.,
            padding_weight: 5.,
            wall: 1.2,
            floor: 0.8,
            clearance: 0.3,
            printer: "p1s".into(),
            plate_margin: 5.,
            online: true,
            filament_price: 25.,
            measured_total: None,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Part {
    pub id: String,
    pub name: String,
    pub mesh: Mesh,
    pub size: [f64; 3],
    pub assembled_offset: [f64; 3],
    pub volume: f64,
    pub fits: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Design {
    pub model: String,
    pub mechanism: Option<press_slide::Mechanism>,
    pub reference_plastic_weight: f64,
    pub outer: [f64; 3],
    pub inner: [f64; 3],
    pub object_offset: [f64; 3],
    pub parts: Vec<Part>,
    pub plastic_weight: f64,
    pub total_weight: Option<f64>,
    pub material_cost: f64,
    pub quotes: Vec<Quote>,
    pub warnings: Vec<String>,
    pub tariff_date: &'static str,
    pub tariff_valid_until: &'static str,
    pub printer_volume: [f64; 3],
}

fn range(value: f64, min: f64, max: f64, label: &str) -> Result<(), String> {
    if !value.is_finite() || value < min || value > max {
        return Err(format!("{label} doit être entre {min} et {max}."));
    }
    Ok(())
}

pub fn calculate(p: &Params) -> Result<Design, String> {
    for v in p.object {
        range(v, 10., 2500., "Dimension de l’objet (mm)")?;
    }
    range(p.padding, 0., 100., "Calage par face (mm)")?;
    range(p.wall, 1.2, 5., "Paroi (mm)")?;
    if p.model != "press-slide" && p.model != "legacy" {
        return Err("Modèle de boîte inconnu".into());
    }
    range(
        p.floor,
        if p.model == "legacy" { 1.2 } else { 0.8 },
        6.,
        "Fond (mm)",
    )?;
    range(p.clearance, 0.15, 0.6, "Jeu mécanique (mm)")?;
    range(p.plate_margin, 0., 20., "Marge du plateau (mm)")?;
    range(p.padding_weight, 0., 10000., "Poids du calage (g)")?;
    range(p.filament_price, 0., 200., "Prix du filament")?;
    if let Some(w) = p.object_weight {
        range(w, 0.1, 100000., "Poids de l’objet (g)")?;
    }
    if let Some(w) = p.measured_total {
        range(w, 0.1, 100000., "Poids total mesuré (g)")?;
    }
    let volume = match p.printer.as_str() {
        "p1s" => [256.; 3],
        "k2" => [260.; 3],
        _ => return Err("Imprimante inconnue".into()),
    };
    let (inner, outer, object_offset, raw_parts, mechanism) = if p.model == "press-slide" {
        let (inner, outer, offset, parts, mechanism) = press_slide::design(p)?;
        (inner, outer, offset, parts, Some(mechanism))
    } else {
        let (inner, outer, offset, parts) = mesh::design(p);
        (inner, outer, offset, parts, None)
    };
    // Compare to the released v0.1 reference at the SAME usable cavity size.
    // Its standard walls / floor were 1.6 / 2.0 mm; no slicer infill assumptions.
    let reference = Params {
        object: inner.map(|v| v - 2. * p.padding),
        wall: 1.6,
        floor: 2.,
        ..p.clone()
    };
    let reference_plastic_weight = mesh::design(&reference)
        .3
        .iter()
        .map(|p| p.2.volume())
        .sum::<f64>()
        / 1000.
        * 1.24;
    let parts: Vec<Part> = raw_parts
        .into_iter()
        .map(|(id, name, mut mesh, assembled_offset)| {
            let size = mesh.normalize();
            let fits = ((size[0] + p.plate_margin * 2. <= volume[0]
                && size[1] + p.plate_margin * 2. <= volume[1])
                || (size[1] + p.plate_margin * 2. <= volume[0]
                    && size[0] + p.plate_margin * 2. <= volume[1]))
                && size[2] <= volume[2];
            Part {
                id: id.into(),
                name: name.into(),
                volume: mesh.volume(),
                mesh,
                size,
                fits,
                assembled_offset,
            }
        })
        .collect();
    // Geometric solid volume; slicer settings and material density can change actual mass.
    let plastic_weight = parts.iter().map(|p| p.volume).sum::<f64>() / 1000. * 1.24;
    let total_weight = p.measured_total.or(p
        .object_weight
        .map(|w| w + p.padding_weight + plastic_weight));
    let quotes = if let Some(w) = total_weight {
        postal::quotes(outer, w, p.online)
    } else {
        vec![]
    };
    let mut warnings = vec![];
    if parts.iter().any(|p| !p.fits) {
        warnings.push("Une pièce dépasse le volume utile du plateau. Réduisez l’objet ou le calage avant d’exporter.".into());
    }
    if p.padding < 30. {
        warnings.push("Calage inférieur aux 30 mm environ recommandés par la Poste sur chaque face : adaptez la protection à la fragilité de l’objet.".into());
    }
    if total_weight.is_none() {
        warnings.push("Renseignez un poids pour obtenir un tarif. Les dimensions seules ne déterminent pas le prix.".into());
    }
    if total_weight.is_some() && quotes.is_empty() {
        warnings.push("Hors limites des services postaux usuels intégrés. Transport à organiser avec la Poste.".into());
    }
    let mut sorted = outer;
    sorted.sort_by(|a, b| b.total_cmp(a));
    if sorted[0] < 148. || sorted[1] < 105. || sorted[2] < 10. {
        warnings.push("Petit colis : la Poste recommande au moins 148 × 105 × 10 mm. Une lettre peut rester possible selon son format.".into());
    }
    if p.measured_total.is_none() {
        warnings.push("Poids estimé à partir du volume de PLA (1,24 g/cm³). Remplacez-le par le poids de l’envoi fermé avant affranchissement.".into());
    }
    if p.printer == "p1s" {
        warnings.push("P1S : contrôlez aussi les zones exclues et la ligne de purge du profil Bambu Studio ; la marge rectangulaire ne les modélise pas.".into());
    }
    if p.model == "press-slide" {
        warnings.push("Fermeture à pression : imprimez d’abord l’essai, vérifiez le clic et l’ouverture sans forcer. La durée de vie du ressort PLA et la résistance au transport restent à tester ; scellez l’envoi avec un adhésif.".into());
        warnings.push("Boîte : fond posé au plateau. Couvercle : face lisse dessous, nervures et bouton dessus. Contrôlez le petit pont du verrou et les lèvres des rails dans le slicer.".into());
        if inner[0] > p.object[0] + 2. * p.padding + 0.01
            || inner[1] > p.object[1] + 2. * p.padding + 0.01
        {
            warnings.push("Cavité portée à au moins 30 × 40 mm pour conserver la longueur du ressort, même avec un petit objet.".into());
        }
    } else {
        warnings.push("Ancien modèle à clavette : prototype PLA à tester. Sécurisez la fermeture avec de l’adhésif.".into());
    }
    Ok(Design {
        model: p.model.clone(),
        mechanism,
        reference_plastic_weight,
        outer,
        inner,
        object_offset,
        material_cost: plastic_weight / 1000. * p.filament_price,
        plastic_weight,
        total_weight,
        parts,
        quotes,
        warnings,
        tariff_date: "2026-09-07",
        tariff_valid_until: "2026-12-31",
        printer_volume: volume,
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    action: String,
    params: Params,
    part: Option<String>,
    format: Option<String>,
}

pub fn dispatch(input: &str) -> Result<String, String> {
    let request: Request =
        serde_json::from_str(input).map_err(|e| format!("Paramètres invalides : {e}"))?;
    let design = calculate(&request.params)?;
    match request.action.as_str() {
        "calculate" => serde_json::to_string(&design).map_err(|e| e.to_string()),
        "export" => {
            let part_id = request.part.as_deref().ok_or("Pièce manquante")?;
            let part = design
                .parts
                .iter()
                .find(|p| p.id == part_id)
                .ok_or("Pièce inconnue")?;
            if !part.fits {
                return Err("Cette pièce dépasse le plateau sélectionné. Export bloqué.".into());
            }
            let format = request.format.as_deref().ok_or("Format manquant")?;
            let bytes = match format {
                "stl" => export::stl(&part.mesh),
                "3mf" => export::three_mf(&part.mesh, &part.name)?,
                _ => return Err("Format inconnu".into()),
            };
            Ok(serde_json::json!({"bytes": bytes, "filename": format!("boxmaker-{}-PLA.{format}", part.id)}).to_string())
        }
        _ => Err("Action inconnue".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn refuses_invalid_inputs() {
        for invalid in [f64::NAN, f64::INFINITY, -1., 0., 5000.] {
            let p = Params {
                object: [invalid, 70., 30.],
                ..Default::default()
            };
            assert!(calculate(&p).is_err());
        }
    }
    #[test]
    fn measured_weight_overrides_estimate() {
        let p = Params {
            measured_total: Some(2000.),
            object_weight: None,
            ..Default::default()
        };
        let d = calculate(&p).unwrap();
        assert_eq!(d.total_weight, Some(2000.));
        assert_eq!(
            d.quotes
                .iter()
                .find(|q| q.service == "PostPac Economy")
                .unwrap()
                .cents,
            750
        );
    }
    #[test]
    fn no_weight_no_quotes() {
        let d = calculate(&Params {
            object_weight: None,
            ..Default::default()
        })
        .unwrap();
        assert!(d.quotes.is_empty());
    }
    #[test]
    fn oversized_design_cannot_export() {
        let p = Params {
            object: [260., 100., 30.],
            ..Default::default()
        };
        let request =
            serde_json::json!({"action":"export","params":p,"part":"body","format":"stl"});
        assert!(dispatch(&request.to_string()).is_err());
    }
}
