// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

use std::fs;
use std::path::PathBuf;
// use tauri::api::path::{app_data_dir, local_data_dir};
use tauri::api::path::{app_data_dir, resolve_path, BaseDirectory};

use std::sync::Mutex;
use tauri::{AppHandle, Manager};

struct AppState {
    handle: AppHandle,
}

#[tauri::command]
fn save_auth_token(state: tauri::State<'_, AppState>, token: String) -> Result<(), String> {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

    // let save_path = tauri::api::path::resolve_path(
    //     &config,
    //     package_info,
    //     &env,
    //     "auth".into(),
    //     Some(tauri::api::path::BaseDirectory::AppData),
    // )
    // .map_err(|e| e.to_string())?;

    println!("save_auth_token");

    let app_data_path = app_data_dir(&config).ok_or("Failed to get AppData directory")?;
    let save_path = app_data_path.join("auth");

    if let Some(parent) = save_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    std::fs::write(save_path, token)
        .map_err(|e| e.to_string())
        .expect("Failed to create auth file");

    Ok(())
}

fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState { handle });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_auth_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
