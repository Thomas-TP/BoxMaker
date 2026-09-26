//! Experimental tear-out seal and two independent socket coupons.
//! This example does not modify Boxmaker's production geometry. The exposed
//! release windows are for observing the clips, not a secure shipping housing.
use boxmaker_core::{Mesh, Part};
use manifold_csg::{CrossSection, Manifold};
use std::{fs, path::Path};

// Reuse the production encoders without changing their public API.
#[path = "../src/export.rs"]
#[allow(dead_code)]
mod export;

const THICKNESS: f64 = 1.6;
const CENTERS: [f64; 2] = [10., 30.6];
const MID: f64 = 20.3;

fn block(x: f64, y: f64, z: f64, w: f64, d: f64, h: f64) -> Manifold {
    Manifold::cube(w, d, h, false).translate(x, y, z)
}

fn profile(points: Vec<[f64; 2]>, height: f64) -> Manifold {
    CrossSection::from_polygons(&[points]).extrude(height)
}

fn seal(bridge_height: f64, marks: usize) -> Manifold {
    let mut solids = vec![block(MID - 2.2, -5.8, 0., 4.4, 13.8, THICKNESS)];
    for center in CENTERS {
        solids.push(block(center - 4., 0., 0., 8., 8., THICKNESS));
        for side in [-1., 1.] {
            // The flexible arms lie in the print XY plane. The reverse hook
            // shoulder is square; the leading face compresses on insertion.
            let arm = profile(
                vec![
                    [1.3, 7.5],
                    [2.5, 7.5],
                    [2.5, 22.6],
                    [3.25, 22.6],
                    [2.5, 25.6],
                    [1.3, 25.6],
                ],
                THICKNESS,
            );
            let arm = if side < 0. {
                arm.mirror([1., 0., 0.])
            } else {
                arm
            };
            solids.push(arm.translate(center, 0., 0.));
            // Small root gusset, entirely outside the useful flexing length.
            let root = profile(vec![[0.8, 7.7], [2.5, 7.7], [1.3, 9.5]], THICKNESS);
            let root = if side < 0. {
                root.mirror([1., 0., 0.])
            } else {
                root
            };
            solids.push(root.translate(center, 0., 0.));
        }
    }
    // Three pairs of bridges. In-plane separation loads the complete set;
    // lifting the central rigid tab aims to break the pairs progressively.
    // These are coupon dimensions, not a calibrated breaking-force claim.
    for y in [1.0, 3.5, 6.0] {
        solids.push(block(13.8, y, 0., 4.5, 1.0, bridge_height));
        solids.push(block(22.3, y, 0., 4.5, 1.0, bridge_height));
    }
    // One, two or three tactile bars identify the 0.6 / 0.8 / 1.0 mm versions.
    for i in 0..marks {
        solids.push(block(MID - 1.6, -4.8 + i as f64 * 1.5, 1.5, 3.2, 0.65, 0.5));
    }
    Manifold::batch_union(&solids)
}

fn holder() -> Manifold {
    let body = block(0., 8.3, 0., 20., 21.3, 4.6);
    let cuts = vec![
        // 0.3 mm nominal side clearance around a 5.0 x 1.6 mm anchor.
        block(7.2, 8.2, 1.2, 5.6, 20., 2.2),
        // Recess behind the retention shoulder: hooks relax after seating.
        block(6.35, 22.2, 1.2, 7.3, 5.8, 2.2),
        // Deliberately exposed coupon windows for inspection and extraction.
        // The final box must conceal access to these surfaces while closed.
        block(-1., 23., 1.2, 8.5, 4.0, 2.2),
        block(12.5, 23., 1.2, 8.5, 4.0, 2.2),
    ];
    &body - &Manifold::batch_union(&cuts)
}

fn part(solid: Manifold, id: &str, name: &str) -> Result<Part, String> {
    let (mut vertices, stride, triangles) = solid.to_mesh_f64();
    for coordinate in &mut vertices {
        *coordinate = (*coordinate * 10_000.).round() / 10_000.;
    }
    let solid = Manifold::from_mesh_f64(&vertices, stride, &triangles)
        .map_err(|e| format!("{name}: {e}"))?
        .as_original()
        .simplify(0.0001);
    solid.status().map_err(|e| format!("{name}: {e}"))?;
    if solid.volume() <= 0. || solid.decompose().len() != 1 {
        return Err(format!("{name}: expected one connected printable solid"));
    }
    let (vertices, stride, triangles) = solid.to_mesh_f64();
    let mut mesh = Mesh {
        vertices: vertices
            .chunks(stride)
            .map(|v| [v[0], v[1], v[2]])
            .collect(),
        triangles: triangles
            .chunks(3)
            .map(|t| [t[0] as usize, t[1] as usize, t[2] as usize])
            .collect(),
    };
    let size = mesh.normalize();
    let volume = mesh.volume();
    Ok(Part {
        id: id.into(),
        name: name.into(),
        mesh,
        size,
        volume,
        assembled_offset: [0.; 3],
        fits: true,
    })
}

fn top_svg(part: &Part) -> String {
    let mut faces = String::new();
    for indices in &part.mesh.triangles {
        let [a, b, c] = indices.map(|i| part.mesh.vertices[i]);
        let normal_z = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        if normal_z <= 0.000001 {
            continue;
        }
        let center = (a[0] + b[0] + c[0]) / 3.;
        let color = if (12.1..16.5).contains(&center) {
            "#e58a36"
        } else if (8.0..20.6).contains(&center) {
            "#e8b570"
        } else {
            "#527bd1"
        };
        let points = [a, b, c]
            .map(|p| format!("{:.2},{:.2}", 250. + p[0] * 10., 100. + p[1] * 10.))
            .join(" ");
        faces.push_str(&format!(
            r##"<polygon points="{points}" fill="{color}" stroke="{color}" stroke-width="0.3"/>"##
        ));
    }
    format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="960" height="570" viewBox="0 0 960 570">
<rect width="960" height="570" fill="#f4f6fa"/>
<g font-family="Segoe UI,Arial,sans-serif" fill="#233148">
<text x="36" y="48" font-size="26" font-weight="600">Un scellé, deux ancrages</text>
<text x="36" y="78" font-size="16">Prototype imprimable · vue de dessus du fichier 3MF</text>
{faces}
<g fill="none" stroke="#758198" stroke-width="1.5">
<path d="M 393 115 L 635 115"/><path d="M 345 205 L 155 205"/>
<path d="M 292 334 L 155 334"/><path d="M 498 334 L 635 334"/>
</g>
<text x="650" y="120" font-size="18">Languette à soulever</text>
<text x="35" y="198" font-size="18">Petits ponts</text>
<text x="35" y="222" font-size="16">de rupture</text>
<text x="35" y="328" font-size="18">Ancrage boîte</text>
<text x="650" y="340" font-size="18">Ancrage couvercle</text>
<text x="36" y="485" font-size="19">Retirer le centre sépare les deux ancrages.</text>
<text x="36" y="516" font-size="16">Les logements et la résistance réelle restent à mettre au point sur des impressions.</text>
</g></svg>"##
    )
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let arg = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "output/printed-seal-v1".into());
    let output = Path::new(&arg);
    fs::create_dir_all(output)?;
    let mut parts = vec![
        part(holder(), "logement-a", "Logement A - coupon ouvert")?,
        part(holder(), "logement-b", "Logement B - coupon ouvert")?,
    ];
    for (index, height) in [0.6, 0.8, 1.0].into_iter().enumerate() {
        parts.push(part(
            seal(height, index + 1),
            &format!("scelle-{}", index + 1),
            &format!("Scelle {} - ponts {height:.1} mm", index + 1),
        )?);
    }
    for part in &parts {
        fs::write(
            output.join(format!("{}.stl", part.id)),
            export::stl(&part.mesh),
        )?;
    }
    fs::write(
        output.join("Boxmaker-scelle-imprime-prototype.3mf"),
        export::three_mf_parts(&parts)?,
    )?;
    fs::write(output.join("scelle-principe.svg"), top_svg(&parts[3]))?;
    fs::write(
        output.join("parts.json"),
        serde_json::to_vec_pretty(&parts)?,
    )?;
    for part in &parts {
        println!(
            "{}: {:.1} x {:.1} x {:.1} mm, {:.3} g de PLA theorique",
            part.id,
            part.size[0],
            part.size[1],
            part.size[2],
            part.volume * 0.00124
        );
    }
    println!("Export: {}", output.display());
    Ok(())
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
