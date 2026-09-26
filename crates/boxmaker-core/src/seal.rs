//! Flush, single-use pin in the unused rear corner of the press-slide box.
//! The pin prints flat, then stands vertically through a rear-open lid slot.
//! Geometry is in millimetres; snap and break forces need physical validation.
use manifold_csg::{CrossSection, Manifold};

const HEAD_LEFT: f64 = 4.2;
const HEAD_RIGHT: f64 = 2.6;
const HEAD_HEIGHT: f64 = 0.8;
const PIN_THICKNESS: f64 = 1.2;
const BREAK_THICKNESS: f64 = 0.8;
const SLOT_HALF_WIDTH: f64 = 1.6;
const BORE_HALF_WIDTH: f64 = 1.15;
const POCKET_HALF_WIDTH: f64 = 1.6;
const HEAD_ABOVE_LID: f64 = 0.85;

pub fn min_inner_width(spring_width: f64, gap: f64) -> f64 {
    spring_width + 10.2 + 3. * gap
}

fn pin_x(box_width: f64, wall: f64) -> f64 {
    box_width - wall - 4.1
}

fn pin_y(depth: f64, wall: f64) -> f64 {
    depth - wall - 3.
}

fn cuboid(min: [f64; 3], max: [f64; 3]) -> Manifold {
    Manifold::cube(max[0] - min[0], max[1] - min[1], max[2] - min[2], false)
        .translate(min[0], min[1], min[2])
}

pub fn printed_seal() -> Manifold {
    // The two long arms bend in their printing plane during the one-time snap.
    // Thin roots sever the head when its edge is lifted with a fingernail.
    let mut pieces = vec![cuboid(
        [-HEAD_LEFT, 0., 0.],
        [HEAD_RIGHT, HEAD_HEIGHT, PIN_THICKNESS],
    )];
    let arm = CrossSection::from_polygons(&[vec![
        [0.25, 1.45],
        [0.95, 1.45],
        [0.95, 9.5],
        [1.35, 9.5],
        [1.35, 9.9],
        [0.95, 11.],
        [0.25, 11.],
    ]])
    .extrude(PIN_THICKNESS);
    for side in [-1., 1.] {
        pieces.push(if side < 0. {
            arm.mirror([1., 0., 0.])
        } else {
            arm.clone()
        });
        let (left, right) = if side < 0. {
            (-0.95, -0.25)
        } else {
            (0.25, 0.95)
        };
        pieces.push(cuboid(
            [left, HEAD_HEIGHT - 0.05, 0.],
            [right, 1.5, BREAK_THICKNESS],
        ));
    }
    Manifold::batch_union(&pieces)
}

fn position(box_width: f64, depth: f64, lid_top: f64, wall: f64) -> [f64; 3] {
    [
        pin_x(box_width, wall),
        pin_y(depth, wall) - PIN_THICKNESS / 2.,
        lid_top + HEAD_ABOVE_LID,
    ]
}

pub fn assembled_offset(box_width: f64, depth: f64, lid_top: f64, wall: f64) -> [f64; 3] {
    let [x, y, z] = position(box_width, depth, lid_top, wall);
    // Mesh::normalize removes the seal's local minimum x (-4.2 mm).
    [x - HEAD_LEFT, y, z]
}

pub fn assembled_solid(
    seal: &Manifold,
    box_width: f64,
    depth: f64,
    lid_top: f64,
    wall: f64,
) -> Manifold {
    let [x, y, z] = position(box_width, depth, lid_top, wall);
    seal.rotate(-90., 0., 0.).translate(x, y, z)
}

pub fn add_sockets(
    body: Manifold,
    lid: Manifold,
    box_width: f64,
    depth: f64,
    lid_z: f64,
    floor: f64,
    wall: f64,
) -> (Manifold, Manifold) {
    let x = pin_x(box_width, wall);
    let y = pin_y(depth, wall);
    let bottom = (lid_z - 11.).max(floor + 0.4);

    // A short tower joins the existing side and back walls. It sits behind the
    // guaranteed object envelope and below the lid, so it adds no outer size.
    let tower = cuboid(
        [x - 3.1, y - 2., bottom],
        [box_width - wall - 0.4, depth - wall + 0.1, lid_z - 0.3],
    );
    let bore = cuboid(
        [x - BORE_HALF_WIDTH, y - 0.9, lid_z - 7.2],
        [x + BORE_HALF_WIDTH, y + 0.9, lid_z + 0.5],
    );
    let pocket = cuboid(
        [x - POCKET_HALF_WIDTH, y - 0.9, bottom - 0.5],
        [x + POCKET_HALF_WIDTH, y + 0.9, lid_z - 7.2],
    );
    let body = &(&body + &tower) - &Manifold::batch_union(&[bore, pocket]);

    // This slot opens at the rear edge. When the head has been torn away, the
    // shank remains in the body while the lid slides forward around it.
    let lid_slot = cuboid(
        [x - SLOT_HALF_WIDTH, y - 1.3, lid_z - 0.5],
        [x + SLOT_HALF_WIDTH, depth + 1., lid_z + 4.],
    );
    (body, &lid - &lid_slot)
}
