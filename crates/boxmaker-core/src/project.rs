use crate::{Params, validate};
use serde::{Deserialize, Serialize};

pub const VERSION: u32 = 5;

#[derive(Deserialize)]
struct Project {
    version: u32,
    params: Params,
}

#[derive(Debug, Serialize)]
pub struct LoadedProject {
    pub params: Params,
    pub message: &'static str,
}

pub fn load(value: serde_json::Value) -> Result<LoadedProject, String> {
    if value.to_string().len() > 32_768 {
        return Err("Fichier trop volumineux".into());
    }
    let version = value
        .get("version")
        .and_then(serde_json::Value::as_u64)
        .ok_or("Projet Boxmaker incompatible")?;
    if version > u64::from(VERSION) {
        return Err("Ce projet nécessite une version plus récente de Boxmaker. Mettez l’application à jour.".into());
    }
    if version == 0 {
        return Err("Projet Boxmaker incompatible".into());
    }
    let mut project: Project = serde_json::from_value(value).map_err(|_| {
        "Le projet est incomplet ou contient des paramètres non valides.".to_string()
    })?;
    validate(&project.params)?;
    let migrated = project.params.model == "press-slide" && project.version < 3;
    let compact_seal =
        project.params.model == "press-slide" && project.params.seal && project.version < 5;
    if migrated || compact_seal {
        project.params.measured_total = None;
    }
    Ok(LoadedProject {
        params: project.params,
        message: if migrated {
            "Projet adapté à la nouvelle fermeture et orienté automatiquement. Repesez l’envoi."
        } else if compact_seal {
            "Scellé compact actualisé. Repesez l’envoi avant expédition."
        } else {
            "Projet chargé."
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn supported_versions_round_trip_and_migrate_measured_weight() {
        for version in 1..=VERSION {
            let params = Params {
                seal: true,
                measured_total: Some(180.),
                ..Default::default()
            };
            let result = load(serde_json::json!({"version": version, "params": params})).unwrap();
            assert_eq!(result.params.object, [100., 70., 30.]);
            assert_eq!(result.params.seal, params.seal);
            assert_eq!(
                result.params.measured_total,
                if version < 5 { None } else { Some(180.) }
            );
            let saved = serde_json::json!({"version": VERSION, "params": result.params});
            assert_eq!(load(saved).unwrap().params.padding, 0.);
        }
    }

    #[test]
    fn actual_legacy_fixtures_preserve_explicit_padding_and_model() {
        let legacy = load(
            serde_json::from_str(include_str!(
                "../../../scripts/fixtures/legacy.boxmaker.json"
            ))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(legacy.params.model, "legacy");
        assert!(!legacy.params.seal);
        assert_eq!(legacy.params.padding, 5.);
        assert_eq!(legacy.params.object_clearance, 0.3);
        let press = load(
            serde_json::from_str(include_str!(
                "../../../scripts/fixtures/press-slide-v2.boxmaker.json"
            ))
            .unwrap(),
        )
        .unwrap();
        assert_eq!(press.params.model, "press-slide");
        assert_eq!(press.params.padding, 5.);
        assert_eq!(press.params.measured_total, None);
    }

    #[test]
    fn corrupt_future_and_out_of_range_projects_are_rejected() {
        for value in [
            serde_json::json!({}),
            serde_json::json!({"version": 0}),
            serde_json::json!({"version": 6}),
            serde_json::json!({"version": 5, "params": {}}),
        ] {
            assert!(load(value).is_err());
        }
        let mut value = serde_json::json!({"version": VERSION, "params": Params::default()});
        value["params"]["wall"] = serde_json::json!(0.1);
        assert!(load(value.clone()).unwrap_err().contains("Paroi"));
        value["params"]["wall"] = serde_json::json!("1.2");
        assert!(load(value).is_err());
    }
}
