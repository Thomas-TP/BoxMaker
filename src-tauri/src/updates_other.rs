use serde::Serialize;

#[derive(Default)]
pub struct Updates;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateView {
    state: &'static str,
    message: &'static str,
    version: Option<String>,
    notes: Option<String>,
}

#[tauri::command]
pub async fn check_update(beta: bool) -> Result<UpdateView, String> {
    let _ = beta;
    Ok(UpdateView {
        state: "unavailable",
        message: "Sur macOS, téléchargez les nouvelles versions depuis les releases GitHub. Cette version ne dispose pas de mise à jour intégrée.",
        version: None,
        notes: None,
    })
}

#[tauri::command]
pub fn update_source() -> &'static str {
    "unavailable"
}

#[tauri::command]
pub async fn download_update() -> Result<(), String> {
    Err("Téléchargez les nouvelles versions depuis les releases GitHub.".into())
}

#[tauri::command]
pub async fn install_update() -> Result<(), String> {
    Err("Téléchargez les nouvelles versions depuis les releases GitHub.".into())
}
