//! Two-piece sliding enclosure: load-bearing rails and a low-travel, unloaded
//! cantilever catch. All dimensions are millimetres; this is a printable
//! prototype, not a fatigue or shipping-strength certification.
use crate::{Mesh, Params, mesh::RawPart, spring};
use manifold_csg::{CrossSection, JoinType, Manifold};
use serde::Serialize;

const RIB: f64 = 1.2;
const RIB_WIDTH: f64 = 1.2;
const LID: f64 = 1.2;
const ENGAGEMENT: f64 = 0.65;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mechanism {
    pub tongue_center: f64,
    pub tongue_half_width: f64,
    pub tongue_root: f64,
    pub hook_y: f64,
    pub release_travel: f64,
    pub beam_thickness: f64,
    pub engagement: f64,
    pub lid_z: f64,
    pub beam_length: f64,
    pub tip_thickness: f64,
    pub press_y: f64,
    pub estimated_force: f64,
    pub force_range: [f64; 2],
    pub strain_percent: f64,
    pub stop_strain_percent: f64,
    pub headroom: f64,
    pub rear_allowance: f64,
    pub deflection_profile: Vec<[f64; 2]>,
}
impl Mechanism {
    pub fn deflection_at(&self, distance: f64) -> f64 {
        let points = &self.deflection_profile;
        let position = (distance / points.last().unwrap()[0] * (points.len() - 1) as f64)
            .clamp(0., (points.len() - 1) as f64);
        let index = (position.floor() as usize).min(points.len() - 2);
        points[index][1] + (position - index as f64) * (points[index + 1][1] - points[index][1])
    }
}

pub(crate) struct Layout {
    pub spring: spring::Spring,
    pub inner: [f64; 3],
    pub outer: [f64; 3],
    pub lid_z: f64,
    pub headroom: f64,
    pub stop: f64,
    pub rear: f64,
    pub bridge_bottom: f64,
}

pub(crate) fn layout(p: &Params) -> Layout {
    let requested = p.object.map(|v| v + 2. * (p.padding + p.object_clearance));
    let (spring, inner) = spring::choose(requested, p.clearance);
    let rear = spring::BRIDGE + 2.5 + 2. * p.clearance + 0.3;
    let headroom = spring.deflection(spring.length + 2.8) + 0.15;
    let stop = headroom;
    let lid_z = p.floor + RIB + inner[2] + headroom;
    let bridge_bottom =
        lid_z + spring.thickness(spring.length - spring::BRIDGE - p.clearance) + p.clearance;
    let h =
        ((bridge_bottom + 1.2).max(lid_z + spring.root_thickness + 0.5) * 10_000.).ceil() / 10_000.;
    Layout {
        inner,
        outer: [
            inner[0] + 2. * (p.wall + RIB),
            inner[1] + 2. * p.wall + RIB + rear,
            h,
        ],
        spring,
        lid_z,
        headroom,
        stop,
        rear,
        bridge_bottom,
    }
}

pub(crate) struct Assembly {
    pub inner: [f64; 3],
    pub outer: [f64; 3],
    pub object_offset: [f64; 3],
    pub mechanism: Mechanism,
    pub body: Manifold,
    pub lid: Manifold,
}

fn cuboid(min: [f64; 3], max: [f64; 3]) -> Manifold {
    Manifold::cube(max[0] - min[0], max[1] - min[1], max[2] - min[2], false)
        .translate(min[0], min[1], min[2])
}

fn rounded_rect(x: f64, y: f64, w: f64, d: f64, r: f64) -> CrossSection {
    CrossSection::square(w - 2. * r, d - 2. * r, false)
        .offset(r, JoinType::Round, 2., 32)
        .translate(x + r, y + r)
}

fn ribs_between(start: f64, end: f64) -> Vec<f64> {
    // Bound work even for out-of-printer-range inputs; printable parts have
    // a rib at most every 28 mm. Every rib is supported by the continuous skin.
    let count = ((end - start) / 28.).ceil().clamp(1., 12.) as usize;
    (1..count)
        .map(|i| start + (end - start) * i as f64 / count as f64)
        .collect()
}

fn prism_xz(points: &[[f64; 2]], length: f64) -> Manifold {
    // Extrude an X/Z profile along positive Y.
    let polygon = points.iter().rev().map(|p| [p[0], -p[1]]).collect();
    CrossSection::from_polygons(&[polygon])
        .extrude(length)
        .rotate(-90., 0., 0.)
}

fn printable(solid: &Manifold) -> Result<Manifold, String> {
    let (mut vertices, stride, triangles) = solid.to_mesh_f64();
    // Resolve floating-point seams on a 0.1 micron grid, then let Manifold
    // rebuild the topology instead of dropping triangles from an open mesh.
    for value in &mut vertices {
        *value = (*value * 10_000.).round() / 10_000.;
    }
    Manifold::from_mesh_f64(&vertices, stride, &triangles)
        .map(|s| s.as_original().simplify(0.0001))
        .map_err(|e| format!("Préparation du maillage imprimable : {e}"))
}

pub(crate) fn assembly(p: &Params) -> Result<Assembly, String> {
    let layout = layout(p);
    let inner = layout.inner;
    let spring = &layout.spring;
    let t = p.wall;
    let g = p.clearance;
    let front = t + RIB;
    let [w, d, h] = layout.outer;
    let lid_z = layout.lid_z;
    let roof_z = lid_z + LID + g;
    let slot_width = 0.8 + 2. * g;
    let half_tongue = spring.width / 2.;
    // Keep the root beside a rail rather than on the flexible middle of a
    // large panel. Its reinforced connection has a bounded lateral span.
    let cx = t + RIB + g + slot_width + half_tongue + 0.8;
    let bridge_start = d - t - layout.rear;
    let bridge_end = bridge_start + spring::BRIDGE;
    let hook_y = bridge_end + g;
    let root = hook_y - spring.length;
    let stop = layout.stop;
    let outer = rounded_rect(0., 0., w, d, t + RIB);
    let cavity = rounded_rect(t, t, w - 2. * t, d - 2. * t, RIB);
    let shell = &outer.extrude(h) - &cavity.extrude(h + 2.).translate(0., 0., p.floor);
    let mut adds = vec![shell];
    // Internal floor lattice and vertical ribs keep the exterior closed and
    // smooth. The guaranteed object envelope starts above / inside all ribs.
    for x in ribs_between(t + RIB, w - t - RIB) {
        adds.push(cuboid(
            [x - RIB_WIDTH / 2., t, p.floor],
            [x + RIB_WIDTH / 2., d - t, p.floor + RIB],
        ));
        adds.push(cuboid(
            [x - RIB_WIDTH / 2., t, p.floor],
            [x + RIB_WIDTH / 2., t + RIB, lid_z - g],
        ));
        adds.push(cuboid(
            [x - RIB_WIDTH / 2., d - t - RIB, p.floor],
            [x + RIB_WIDTH / 2., d - t, lid_z - g],
        ));
    }
    for y in ribs_between(front, d - t - RIB) {
        adds.push(cuboid(
            [t, y - RIB_WIDTH / 2., p.floor],
            [w - t, y + RIB_WIDTH / 2., p.floor + RIB],
        ));
        for x in [t, w - t - RIB] {
            adds.push(cuboid(
                [x, y - RIB_WIDTH / 2., p.floor],
                [x + RIB, y + RIB_WIDTH / 2., lid_z - g],
            ));
        }
    }
    // The 45-degree underside grows out of the wall instead of beginning as
    // an unsupported horizontal shelf. Slot clearance is explicit above/below.
    {
        let z = lid_z - g - RIB;
        let rail = prism_xz(
            &[
                [t - 0.1, z - RIB],
                [t + RIB, z],
                [t + RIB, z + RIB],
                [t - 0.1, z + RIB],
            ],
            d - t,
        );
        adds.push(rail.clone());
        adds.push(rail.mirror([1., 0., 0.]).translate(w, 0., 0.));
    }
    for x in [0., w - t - RIB] {
        adds.push(cuboid([x, 0., roof_z], [x + t + RIB, d - t, h]));
    }
    // Two slender pillars pass through the open slots of the lid. A short
    // bridge across them is the fixed striker. No deep solid central boss.
    for side in [-1., 1.] {
        let x = cx + side * (half_tongue + slot_width / 2.);
        adds.push(cuboid([x - 0.4, bridge_start, 0.], [x + 0.4, d, h]));
    }
    adds.push(cuboid(
        [
            cx - half_tongue - slot_width / 2. - 0.4,
            bridge_start,
            layout.bridge_bottom,
        ],
        [cx + half_tongue + slot_width / 2. + 0.4, bridge_end, h],
    ));
    let stop_profile = CrossSection::from_polygons(&[vec![
        [bridge_start, lid_z - stop - 1.],
        [d, lid_z - stop - 1. - (d - bridge_start)],
        [d, lid_z - stop],
        [bridge_start, lid_z - stop],
    ]]);
    adds.push(
        stop_profile
            .extrude(spring.width)
            .rotate(90., 0., 90.)
            .translate(cx - half_tongue, 0., 0.),
    );
    let mut body = Manifold::batch_union(&adds).intersection(&outer.extrude(h));
    let cuts = vec![
        // Open the insertion slot and relieve the curved front corners.
        cuboid([-1., -1., lid_z - g], [w + 1., t + RIB + 0.1, h + 1.]),
        // Tongue travel pocket. Its floor is also an over-travel stop.
        cuboid(
            [cx - half_tongue - g, bridge_start - 0.01, lid_z - stop],
            [cx + half_tongue + g, d + 1., layout.bridge_bottom],
        ),
    ];
    body = &body - &Manifold::batch_union(&cuts);

    let lid_x = t + g;
    let lid_y = g;
    let lid_w = w - 2. * (t + g);
    let lid_d = d - t - 2. * g;
    let outline = rounded_rect(lid_x, lid_y, lid_w, lid_d, 0.7);
    let mut lid = outline.extrude(LID).translate(0., 0., lid_z);
    let mut slots = vec![];
    for side in [-1., 1.] {
        let x = cx + side * (half_tongue + slot_width / 2.);
        let slot = rounded_rect(
            x - slot_width / 2.,
            root,
            slot_width,
            d + 2. - root,
            slot_width / 2. - 0.01,
        );
        slots.push(slot.extrude(LID + 4.).translate(0., 0., lid_z - 1.));
    }
    lid = &lid - &Manifold::batch_union(&slots);
    // The leading ramp pushes the long tongue down on closing. The vertical
    // front shoulder catches behind the bridge, without sustained deflection.
    let tooth_height = layout.bridge_bottom - lid_z - spring.tip_thickness + ENGAGEMENT;
    let tooth_profile = CrossSection::from_polygons(&[vec![
        [hook_y, 0.],
        [hook_y + 2.5, 0.],
        [hook_y + 2.5, 0.05],
        [hook_y, tooth_height],
    ]]);
    let tooth = tooth_profile
        .extrude(spring.width - 2.)
        .rotate(90., 0., 90.)
        .translate(cx - half_tongue + 1., 0., lid_z + spring.tip_thickness);
    let beam = CrossSection::from_polygons(&[vec![
        [root - 3., 0.],
        [lid_y + lid_d, 0.],
        [lid_y + lid_d, spring.tip_thickness],
        [hook_y, spring.tip_thickness],
        [root, spring.root_thickness],
        [root - 3., spring.root_thickness],
    ]])
    .extrude(spring.width)
    .rotate(90., 0., 90.)
    .translate(cx - half_tongue, 0., lid_z);
    let root_support = cuboid(
        [t + RIB + g + 0.05, root - 3., lid_z],
        [
            cx + half_tongue + slot_width + 0.8,
            root,
            lid_z + spring.root_thickness + 0.4,
        ],
    );
    let mut lid_adds = vec![lid, tooth, beam, root_support];
    // Raised finger texture identifies the press area, protected below the rim.
    for y in [hook_y - 3.7, hook_y - 4.7, hook_y - 5.7] {
        lid_adds.push(
            rounded_rect(cx - half_tongue + 1., y, spring.width - 2., 0.6, 0.25)
                .extrude(0.25 + spring.thickness(y - root) - spring.tip_thickness)
                .translate(0., 0., lid_z + spring.tip_thickness),
        );
    }
    // Low top ribs print upwards from a flat underside. Keep the compliant
    // tongue and the upper side rails free of reinforcement.
    let rib_mask = cuboid(
        [t + RIB + g + 0.4, t + 0.8, lid_z + LID],
        [w - t - RIB - g - 0.4, d - t - g - 1., lid_z + LID + 0.8],
    );
    let flex_bay = cuboid(
        [cx - half_tongue - slot_width - 1.5, root - 3.2, lid_z],
        [cx + half_tongue + slot_width + 1.5, d + 1., h + 1.],
    );
    let mut top_ribs = vec![];
    for x in ribs_between(lid_x, lid_x + lid_w) {
        top_ribs.push(cuboid(
            [x - 0.6, lid_y, lid_z + LID],
            [x + 0.6, lid_y + lid_d, lid_z + LID + 0.8],
        ));
    }
    for y in ribs_between(lid_y, lid_y + lid_d) {
        top_ribs.push(cuboid(
            [lid_x, y - 0.6, lid_z + LID],
            [lid_x + lid_w, y + 0.6, lid_z + LID + 0.8],
        ));
    }
    if !top_ribs.is_empty() {
        lid_adds.push(&Manifold::batch_union(&top_ribs).intersection(&rib_mask) - &flex_bay);
    }
    // Boolean intersections can leave sub-micron edges. Remove them before
    // float32 STL / six-decimal 3MF conversion collapses adjacent vertices.
    // 0.0001 mm is far below printing clearance and layer dimensions.
    let body = printable(&body)?;
    let lid = printable(&Manifold::batch_union(&lid_adds))?;
    for (name, solid) in [("boîte", &body), ("couvercle", &lid)] {
        solid
            .status()
            .map_err(|e| format!("Géométrie du {name} invalide : {e}"))?;
        if solid.volume() <= 0. || solid.decompose().len() != 1 {
            return Err(format!("La pièce {name} doit former un seul volume fermé."));
        }
    }
    Ok(Assembly {
        inner,
        outer: [w, d, h],
        object_offset: [
            t + RIB + (inner[0] - p.object[0]) / 2.,
            front + (inner[1] - p.object[1]) / 2.,
            p.floor + RIB + p.padding + p.object_clearance,
        ],
        mechanism: Mechanism {
            tongue_center: cx,
            tongue_half_width: half_tongue,
            tongue_root: root,
            hook_y,
            release_travel: spring::RELEASE,
            beam_thickness: spring.root_thickness,
            engagement: ENGAGEMENT,
            lid_z,
            beam_length: spring.length,
            tip_thickness: spring.tip_thickness,
            press_y: hook_y - spring::FINGER_OFFSET,
            estimated_force: spring.force,
            force_range: [
                spring.force * 2000. / spring::MODULUS,
                spring.force * 3500. / spring::MODULUS,
            ],
            strain_percent: spring.strain * 100.,
            stop_strain_percent: spring.strain * 100. * stop
                / spring.deflection(spring.length - spring::BRIDGE - g),
            headroom: layout.headroom,
            rear_allowance: layout.rear,
            deflection_profile: (0..=128)
                .map(|i| {
                    let y = (spring.length + 3.) * i as f64 / 128.;
                    [y, spring.deflection(y)]
                })
                .collect(),
        },
        body,
        lid,
    })
}

fn mesh(solid: &Manifold) -> Mesh {
    let (vertices, stride, triangles) = solid.to_mesh_f64();
    Mesh {
        vertices: vertices
            .chunks_exact(stride)
            .map(|p| [p[0], p[1], p[2]])
            .collect(),
        triangles: triangles
            .as_chunks::<3>()
            .0
            .iter()
            .map(|t| [t[0] as usize, t[1] as usize, t[2] as usize])
            .collect(),
    }
}

type DesignOutput = ([f64; 3], [f64; 3], [f64; 3], Vec<RawPart>, Mechanism);
pub fn design(p: &Params) -> Result<DesignOutput, String> {
    let a = assembly(p)?;
    let bounds = a.lid.bounding_box().ok_or("Couvercle vide")?;
    Ok((
        a.inner,
        a.outer,
        a.object_offset,
        vec![
            ("body", "Boîte nervurée", mesh(&a.body), [0.; 3]),
            ("lid", "Couvercle à pression", mesh(&a.lid), bounds.min()),
        ],
        a.mechanism,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_mesh(s: &Manifold) {
        let m = mesh(s);
        let mut edges = HashMap::<(usize, usize), (usize, i32)>::new();
        for triangle in &m.triangles {
            for (a, b) in [
                (triangle[0], triangle[1]),
                (triangle[1], triangle[2]),
                (triangle[2], triangle[0]),
            ] {
                let edge = edges.entry((a.min(b), a.max(b))).or_default();
                edge.0 += 1;
                edge.1 += if a < b { 1 } else { -1 };
            }
        }
        assert!(
            edges.values().all(|e| *e == (2, 0)),
            "exported mesh is not closed and oriented"
        );
        assert!((m.volume() - s.volume()).abs() < 1e-5);
        assert!(m.volume() > 0.);
    }

    fn pressed(a: &Assembly) -> Manifold {
        let m = &a.mechanism;
        a.lid.warp(|x, y, z| {
            let mut p = [x, y, z];
            if (p[0] - m.tongue_center).abs() <= m.tongue_half_width + 0.001 && p[1] > m.tongue_root
            {
                p[2] -= m.deflection_at(p[1] - m.tongue_root);
            }
            p
        })
    }

    #[test]
    fn watertight_nonintersecting_parts_and_free_object_space() {
        for (size, wall, floor, gap) in [
            ([10.; 3], 1.2, 0.8, 0.15),
            ([100., 70., 30.], 1.2, 0.8, 0.3),
            ([220., 210., 200.], 1.2, 0.8, 0.6),
            ([10.; 3], 5., 6., 0.6),
            ([220., 30., 10.], 2.4, 1.2, 0.15),
        ] {
            let p = Params {
                object: size,
                wall,
                floor,
                clearance: gap,
                padding: 0.,
                ..Default::default()
            };
            let a = assembly(&p).unwrap();
            assert_mesh(&a.body);
            assert_mesh(&a.lid);
            assert!(
                a.body.intersection(&a.lid).volume() < 1e-5,
                "closed parts overlap: {p:?}"
            );
            let free = cuboid(
                a.object_offset,
                std::array::from_fn(|i| a.object_offset[i] + size[i]),
            );
            assert!(
                a.body.intersection(&free).volume() < 1e-5,
                "body invades the object: {p:?}"
            );
            assert!(a.lid.intersection(&free).volume() < 1e-5);
            for s in [&a.body, &a.lid] {
                for v in mesh(s).vertices {
                    for (i, value) in v.iter().enumerate() {
                        assert!(
                            *value >= -1e-6 && *value <= a.outer[i] + 1e-6,
                            "outside: {v:?}, outer {:?}, params {p:?}",
                            a.outer
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn latch_blocks_sliding_but_releases_when_pressed() {
        for object in [
            [10.; 3],
            [30., 40., 10.],
            [100., 70., 30.],
            [220., 210., 200.],
        ] {
            for gap in [0.15, 0.3, 0.6] {
                let a = assembly(&Params {
                    object,
                    clearance: gap,
                    ..Default::default()
                })
                .unwrap();
                assert!(
                    a.body
                        .intersection(&a.lid.translate(0., -gap - 0.4, 0.))
                        .volume()
                        > 0.05,
                    "shoulder must block the closed lid"
                );
                let open = pressed(&a);
                let free = cuboid(
                    a.object_offset,
                    std::array::from_fn(|i| a.object_offset[i] + object[i]),
                );
                for displacement in [0., 0.5, 1., 2., 4., 6., 8.] {
                    let intersection = a
                        .body
                        .intersection(&open.translate(0., -displacement, 0.))
                        .volume();
                    assert!(
                        open.translate(0., -displacement, 0.)
                            .intersection(&free)
                            .volume()
                            < 1e-5,
                        "pressed lid touches the object"
                    );
                    assert!(
                        intersection < 1e-5,
                        "pressed lid collides at travel {displacement}, gap {gap}: {intersection} mm³, bounds {:?}",
                        a.body
                            .intersection(&open.translate(0., -displacement, 0.))
                            .bounding_box()
                    );
                }
                for displacement in [6., 8., 10., 20., 40., 70., 95.] {
                    assert!(
                        a.body
                            .intersection(&a.lid.translate(0., -displacement, 0.))
                            .volume()
                            < 1e-5,
                        "released lid must slide freely after disengagement"
                    );
                }
            }
        }
    }

    #[test]
    fn saves_material_at_equal_usable_volume() {
        for object in [[100., 70., 30.], [150., 100., 50.], [220., 180., 100.]] {
            let d = crate::calculate(&Params {
                object,
                ..Default::default()
            })
            .unwrap();
            assert!(
                d.plastic_weight < d.reference_plastic_weight * 0.8,
                "at least 20% lighter for {object:?}: {} vs {}",
                d.plastic_weight,
                d.reference_plastic_weight
            );
            println!(
                "{object:?}: {:.2} g vs {:.2} g ({:.1}% saved)",
                d.plastic_weight,
                d.reference_plastic_weight,
                (1. - d.plastic_weight / d.reference_plastic_weight) * 100.
            );
        }
    }
}
