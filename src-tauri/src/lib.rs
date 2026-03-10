#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter, Manager};

#[derive(Default)]
struct AppState {
    opened_image_sources: Arc<Mutex<Vec<String>>>,
}

#[tauri::command]
fn window_ready(app: AppHandle) {
    app.emit("got-files", 2).unwrap();
    #[cfg(target_os = "macos")]
    {
        let state = app.state::<AppState>();
        let opened_image_sources = state.opened_image_sources.lock().unwrap();

        // Remove file:// prefix from all URLs
        let formatted_sources: Vec<String> = opened_image_sources
            .iter()
            .map(|url| url.replace("file://", ""))
            .collect();

        app.emit("got-files", formatted_sources)
            .unwrap_or_else(|err| eprintln!("Emit error: {:?}", err));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState::default();

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_cli::init())
        .invoke_handler(tauri::generate_handler![window_ready])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = event {
                let state = app.state::<AppState>();
                let mut opened_image_sources = state.opened_image_sources.lock().unwrap();
                *opened_image_sources = urls.iter().map(|x| x.to_string()).collect();
            }
        });
}
