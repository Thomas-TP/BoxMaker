use crate::Params;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, Debug, Clone)]
pub struct Mesh {
    pub vertices: Vec<[f64; 3]>,
    pub triangles: Vec<[usize; 3]>,
}

impl Mesh {
    pub fn normalize(&mut self) -> [f64; 3] {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];
        for v in &self.vertices {
            for a in 0..3 {
                min[a] = min[a].min(v[a]);
                max[a] = max[a].max(v[a]);
            }
        }
        for v in &mut self.vertices {
            for a in 0..3 {
                v[a] -= min[a];
            }
        }
        std::array::from_fn(|a| max[a] - min[a])
    }
    pub fn volume(&self) -> f64 {
        self.triangles
            .iter()
            .map(|t| {
                let [a, b, c] = t.map(|i| self.vertices[i]);
                (a[0] * (b[1] * c[2] - b[2] * c[1])
                    + a[1] * (b[2] * c[0] - b[0] * c[2])
                    + a[2] * (b[0] * c[1] - b[1] * c[0]))
                    / 6.
            })
            .sum()
    }
}

#[derive(Clone, Copy)]
struct Block {
    min: [f64; 3],
    max: [f64; 3],
}
fn block(min: [f64; 3], max: [f64; 3]) -> Block {
    Block { min, max }
}
impl Block {
    fn contains(&self, p: [f64; 3]) -> bool {
        (0..3).all(|a| p[a] > self.min[a] && p[a] < self.max[a])
    }
}

/// Exact rectilinear CSG: split on every primitive boundary, classify cells,
/// emit only external faces with shared indexed vertices. No voxel resolution
/// or approximation; no internal overlapping shells or T-junctions.
fn solid(add: &[Block], subtract: &[Block]) -> Mesh {
    let axes: [Vec<f64>; 3] = std::array::from_fn(|a| {
        let mut values: Vec<f64> = add
            .iter()
            .chain(subtract)
            .flat_map(|b| [b.min[a], b.max[a]])
            .collect();
        values.sort_by(f64::total_cmp);
        values.dedup_by(|a, b| (*a - *b).abs() < 1e-8);
        values
    });
    let n = axes.each_ref().map(|a| a.len() - 1);
    let index = |x: usize, y: usize, z: usize| (x * n[1] + y) * n[2] + z;
    let mut cells = vec![false; n[0] * n[1] * n[2]];
    let mut mesh = Mesh {
        vertices: vec![],
        triangles: vec![],
    };
    let mut lookup = HashMap::<[usize; 3], usize>::new();
    for x in 0..n[0] {
        for y in 0..n[1] {
            for z in 0..n[2] {
                let indices = [x, y, z];
                let p =
                    std::array::from_fn(|a| (axes[a][indices[a]] + axes[a][indices[a] + 1]) / 2.);
                cells[index(x, y, z)] =
                    add.iter().any(|b| b.contains(p)) && !subtract.iter().any(|b| b.contains(p));
            }
        }
    }
    for x in 0..n[0] {
        for y in 0..n[1] {
            for z in 0..n[2] {
                if !cells[index(x, y, z)] {
                    continue;
                }
                let faces = [
                    (
                        x == 0 || !cells[index(x.saturating_sub(1), y, z)],
                        [[x, y, z], [x, y, z + 1], [x, y + 1, z + 1], [x, y + 1, z]],
                    ),
                    (
                        x + 1 == n[0] || !cells[index((x + 1).min(n[0] - 1), y, z)],
                        [
                            [x + 1, y, z],
                            [x + 1, y + 1, z],
                            [x + 1, y + 1, z + 1],
                            [x + 1, y, z + 1],
                        ],
                    ),
                    (
                        y == 0 || !cells[index(x, y.saturating_sub(1), z)],
                        [[x, y, z], [x + 1, y, z], [x + 1, y, z + 1], [x, y, z + 1]],
                    ),
                    (
                        y + 1 == n[1] || !cells[index(x, (y + 1).min(n[1] - 1), z)],
                        [
                            [x, y + 1, z],
                            [x, y + 1, z + 1],
                            [x + 1, y + 1, z + 1],
                            [x + 1, y + 1, z],
                        ],
                    ),
                    (
                        z == 0 || !cells[index(x, y, z.saturating_sub(1))],
                        [[x, y, z], [x, y + 1, z], [x + 1, y + 1, z], [x + 1, y, z]],
                    ),
                    (
                        z + 1 == n[2] || !cells[index(x, y, (z + 1).min(n[2] - 1))],
                        [
                            [x, y, z + 1],
                            [x + 1, y, z + 1],
                            [x + 1, y + 1, z + 1],
                            [x, y + 1, z + 1],
                        ],
                    ),
                ];
                for (exposed, corners) in faces {
                    if exposed {
                        let ids = corners.map(|corner| {
                            *lookup.entry(corner).or_insert_with(|| {
                                let id = mesh.vertices.len();
                                mesh.vertices
                                    .push(std::array::from_fn(|a| axes[a][corner[a]]));
                                id
                            })
                        });
                        mesh.triangles.push([ids[0], ids[1], ids[2]]);
                        mesh.triangles.push([ids[0], ids[2], ids[3]]);
                    }
                }
            }
        }
    }
    mesh
}

pub(crate) type RawPart = (&'static str, &'static str, Mesh, [f64; 3]);
pub fn design(p: &Params) -> ([f64; 3], [f64; 3], [f64; 3], Vec<RawPart>) {
    let inner = p.object.map(|v| v + 2. * p.padding);
    let t = p.wall;
    let g = p.clearance;
    let rail = 1.2;
    let lid_t = p.wall;
    let front = 8.;
    let w = inner[0] + 2. * (t + rail);
    let d = inner[1] + front + t;
    let lid_z = p.floor + inner[2];
    let h = lid_z + lid_t + g + 1.2;
    let upper_z = lid_z + lid_t + g;
    let key_x = w / 2.;
    let key_y = front / 2.;
    let adds = [
        block([0., 0., 0.], [w, d, p.floor]),
        block([0., 0., 0.], [t, d, h]),
        block([w - t, 0., 0.], [w, d, h]),
        block([0., d - t, 0.], [w, d, h]),
        block([0., 0., 0.], [w, t, lid_z - g]),
        // Only the key socket needs a deep reinforcement; keep the remaining
        // front wall thin to avoid spending PLA across the entire face.
        block([key_x - 6., 0., 0.], [key_x + 6., front, lid_z - g]),
        block([0., 0., lid_z - g - 1.2], [t + rail, d, lid_z - g]),
        block([w - t - rail, 0., lid_z - g - 1.2], [w, d, lid_z - g]),
        block([0., 0., upper_z], [t + rail, d, h]),
        block([w - t - rail, 0., upper_z], [w, d, h]),
    ];
    let socket = block(
        [key_x - 2. - g, key_y - 1. - g, lid_z - 4. - g],
        [key_x + 2. + g, key_y + 1. + g, h + 1.],
    );
    let body = solid(&adds, &[socket]);
    let lid_min = [t + g, g, lid_z];
    let lid = solid(
        &[block(lid_min, [w - t - g, d - t - g, lid_z + lid_t])],
        &[socket],
    );
    // Print the key upside down: broad head on the bed, shaft upwards.
    let shaft_h = 4. + lid_t;
    let key = solid(
        &[
            block([0., 0., 0.], [8., 5., 1.2]),
            block([2., 1.5, 1.2], [6., 3.5, 1.2 + shaft_h]),
        ],
        &[],
    );
    // Key preview uses an upside-down transform in the viewer.
    let key_offset = [key_x - 4., key_y - 2.5, lid_z + lid_t + 1.2];
    (
        inner,
        [w, d, h],
        [t + rail + p.padding, front + p.padding, p.floor + p.padding],
        vec![
            ("body", "Boîte", body, [0.; 3]),
            ("lid", "Couvercle", lid, lid_min),
            ("key", "Clavette", key, key_offset),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    fn closed(mesh: &Mesh) {
        let mut edges = HashMap::<(usize, usize), (usize, i32)>::new();
        for t in &mesh.triangles {
            for (a, b) in [(t[0], t[1]), (t[1], t[2]), (t[2], t[0])] {
                let edge = edges.entry((a.min(b), a.max(b))).or_default();
                edge.0 += 1;
                edge.1 += if a < b { 1 } else { -1 };
            }
        }
        assert!(
            edges.values().all(|e| *e == (2, 0)),
            "Every edge must have two opposite faces"
        );
        assert!(mesh.volume() > 0.);
    }
    #[test]
    fn exact_box_volume_and_orientation() {
        let m = solid(&[block([0.; 3], [10., 20., 30.])], &[]);
        closed(&m);
        assert!((m.volume() - 6000.).abs() < 1e-6);
    }
    #[test]
    fn all_parts_closed_for_parameter_extremes() {
        for wall in [1.2, 1.6, 5.] {
            for clearance in [0.15, 0.3, 0.6] {
                for size in [[10.; 3], [100., 70., 30.], [220., 210., 200.]] {
                    let p = Params {
                        wall,
                        clearance,
                        object: size,
                        padding: 0.,
                        ..Default::default()
                    };
                    let (_, _, _, parts) = design(&p);
                    for (_, _, mesh, _) in parts {
                        closed(&mesh);
                    }
                }
            }
        }
    }
    #[test]
    fn fits_object_and_key_stays_in_envelope() {
        let p = Params::default();
        let (inner, outer, offset, parts) = design(&p);
        for a in 0..3 {
            assert!(inner[a] >= p.object[a] + 2. * p.padding);
            assert!(offset[a] + p.object[a] < outer[a]);
        }
        assert!(parts[2].3[2] <= outer[2]);
    }
}
