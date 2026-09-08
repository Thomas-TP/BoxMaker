#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri_plugin_dialog::DialogExt;
mod updates;

#[tauri::command]
async fn engine(request: String) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || boxmaker_core::dispatch(&request))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
async fn save_file(
    app: tauri::AppHandle,
    filename: String,
    bytes: Vec<u8>,
) -> Result<bool, String> {
    if bytes.len() > 20_000_000 || filename.contains(['/', '\\']) {
        return Err("Fichier non valide".into());
    }
    tauri::async_runtime::spawn_blocking(move || {
        let path = app
            .dialog()
            .file()
            .set_file_name(filename)
            .blocking_save_file();
        let Some(path) = path else {
            return Ok(false);
        };
        let path = path.into_path().map_err(|e| e.to_string())?;
        std::fs::write(path, bytes).map_err(|e| e.to_string())?;
        Ok(true)
    })
    .await
    .map_err(|e| e.to_string())?
}

fn main() {
    velopack::VelopackApp::build()
        .set_auto_apply_on_startup(false)
        .run();
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(updates::Updates::default())
        .invoke_handler(tauri::generate_handler![
            engine,
            save_file,
            updates::check_update,
            updates::download_update,
            updates::install_update
        ])
        .run(tauri::generate_context!())
        .expect("Impossible de démarrer Boxmaker");
}
