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

use async_channel::{unbounded, Receiver};
use async_std::task;
use futures::stream::StreamExt;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::pin::Pin;

async fn watch_directory(mut events: Pin<Box<Receiver<notify::Result<Event>>>>) {
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                println!("Event: {:?}", event);

                // Trigger your sync process here based on the event
                sync_files().await;
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

async fn sync_files() {
    // Your sync logic goes here
    println!("Syncing files...");
}

// #[async_std::main]
async fn watch_folder() -> notify::Result<()> {
    let (tx, rx) = unbounded();
    let watcher_config = Config::default()
        .with_poll_interval(std::time::Duration::from_secs(2))
        .with_compare_contents(true);
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            task::block_on(async {
                tx.send(res).await.unwrap();
            });
        },
        watcher_config,
    )?;

    watcher.watch(
        Path::new("C:/Users/alext/CommonOSFiles"),
        RecursiveMode::Recursive,
    )?;

    // Pin the receiver
    let pinned_rx = Box::pin(rx);

    // Spawn the directory watcher as an async task
    task::spawn(watch_directory(pinned_rx));

    // Keep the main function alive to listen for events
    loop {
        task::sleep(std::time::Duration::from_secs(60)).await;
    }
}

struct AppState {
    handle: AppHandle,
}

#[tauri::command]
fn save_auth_token(state: tauri::State<'_, AppState>, token: String) -> Result<(), String> {
    let handle = &state.handle;
    let config = handle.config();
    let package_info = handle.package_info();
    let env = handle.env();

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

#[async_std::main]
async fn main() {
    // tauri::Builder::default()
    //     .invoke_handler(tauri::generate_handler![greet])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");

    println!("Initialized CommonOS Files");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            app.manage(AppState { handle });

            // Spawn the folder watcher task
            task::spawn(async {
                println!("Starting Folder Watch...");

                if let Err(e) = watch_folder().await {
                    println!("Error watching folder: {:?}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_auth_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
