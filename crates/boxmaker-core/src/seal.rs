//! Optional, single-use printed bridge between the sliding lid and the body.
//! Geometry is in millimetres. No insertion or break force is certified.
use manifold_csg::{CrossSection, Manifold};

pub const BODY_EAR: f64 = 8.5;
pub const REAR_EXTENSION: f64 = 14.;
pub const MIN_INNER_WIDTH_OVER_SPRING: f64 = 16.5;
pub const BOSS_ABOVE_LID: f64 = 4.2;
const SEAL_THICKNESS: f64 = 1.6;
const BRIDGE_THICKNESS: f64 = 0.8;
const HOLE_GAP: f64 = 0.3;

fn cuboid(min: [f64; 3], max: [f64; 3]) -> Manifold {
    Manifold::cube(max[0] - min[0], max[1] - min[1], max[2] - min[2], false)
        .translate(min[0], min[1], min[2])
}

fn prong(center: f64) -> Vec<Manifold> {
    let mut pieces = vec![cuboid(
        [center - 4., 0., 0.],
        [center + 4., 8., SEAL_THICKNESS],
    )];
    for side in [-1., 1.] {
        let section = CrossSection::from_polygons(&[vec![
            [1.3, 7.5],
            [2.5, 7.5],
            [2.5, 22.6],
            [3.25, 22.6],
            [2.5, 25.6],
            [1.3, 25.6],
        ]]);
        let arm = section.extrude(SEAL_THICKNESS);
        pieces.push(
            (if side < 0. {
                arm.mirror([1., 0., 0.])
            } else {
                arm
            })
            .translate(center, 0., 0.),
        );
    }
    pieces
}

pub fn printed_seal() -> Manifold {
    // The left anchor belongs to the lid, the right anchor to the body.
    // Removing the middle bridge severs the only connection between them.
    let mut pieces = prong(4.);
    pieces.extend(prong(18.));
    pieces.push(cuboid([9.4, -5.8, 0.], [12.6, 8., SEAL_THICKNESS]));
    for y in [1., 3.5, 6.] {
        pieces.push(cuboid([7.9, y, 0.], [9.5, y + 1., BRIDGE_THICKNESS]));
        pieces.push(cuboid([12.5, y, 0.], [14.1, y + 1., BRIDGE_THICKNESS]));
    }
    for y in [-4.8, -3.3] {
        pieces.push(cuboid([9.7, y, SEAL_THICKNESS - 0.1], [12.3, y + 0.6, 2.]));
    }
    Manifold::batch_union(&pieces)
}

pub fn assembled_offset(box_width: f64, depth: f64, lid_top: f64) -> [f64; 3] {
    // Mesh::normalize removes the seal's local minimum y (-5.8 mm).
    [box_width - 14., depth - 38.8, lid_top + 1.1]
}

pub fn add_sockets(
    body: Manifold,
    lid: Manifold,
    box_width: f64,
    depth: f64,
    lid_top: f64,
    height: f64,
) -> (Manifold, Manifold) {
    let mouth = depth - 25.;
    let seal_z = lid_top + 1.1;
    let low = seal_z - HOLE_GAP;
    let high = seal_z + SEAL_THICKNESS + HOLE_GAP;
    let body_x = box_width + 4.;
    let lid_x = box_width - 10.;

    let body_boss = cuboid(
        [box_width - 0.4, mouth, lid_top - 3.],
        [box_width + BODY_EAR, depth - 4., height],
    );
    let lid_boss = cuboid(
        [lid_x - 4.5, mouth, lid_top - 0.1],
        [lid_x + 4.5, depth - 4., height],
    );
    let socket = |x: f64| {
        Manifold::batch_union(&[
            cuboid([x - 2.8, mouth - 0.2, low], [x + 2.8, mouth + 14.2, high]),
            cuboid([x - 3.7, mouth + 14.2, low], [x + 3.7, mouth + 18.4, high]),
        ])
    };
    let body = &(&body + &body_boss) - &socket(body_x);
    let lid = &(&lid + &lid_boss) - &socket(lid_x);

    // These release windows face the closed box's interior. They are reachable
    // only after opening, when the two broken anchor remnants can be removed.
    let body_release = cuboid(
        [box_width - 1.6, mouth + 15., low],
        [body_x + 2.1, mouth + 18.4, high],
    );
    let lid_release = cuboid(
        [lid_x - 3.2, mouth + 15., lid_top - 1.4],
        [lid_x + 3.2, mouth + 18.4, low + 0.2],
    );
    (&body - &body_release, &lid - &lid_release)
}
