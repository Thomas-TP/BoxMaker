use serde::Serialize;
use std::sync::Mutex;
use tauri::Manager;
use velopack::{UpdateCheck, UpdateInfo, UpdateManager, sources::GithubSource};

pub const REPOSITORY: &str = "https://github.com/Thomas-TP/BoxMaker";

#[derive(Default)]
pub struct Updates(pub Mutex<Session>);

#[derive(Default)]
pub struct Session {
    available: Option<UpdateInfo>,
    downloaded: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateView {
    state: &'static str,
    message: String,
    version: Option<String>,
    notes: Option<String>,
}

fn manager() -> Result<UpdateManager, String> {
    // v0.x is a preview channel. No credentials are embedded in the application.
    UpdateManager::new(GithubSource::new(REPOSITORY, None, true), None, None)
        .map_err(|_| "Installez Boxmaker avec l’installateur Velopack pour utiliser les mises à jour intégrées.".into())
}

#[tauri::command]
pub async fn check_update(app: tauri::AppHandle) -> Result<UpdateView, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let updater = match manager() {
            Ok(value) => value,
            Err(message) => {
                return Ok(UpdateView {
                    state: "unavailable",
                    message,
                    version: None,
                    notes: None,
                });
            }
        };
        let shared = app.state::<Updates>();
        let mut session = shared.0.lock().map_err(|e| e.to_string())?;
        let result = updater
            .check_for_updates()
            .map_err(|e| format!("Vérification impossible : {e}"))?;
        session.available = None;
        session.downloaded = false;
        match result {
            UpdateCheck::UpdateAvailable(info) => {
                if info.IsDowngrade {
                    return Err("Une version plus ancienne ne sera pas installée.".into());
                }
                let view = UpdateView {
                    state: "available",
                    message: "Une nouvelle version est disponible.".into(),
                    version: Some(info.TargetFullRelease.Version.clone()),
                    notes: Some(info.TargetFullRelease.NotesMarkdown.clone()),
                };
                session.available = Some(*info);
                Ok(view)
            }
            UpdateCheck::NoUpdateAvailable => Ok(UpdateView {
                state: "current",
                message: "Vous utilisez la dernière version disponible.".into(),
                version: None,
                notes: None,
            }),
            UpdateCheck::RemoteIsEmpty => Ok(UpdateView {
                state: "empty",
                message: "Aucune version téléchargeable n’est encore publiée.".into(),
                version: None,
                notes: None,
            }),
        }
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn download_update(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let shared = app.state::<Updates>();
        let mut session = shared.0.lock().map_err(|e| e.to_string())?;
        let info = session
            .available
            .as_ref()
            .ok_or("Vérifiez d’abord les mises à jour.")?;
        manager()?
            .download_updates(info, None)
            .map_err(|e| format!("Téléchargement impossible : {e}"))?;
        session.downloaded = true;
        Ok(())
    })
    .await
    .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn install_update(app: tauri::AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        let shared = app.state::<Updates>();
        let session = shared.0.lock().map_err(|e| e.to_string())?;
        if !session.downloaded {
            return Err("Téléchargez la mise à jour avant de redémarrer.".into());
        }
        let info = session
            .available
            .as_ref()
            .ok_or("Aucune mise à jour en attente.")?;
        manager()?
            .apply_updates_and_restart(info)
            .map_err(|e| format!("Installation impossible : {e}"))
    })
    .await
    .map_err(|e| e.to_string())?
}
