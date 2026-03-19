// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use notify::{RecommendedWatcher, Watcher};
use std::env;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;

fn get_data_file_path() -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data)
        .join("browser-sync-cli")
        .join("bookmarks.json")
}

fn watch_bookmarks_file(app_handle: tauri::AppHandle) {
    let file_path = get_data_file_path();
    let parent_dir = file_path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();

    let _watcher: RecommendedWatcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            if let Ok(event) = res {
                if event.paths.iter().any(|p| p.file_name().map(|n| n == "bookmarks.json").unwrap_or(false)) {
                    // Emit event to frontend
                    let _ = app_handle.emit_all("bookmark-changed", ());
                }
            }
        },
        notify::Config::default().with_poll_interval(Duration::from_secs(1)),
    )
    .expect("Failed to create file watcher");

    // Keep watcher alive by leaking it (for simplicity in this example)
    // In production, you'd want to manage the watcher properly
    std::mem::forget(_watcher);
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Watch for bookmark file changes
            std::thread::spawn(move || {
                let file_path = get_data_file_path();
                let parent_dir = file_path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();

                // Create directory if it doesn't exist
                if !parent_dir.exists() {
                    let _ = std::fs::create_dir_all(&parent_dir);
                }

                let running = Arc::new(AtomicBool::new(true));
                let last_modified = Arc::new(std::sync::Mutex::new(None::<std::time::SystemTime>));

                let app_h = app_handle.clone();
                let running_clone = running.clone();
                let last_mod_clone = last_modified.clone();

                std::thread::spawn(move || {
                    while running_clone.load(Ordering::Relaxed) {
                        if file_path.exists() {
                            if let Ok(metadata) = std::fs::metadata(&file_path) {
                                if let Ok(modified) = metadata.modified() {
                                    let mut last = last_mod_clone.lock().unwrap();
                                    if last.is_none() || last.unwrap() != modified {
                                        *last = Some(modified);
                                        let _ = app_h.emit_all("bookmark-changed", ());
                                    }
                                }
                            }
                        }
                        std::thread::sleep(Duration::from_secs(1));
                    }
                });
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_bookmarks,
            commands::search_bookmarks,
            commands::open_url,
            commands::export_bookmarks,
            commands::export_bookmarks_to_path,
            commands::import_bookmarks,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}