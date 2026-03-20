// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::env;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::Manager;

#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

fn get_data_dir() -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data).join("browser-sync-cli")
}

fn get_data_file_path() -> PathBuf {
    get_data_dir().join("bookmarks.json")
}

#[cfg(target_os = "windows")]
fn register_native_messaging_host(exe_path: &PathBuf) -> Result<(), String> {
    let host_id = "com.browser_sync.cli";
    let manifest_dir = get_data_dir();
    let manifest_path = manifest_dir.join(format!("{}.json", host_id));

    // Create manifest directory if it doesn't exist
    if !manifest_dir.exists() {
        std::fs::create_dir_all(&manifest_dir)
            .map_err(|e| format!("Failed to create manifest directory: {}", e))?;
    }

    // Create manifest JSON
    let manifest = serde_json::json!({
        "name": host_id,
        "description": "Browser Sync CLI Native Messaging Host",
        "path": exe_path.to_string_lossy().replace("\\", "\\\\"),
        "type": "stdio",
        "allowed_origins": []
    });

    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

    std::fs::write(&manifest_path, manifest_json)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;

    // Register in Windows Registry
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    // Register for Chrome
    let chrome_path = r"Software\Google\Chrome\NativeMessagingHosts";
    if let Ok(chrome_key) = hkcu.create_subkey(chrome_path) {
        let (chrome_reg_key, _) = chrome_key;
        chrome_reg_key
            .set_value(host_id, &manifest_path.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to register for Chrome: {}", e))?;
    }

    // Register for Edge
    let edge_path = r"Software\Microsoft\Edge\NativeMessagingHosts";
    if let Ok(edge_key) = hkcu.create_subkey(edge_path) {
        let (edge_reg_key, _) = edge_key;
        edge_reg_key
            .set_value(host_id, &manifest_path.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to register for Edge: {}", e))?;
    }

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn register_native_messaging_host(_exe_path: &PathBuf) -> Result<(), String> {
    // TODO: Implement for macOS/Linux if needed
    Ok(())
}

fn get_native_host_path() -> PathBuf {
    // First, check if native host is in the same directory as the app
    if let Ok(exe_path) = env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let bundled_path = exe_dir.join("browser-sync-native-host.exe");
            if bundled_path.exists() {
                return bundled_path;
            }
        }
    }

    // Fallback to LOCALAPPDATA
    get_data_dir().join("browser-sync-native-host.exe")
}

fn main() {
    // Register native messaging host on startup
    let native_host_path = get_native_host_path();
    if let Err(e) = register_native_messaging_host(&native_host_path) {
        eprintln!("Failed to register native messaging host: {}", e);
    }

    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();

            // Watch for bookmark file changes
            std::thread::spawn(move || {
                let file_path = get_data_file_path();

                // Create directory if it doesn't exist
                let parent_dir = file_path.parent().unwrap_or(&PathBuf::from(".")).to_path_buf();
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
            commands::bookmarks::get_bookmarks,
            commands::bookmarks::search_bookmarks,
            commands::bookmarks::open_url,
            commands::bookmarks::export_bookmarks,
            commands::bookmarks::export_bookmarks_to_path,
            commands::bookmarks::import_bookmarks,
            commands::spaces::get_spaces,
            commands::spaces::save_spaces,
            commands::spaces::create_space,
            commands::spaces::update_space,
            commands::spaces::delete_space,
            commands::spaces::set_active_space,
            commands::spaces::fetch_remote_bookmarks,
            commands::spaces::save_space_cache,
            commands::spaces::get_space_cache,
            commands::native::get_extension_config,
            commands::native::save_extension_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}