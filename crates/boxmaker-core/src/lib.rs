mod export;
mod mesh;
mod postal;
mod press_slide;
mod spring;

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
    #[serde(default = "default_object_clearance")]
    pub object_clearance: f64,
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
fn default_object_clearance() -> f64 {
    0.3
}

impl Default for Params {
    fn default() -> Self {
        Self {
            model: "press-slide".into(),
            object: [100., 70., 30.],
            object_weight: Some(80.),
            padding: 0.,
            object_clearance: default_object_clearance(),
            padding_weight: 0.,
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
    pub oriented_object: [f64; 3],
    pub object_space: [f64; 3],
    pub mechanism_expansion: [f64; 3],
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

fn orient(p: &Params, plate: [f64; 3]) -> [f64; 3] {
    let mut sorted = p.object;
    sorted.sort_by(f64::total_cmp);
    let permutations = [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ];
    let score = |object| {
        let candidate = Params {
            object,
            ..p.clone()
        };
        let outer = press_slide::layout(&candidate).outer;
        let ratios = [
            (outer[0] + 2. * p.plate_margin) / plate[0],
            (outer[1] + 2. * p.plate_margin) / plate[1],
            outer[2] / plate[2],
        ];
        let excess = ratios.into_iter().fold(1., f64::max) - 1.;
        // Fit first; lowest height next. Prefer sliding along the longest
        // in-plane dimension when both directions fit. Values, never input
        // indices, break ties so all six input orders give the same geometry.
        [
            excess,
            outer[2],
            if object[1] >= object[0] { 0. } else { 1. },
            outer.iter().product(),
            object[0],
            object[1],
        ]
    };
    permutations
        .map(|indices| indices.map(|i| sorted[i]))
        .into_iter()
        .min_by(|a, b| {
            let sa = score(*a);
            let sb = score(*b);
            sa.into_iter()
                .zip(sb)
                .map(|(x, y)| x.total_cmp(&y))
                .find(|o| !o.is_eq())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .unwrap()
}

pub fn calculate(p: &Params) -> Result<Design, String> {
    for v in p.object {
        range(v, 10., 2500., "Dimension de l’objet (mm)")?;
    }
    range(p.padding, 0., 100., "Calage par face (mm)")?;
    range(p.object_clearance, 0.1, 2., "Jeu autour de l’objet (mm)")?;
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
    let effective = Params {
        object: if p.model == "press-slide" {
            orient(p, volume)
        } else {
            p.object
        },
        ..p.clone()
    };
    let p = &effective;
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
        if inner[0] > p.object[0] + 2. * (p.padding + p.object_clearance) + 0.01
            || inner[1] > p.object[1] + 2. * (p.padding + p.object_clearance) + 0.01
        {
            warnings.push("Petit objet : la cavité est agrandie uniquement selon l’espace nécessaire à la languette calculée. Le supplément est détaillé sous l’aperçu.".into());
        }
        warnings.push("Effort de pression estimé avec un modèle de poutre et une plage de rigidité du PLA. Filament, couches, température et flexion de l’ancrage peuvent modifier le résultat ; validez sur un essai imprimé.".into());
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
        oriented_object: p.object,
        object_space: std::array::from_fn(|i| inner[i] - p.object[i]),
        mechanism_expansion: std::array::from_fn(|i| {
            (inner[i]
                - p.object[i]
                - 2. * (p.padding
                    + if p.model == "press-slide" {
                        p.object_clearance
                    } else {
                        0.
                    }))
            .max(0.)
        }),
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
    fn all_input_orders_generate_the_same_oriented_geometry() {
        for object in [
            [100., 70., 30.],
            [10., 15., 20.],
            [248., 20., 20.],
            [240., 70., 30.],
        ] {
            let expected = calculate(&Params {
                object,
                ..Default::default()
            })
            .unwrap();
            for indices in [
                [0, 1, 2],
                [0, 2, 1],
                [1, 0, 2],
                [1, 2, 0],
                [2, 0, 1],
                [2, 1, 0],
            ] {
                let actual = calculate(&Params {
                    object: indices.map(|i| object[i]),
                    ..Default::default()
                })
                .unwrap();
                assert_eq!(actual.oriented_object, expected.oriented_object);
                assert_eq!(actual.inner, expected.inner);
                assert_eq!(actual.outer, expected.outer);
                for (a, b) in actual.parts.iter().zip(&expected.parts) {
                    assert_eq!(a.mesh.vertices, b.mesh.vertices);
                    assert_eq!(a.mesh.triangles, b.mesh.triangles);
                }
            }
        }
    }
    #[test]
    fn orientation_uses_height_or_swaps_slide_axis_to_fit_printer() {
        let tall = calculate(&Params {
            object: [248., 20., 20.],
            ..Default::default()
        })
        .unwrap();
        assert_eq!(tall.oriented_object, [20., 20., 248.]);
        assert!(tall.parts.iter().all(|p| p.fits));
        let wide = calculate(&Params {
            object: [240., 70., 30.],
            ..Default::default()
        })
        .unwrap();
        assert_eq!(wide.oriented_object, [240., 70., 30.]);
        assert!(wide.parts.iter().all(|p| p.fits));
        let impossible = calculate(&Params {
            object: [260.; 3],
            ..Default::default()
        })
        .unwrap();
        assert!(impossible.parts.iter().any(|p| !p.fits));
    }
    #[test]
    fn snug_space_contains_only_explicit_clearance_without_hidden_minimum() {
        let d = calculate(&Params::default()).unwrap();
        assert_eq!(d.oriented_object, [70., 100., 30.]);
        for i in 0..3 {
            assert!((d.inner[i] - d.oriented_object[i] - 0.6).abs() < 1e-8);
            assert!(d.mechanism_expansion[i] < 1e-8);
        }
        let p = calculate(&Params {
            padding: 2.,
            ..Default::default()
        })
        .unwrap();
        for i in 0..3 {
            assert!((p.inner[i] - p.oriented_object[i] - 4.6).abs() < 1e-8);
        }
        let tiny = calculate(&Params {
            object: [10.; 3],
            ..Default::default()
        })
        .unwrap();
        assert!(tiny.inner[0] < 30. && tiny.inner[1] < 40.);
        assert!(tiny.mechanism_expansion.iter().any(|v| *v > 0.));
    }
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
