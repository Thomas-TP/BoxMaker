use crate::Mesh;
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
    let model = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?><model unit="millimeter" xml:lang="fr-FR" xmlns="http://schemas.microsoft.com/3dmanufacturing/core/2015/02"><metadata name="Application">Swiss3Design Boxmaker</metadata><resources><object id="1" type="model" name="{name}"><mesh><vertices>{vertices}</vertices><triangles>{triangles}</triangles></mesh></object></resources><build><item objectid="1"/></build></model>"#
    );
    file("3D/3dmodel.model", &model)?;
    zip.finish()
        .map(|c| c.into_inner())
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
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
