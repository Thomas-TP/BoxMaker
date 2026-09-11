use crate::{Mesh, Part};
use std::io::{Cursor, Write};
use zip::{ZipWriter, write::SimpleFileOptions};

pub fn stl(mesh: &Mesh) -> Vec<u8> {
    let mut bytes = vec![0u8; 80];
    let label = b"Swiss3Design Boxmaker - millimetres - PLA prototype";
    bytes[..label.len()].copy_from_slice(label);
    bytes.extend_from_slice(&(mesh.triangles.len() as u32).to_le_bytes());
    for triangle in &mesh.triangles {
        let [a, b, c] = triangle.map(|i| mesh.vertices[i]);
        let u: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
        let v: [f64; 3] = std::array::from_fn(|i| c[i] - a[i]);
        let n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let length = n.iter().map(|v| v * v).sum::<f64>().sqrt();
        for value in n.map(|v| v / length).into_iter().chain(a).chain(b).chain(c) {
            bytes.extend_from_slice(&(value as f32).to_le_bytes());
        }
        bytes.extend_from_slice(&[0, 0]);
    }
    bytes
}

pub fn three_mf(mesh: &Mesh, name: &str) -> Result<Vec<u8>, String> {
    three_mf_objects(&[(mesh, name, 0.)])
}

/// Independent build items, in print orientation, with the same 15 mm spacing
/// as the parts preview. This is an inspection layout, not a plate packer.
pub fn three_mf_parts(parts: &[Part]) -> Result<Vec<u8>, String> {
    let mut x = 0.;
    let objects: Vec<_> = parts
        .iter()
        .map(|part| {
            let object = (&part.mesh, part.name.as_str(), x);
            x += part.size[0] + 15.;
            object
        })
        .collect();
    three_mf_objects(&objects)
}

fn three_mf_objects(objects: &[(&Mesh, &str, f64)]) -> Result<Vec<u8>, String> {
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    let mut file = |path: &str, content: &str| -> Result<(), String> {
        zip.start_file(path, options).map_err(|e| e.to_string())?;
        zip.write_all(content.as_bytes()).map_err(|e| e.to_string())
    };
    file(
        "[Content_Types].xml",
        r#"<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/><Default Extension="model" ContentType="application/vnd.ms-package.3dmanufacturing-3dmodel+xml"/></Types>"#,
    )?;
    file(
        "_rels/.rels",
        r#"<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Target="/3D/3dmodel.model" Id="rel0" Type="http://schemas.microsoft.com/3dmanufacturing/2013/01/3dmodel"/></Relationships>"#,
    )?;
    let mut resources = String::new();
    let mut build = String::new();
    for (index, (mesh, name, x)) in objects.iter().enumerate() {
        let id = index + 1;
        let name = name
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;");
        let vertices = mesh
            .vertices
            .iter()
            .map(|v| {
                format!(
                    "<vertex x=\"{:.6}\" y=\"{:.6}\" z=\"{:.6}\"/>",
                    v[0], v[1], v[2]
                )
            })
            .collect::<String>();
        let triangles = mesh
            .triangles
            .iter()
            .map(|t| {
                format!(
                    "<triangle v1=\"{}\" v2=\"{}\" v3=\"{}\"/>",
                    t[0], t[1], t[2]
                )
            })
            .collect::<String>();
        resources.push_str(&format!(
            r#"<object id="{id}" type="model" name="{name}"><mesh><vertices>{vertices}</vertices><triangles>{triangles}</triangles></mesh></object>"#
        ));
        build.push_str(&format!(
            r#"<item objectid="{id}" transform="1 0 0 0 1 0 0 0 1 {x:.6} 0 0"/>"#
        ));
    }
    let model = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><model unit="millimeter" xml:lang="fr-FR" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02"><metadata name="Application">Swiss3Design Boxmaker</metadata><resources>{resources}</resources><build>{build}</build></model>"#
    );
    file("3D/3dmodel.model", &model)?;
    zip.finish()
        .map(|c| c.into_inner())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn assert_closed_faces(faces: impl Iterator<Item = [[f64; 3]; 3]>) {
        let mut edges = HashMap::new();
        for [a, b, c] in faces {
            assert!(
                a != b && b != c && c != a,
                "collapsed export triangle: {a:?}, {b:?}, {c:?}"
            );
            let u: [f64; 3] = std::array::from_fn(|i| b[i] - a[i]);
            let v: [f64; 3] = std::array::from_fn(|i| c[i] - a[i]);
            let cross = [
                u[1] * v[2] - u[2] * v[1],
                u[2] * v[0] - u[0] * v[2],
                u[0] * v[1] - u[1] * v[0],
            ];
            assert!(cross.iter().any(|n| *n != 0.), "zero-area export triangle");
            let key = |p: [f64; 3]| p.map(|x| if x == 0. { 0 } else { x.to_bits() });
            for (u, v) in [(a, b), (b, c), (c, a)] {
                *edges.entry((key(u), key(v))).or_insert(0usize) += 1;
            }
        }
        let bad: Vec<_> = edges
            .iter()
            .filter(|&(&(u, v), &n)| n != 1 || edges.get(&(v, u)) != Some(&1))
            .take(3)
            .collect();
        assert!(
            edges
                .iter()
                .all(|(&(u, v), &n)| n == 1 && edges.get(&(v, u)) == Some(&1)),
            "export is not closed and oriented after coordinate conversion: {bad:?}"
        );
    }

    #[test]
    fn serialized_coordinates_preserve_closed_nondegenerate_meshes() {
        for (object, padding, wall, floor, clearance) in [
            ([30., 40., 10.], 0., 1.2, 0.8, 0.3),
            ([10.; 3], 0., 1.2, 0.8, 0.15),
            ([100., 70., 30.], 5., 1.2, 0.8, 0.3),
            ([220., 210., 200.], 0., 1.2, 0.8, 0.6),
            ([10.; 3], 0., 5., 6., 0.6),
        ] {
            let design = crate::calculate(&crate::Params {
                object,
                padding,
                wall,
                floor,
                clearance,
                ..Default::default()
            })
            .unwrap();
            for part in design.parts {
                println!("STL: {object:?}, gap {clearance}, {}", part.id);
                let bytes = stl(&part.mesh);
                assert_closed_faces(bytes[84..].chunks_exact(50).map(|record| {
                    std::array::from_fn(|i| {
                        std::array::from_fn(|j| {
                            let offset = 12 + i * 12 + j * 4;
                            f32::from_le_bytes(record[offset..offset + 4].try_into().unwrap())
                                as f64
                        })
                    })
                }));
                let bytes = three_mf(&part.mesh, &part.name).unwrap();
                println!("3MF: {object:?}, gap {clearance}, {}", part.id);
                let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
                let mut model = String::new();
                std::io::Read::read_to_string(
                    &mut archive.by_name("3D/3dmodel.model").unwrap(),
                    &mut model,
                )
                .unwrap();
                let attribute = |element: &str, name: &str| -> f64 {
                    element
                        .split_once(&format!("{name}=\""))
                        .unwrap()
                        .1
                        .split('"')
                        .next()
                        .unwrap()
                        .parse()
                        .unwrap()
                };
                let vertices: Vec<[f64; 3]> = model
                    .split("<vertex ")
                    .skip(1)
                    .map(|v| [attribute(v, "x"), attribute(v, "y"), attribute(v, "z")])
                    .collect();
                assert_closed_faces(
                    model.split("<triangle ").skip(1).map(|t| {
                        ["v1", "v2", "v3"].map(|name| vertices[attribute(t, name) as usize])
                    }),
                );
            }
        }
    }

    #[test]
    fn complete_3mf_keeps_independent_meshes_and_print_positions() {
        for model in ["press-slide", "legacy"] {
            let design = crate::calculate(&crate::Params {
                model: model.into(),
                floor: 2.,
                ..Default::default()
            })
            .unwrap();
            let bytes = three_mf_parts(&design.parts).unwrap();
            let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
            let mut xml = String::new();
            std::io::Read::read_to_string(
                &mut archive.by_name("3D/3dmodel.model").unwrap(),
                &mut xml,
            )
            .unwrap();
            assert_eq!(xml.matches("<object ").count(), design.parts.len());
            assert_eq!(xml.matches("<item ").count(), design.parts.len());
            assert!(!xml.contains("<components>"));
            let attribute = |element: &str, name: &str| -> String {
                element
                    .split_once(&format!("{name}=\""))
                    .unwrap()
                    .1
                    .split('"')
                    .next()
                    .unwrap()
                    .into()
            };
            let mut previous_max = None;
            for (index, ((object, item), part)) in xml
                .split("<object ")
                .skip(1)
                .zip(xml.split("<item ").skip(1))
                .zip(&design.parts)
                .enumerate()
            {
                let object = object.split_once("</object>").unwrap().0;
                let id = (index + 1).to_string();
                assert_eq!(attribute(object, "id"), id);
                assert_eq!(attribute(item, "objectid"), id);
                assert_eq!(attribute(object, "name"), part.name);
                let transform: Vec<f64> = attribute(item, "transform")
                    .split_whitespace()
                    .map(|v| v.parse().unwrap())
                    .collect();
                assert_eq!(&transform[..9], &[1., 0., 0., 0., 1., 0., 0., 0., 1.]);
                assert_eq!(&transform[10..], &[0., 0.]);
                let vertices: Vec<[f64; 3]> = object
                    .split("<vertex ")
                    .skip(1)
                    .map(|v| ["x", "y", "z"].map(|a| attribute(v, a).parse().unwrap()))
                    .collect();
                assert_eq!(vertices.len(), part.mesh.vertices.len());
                for (actual, expected) in vertices.iter().zip(&part.mesh.vertices) {
                    for axis in 0..3 {
                        assert!((actual[axis] - expected[axis]).abs() < 0.000001);
                    }
                }
                assert_eq!(
                    vertices.iter().map(|v| v[2]).fold(f64::INFINITY, f64::min),
                    0.
                );
                if let Some(max) = previous_max {
                    assert!(transform[9] - max >= 14.999);
                }
                previous_max = Some(transform[9] + part.size[0]);
                assert_eq!(
                    object.matches("<triangle ").count(),
                    part.mesh.triangles.len()
                );
                assert_closed_faces(object.split("<triangle ").skip(1).map(|t| {
                    ["v1", "v2", "v3"].map(|a| vertices[attribute(t, a).parse::<usize>().unwrap()])
                }));
            }
        }
    }

    #[test]
    fn stl_length_and_3mf_package() {
        let d = crate::calculate(&crate::Params::default()).unwrap();
        for p in d.parts {
            let bytes = stl(&p.mesh);
            assert_eq!(bytes.len(), 84 + 50 * p.mesh.triangles.len());
            let bytes = three_mf(&p.mesh, &p.name).unwrap();
            let mut archive = zip::ZipArchive::new(Cursor::new(bytes)).unwrap();
            assert_eq!(archive.len(), 3);
            let mut model = String::new();
            std::io::Read::read_to_string(
                &mut archive.by_name("3D/3dmodel.model").unwrap(),
                &mut model,
            )
            .unwrap();
            assert!(model.contains("unit=\"millimeter\""));
            assert_eq!(model.matches("<triangle ").count(), p.mesh.triangles.len());
        }
    }
}
