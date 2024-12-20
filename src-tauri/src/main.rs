// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

mod gql;

use lazy_static::lazy_static;
use notify::EventKind;
use std::fs;
use tauri::api::path::document_dir;
// use tauri::api::path::{app_data_dir, local_data_dir};
use tauri::api::path::{app_data_dir, resolve_path, BaseDirectory};

use std::sync::Arc;
use std::sync::Mutex;
use tauri::{App, AppHandle, Manager};

use async_channel::{unbounded, Receiver};
use futures::stream::StreamExt;
use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use tokio::time::{sleep, Duration};

use crate::gql::uploadSync::upload_sync_one_file;

async fn watch_directory(
    mut events: Pin<Box<Receiver<notify::Result<Event>>>>,
    auth_token: String,
    sync_dir: PathBuf,
) {
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                println!("Sync Folder Event: {:?}", event);

                if (event.kind == EventKind::Create(notify::event::CreateKind::Any)) {
                    // Trigger your sync process here based on the event
                    // sync_files(auth_token.clone(), event.paths, &sync_dir).await;

                    // Filter out directories and only process files
                    let file_paths: Vec<PathBuf> = event
                        .paths
                        .iter()
                        .filter_map(|path| {
                            if fs::metadata(path)
                                .map(|metadata| metadata.is_file())
                                .unwrap_or(false)
                            {
                                Some(path.clone())
                            } else {
                                None
                            }
                        })
                        .collect();

                    // Trigger your sync process here based on the filtered file paths
                    if !file_paths.is_empty() {
                        println!("Create and sync triggered...");

                        sync_files(auth_token.clone(), file_paths, &sync_dir).await;
                    }
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

async fn sync_files(auth_token: String, paths: Vec<PathBuf>, sync_dir: &Path) {
    for path in paths {
        if let Ok(relative_path) = path.strip_prefix(sync_dir) {
            // Extract the directory path and file name
            if let Some(directory) = relative_path.parent() {
                if let Some(file_name) = relative_path.file_name() {
                    let directory_str = directory.to_string_lossy().into_owned();
                    let file_name_str = file_name.to_string_lossy().into_owned();

                    // Log the relative directory and file name
                    // println!("Directory: {}, File name: {}", directory_str, file_name_str);

                    // Read the file content
                    let file_content = std::fs::read(&path)
                        .map(base64::encode)
                        .unwrap_or_else(|_| "".to_string());

                    // Log the file content
                    println!("Syncing file: {:?} from {:?}", file_name_str, directory_str);

                    // logic to upload `file_content` to S3
                    let _ = upload_sync_one_file(
                        auth_token.clone(),
                        file_name_str,
                        directory_str,
                        file_content,
                    )
                    .await;
                } else {
                    println!("Could not extract file name from path: {:?}", relative_path);
                }
            } else {
                println!("Could not extract directory from path: {:?}", relative_path);
            }
        } else {
            println!(
                "Path {:?} is not within base directory {:?}",
                path, sync_dir
            );
        }
    }
}

use tokio::task::JoinHandle;

// Global state to track the current watcher task
lazy_static! {
    static ref CURRENT_WATCHER: Arc<Mutex<Option<JoinHandle<()>>>> = Arc::new(Mutex::new(None));
}

fn spawn_watcher(auth_token: String) {
    // Get the sync directory
    let sync_dir = document_dir()
        .expect("Couldn't get Documents directory")
        .join("CommonOS");

    // Clone the CURRENT_WATCHER reference for use in async block
    let watcher_handle = CURRENT_WATCHER.clone();

    // Spawn the new watcher task
    tokio::task::spawn(async move {
        // Cancel the previous task if it exists
        let mut current = watcher_handle.lock().expect("Couldn't get watcher handle");
        if let Some(handle) = current.take() {
            println!("Cancelling previous folder watcher...");
            handle.abort();
        }

        // Create and store the new task
        let new_handle = tokio::task::spawn(async move {
            println!("Starting Folder Watch... {:?}", sync_dir);
            if let Err(e) = watch_folder(auth_token, sync_dir).await {
                println!("Error watching folder: {:?}", e);
            }
        });

        *current = Some(new_handle);
    });
}

async fn watch_folder(auth_token: String, sync_dir: PathBuf) -> notify::Result<()> {
    let (tx, rx) = unbounded();
    let watcher_config = Config::default()
        .with_poll_interval(Duration::from_secs(2))
        .with_compare_contents(true);
    let mut watcher: RecommendedWatcher = Watcher::new(
        move |res| {
            tokio::task::block_in_place(|| {
                tx.send_blocking(res).unwrap();
            });
        },
        watcher_config,
    )?;

    watcher.watch(&sync_dir, RecursiveMode::Recursive)?;

    // Pin the receiver
    let pinned_rx = Box::pin(rx);

    // Store the watch_directory task handle
    let watch_handle = tokio::task::spawn(async move {
        watch_directory(pinned_rx, auth_token, sync_dir).await;
    });

    // Wait for the task to complete or be cancelled
    tokio::select! {
        _ = watch_handle => {},
        _ = tokio::signal::ctrl_c() => {
            // Clean shutdown logic here if needed
        },
    }

    Ok(())
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

    // let app_data_path = app_data_dir(&config).ok_or("Failed to get AppData directory")?;
    let sync_dir = document_dir()
        .expect("Couldn't get Documents directory")
        .join("CommonOS");
    let save_path = sync_dir.join("auth");

    if let Some(parent) = save_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    std::fs::write(save_path, token.clone())
        .map_err(|e| e.to_string())
        .expect("Failed to create auth file");

    // spawns on either startup or on save token
    spawn_watcher(token.clone());

    Ok(())
}

fn read_auth_token(config: &Arc<tauri::Config>) -> String {
    println!("read_auth_token");

    // let app_data_path = app_data_dir(config)
    //     .ok_or("Failed to get AppData directory (1)")
    //     .expect("Failed to get AppData directory (2)");
    let sync_dir = document_dir()
        .expect("Couldn't get Documents directory")
        .join("CommonOS");
    let read_path = sync_dir.join("auth");

    // pull String content from read_path
    let auth_data =
        String::from_utf8_lossy(&std::fs::read(read_path).unwrap_or_default()).to_string();

    auth_data
}

#[tokio::main]
async fn main() {
    println!("Initialized CommonOS Files");

    tauri::Builder::default()
        .setup(|app| {
            let handle = app.handle();
            let config = handle.config();

            app.manage(AppState { handle });

            // this gets stale
            let auth_token = read_auth_token(&config);

            if (auth_token.len() > 0) {
                // spawns on either startup or on save token
                spawn_watcher(auth_token);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![save_auth_token])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
