use reqwest::Client;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{sync::Mutex, time::Duration};
use tauri::Manager;
use tauri_plugin_updater::{Update, UpdaterExt};

const RELEASES: &str = "https://api.github.com/repos/Thomas-TP/BoxMaker/releases?per_page=100";

#[derive(Default)]
pub struct Updates(Mutex<Session>);

#[derive(Default)]
struct Session {
    available: Option<Update>,
    bytes: Option<Vec<u8>>,
    revision: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateView {
    state: &'static str,
    message: &'static str,
    version: Option<String>,
    notes: Option<String>,
}

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    draft: bool,
    prerelease: bool,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
}

fn release_tag(releases: &[Release], beta: bool) -> Option<&str> {
    releases
        .iter()
        .filter(|r| !r.draft && (beta || !r.prerelease))
        .filter(|r| {
            r.assets
                .iter()
                .any(|asset| asset.name == "mac-updater.json")
        })
        .filter_map(|r| {
            Version::parse(r.tag_name.trim_start_matches('v'))
                .ok()
                .map(|v| (v, r.tag_name.as_str()))
        })
        .filter(|(v, _)| beta || (v.major > 0 && v.pre.is_empty()))
        .max_by(|(a, _), (b, _)| a.cmp(b))
        .map(|(_, tag)| tag)
}

#[tauri::command]
pub fn update_source() -> &'static str {
    "github"
}

#[tauri::command]
pub async fn check_update(app: tauri::AppHandle, beta: bool) -> Result<UpdateView, String> {
    let revision = {
        let shared = app.state::<Updates>();
        let mut session = shared
            .0
            .lock()
            .map_err(|_| "Vérifiez d’abord les mises à jour.")?;
        session.available = None;
        session.bytes = None;
        session.revision += 1;
        session.revision
    };
    let client = Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent("Boxmaker-updater")
        .build()
        .map_err(|_| "Le service de mise à jour n’a pas pu démarrer.")?;
    let releases: Vec<Release> = client
        .get(RELEASES)
        .send()
        .await
        .and_then(reqwest::Response::error_for_status)
        .map_err(|_| "Vérification impossible : vérifiez votre connexion puis réessayez.")?
        .json()
        .await
        .map_err(|_| "Le service de mise à jour a renvoyé une réponse non valide.")?;
    let Some(tag) = release_tag(&releases, beta) else {
        return Ok(UpdateView {
            state: "empty",
            message: "Aucune version téléchargeable n’est encore publiée.",
            version: None,
            notes: None,
        });
    };
    // Parse and reformat the version to prevent a tag from changing the URL path.
    let version = Version::parse(tag.trim_start_matches('v'))
        .map_err(|_| "Version de mise à jour non valide.")?;
    let endpoint = format!(
        "https://github.com/Thomas-TP/BoxMaker/releases/download/v{version}/mac-updater.json"
    );
    let update = app
        .updater_builder()
        .endpoints(vec![
            endpoint
                .parse()
                .map_err(|_| "Version de mise à jour non valide.")?,
        ])
        .map_err(|_| "Le service de mise à jour n’a pas pu démarrer.")?
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|_| "Le service de mise à jour n’a pas pu démarrer.")?
        .check()
        .await
        .map_err(|_| "Vérification impossible : vérifiez votre connexion puis réessayez.")?;
    let shared = app.state::<Updates>();
    let mut session = shared
        .0
        .lock()
        .map_err(|_| "Vérifiez d’abord les mises à jour.")?;
    if session.revision != revision {
        return Err("Le canal a changé. Vérifiez à nouveau les mises à jour.".into());
    }
    match update {
        Some(update) => {
            let result = UpdateView {
                state: "available",
                message: "Une nouvelle version est disponible.",
                version: Some(update.version.clone()),
                notes: update.body.clone(),
            };
            session.available = Some(update);
            Ok(result)
        }
        None => Ok(UpdateView {
            state: "current",
            message: "Vous utilisez la dernière version disponible.",
            version: None,
            notes: None,
        }),
    }
}

#[tauri::command]
pub async fn download_update(app: tauri::AppHandle) -> Result<(), String> {
    let (update, revision) = {
        let shared = app.state::<Updates>();
        let session = shared
            .0
            .lock()
            .map_err(|_| "Vérifiez d’abord les mises à jour.")?;
        (
            session
                .available
                .clone()
                .ok_or("Vérifiez d’abord les mises à jour.")?,
            session.revision,
        )
    };
    // Tauri verifies the updater signature before these bytes can be installed.
    let bytes = update.download(|_, _| {}, || {}).await
        .map_err(|_| "Téléchargement impossible : vérifiez votre connexion et l’espace disque, puis réessayez.")?;
    let shared = app.state::<Updates>();
    let mut session = shared
        .0
        .lock()
        .map_err(|_| "Vérifiez d’abord les mises à jour.")?;
    if session.revision != revision {
        return Err("Le canal a changé. Vérifiez à nouveau les mises à jour.".into());
    }
    session.bytes = Some(bytes);
    Ok(())
}

#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let shared = app.state::<Updates>();
        let mut session = shared.0.lock().map_err(|_| "Vérifiez d’abord les mises à jour.")?;
        let update = session.available.as_ref().ok_or("Aucune mise à jour en attente.")?;
        let bytes = session.bytes.as_ref().ok_or("Téléchargez la mise à jour avant de redémarrer.")?;
        update.install(bytes).map_err(|_| "Installation impossible : placez Boxmaker dans Applications et vérifiez les permissions du dossier.")?;
        session.bytes = None;
        drop(session);
        app.restart();
        #[allow(unreachable_code)]
        Ok(())
    }).await.map_err(|_| "Installation impossible : réessayez ou téléchargez le DMG depuis GitHub.")?
}

#[cfg(test)]
mod tests {
    use super::*;
    fn release(tag: &str, prerelease: bool, draft: bool) -> Release {
        Release {
            tag_name: tag.into(),
            prerelease,
            draft,
            assets: vec![Asset {
                name: "mac-updater.json".into(),
            }],
        }
    }
    #[test]
    fn stable_ignores_previews_drafts_and_older_releases() {
        let releases = vec![
            release("v1.1.0-beta.1", true, false),
            release("v1.2.0", false, true),
            release("v1.0.1", false, false),
            release("v0.8.3", true, false),
            release("v1.0.0", false, false),
        ];
        assert_eq!(release_tag(&releases, false), Some("v1.0.1"));
        assert_eq!(release_tag(&releases, true), Some("v1.1.0-beta.1"));
        assert_eq!(
            release_tag(&[release("v1.1.0-beta.1", false, false)], false),
            None
        );
    }
    #[test]
    fn beta_returns_newer_stable_version_after_beta_finishes() {
        assert_eq!(
            release_tag(
                &[
                    release("v1.0.0-beta.2", true, false),
                    release("v1.0.0", false, false)
                ],
                true
            ),
            Some("v1.0.0")
        );
    }
}
