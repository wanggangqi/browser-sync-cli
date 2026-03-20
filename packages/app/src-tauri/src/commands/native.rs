use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionConfig {
    pub chrome_extension_id: Option<String>,
    pub edge_extension_id: Option<String>,
}

fn get_data_dir() -> PathBuf {
    let app_data = std::env::var("LOCALAPPDATA")
        .or_else(|_| std::env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());
    PathBuf::from(app_data).join("browser-sync-cli")
}

fn get_manifest_path() -> PathBuf {
    get_data_dir().join("com.browser_sync.cli.json")
}

#[tauri::command]
pub fn get_extension_config() -> Result<ExtensionConfig, String> {
    let manifest_path = get_manifest_path();

    if !manifest_path.exists() {
        return Ok(ExtensionConfig {
            chrome_extension_id: None,
            edge_extension_id: None,
        });
    }

    let content = fs::read_to_string(&manifest_path)
        .map_err(|e| format!("Failed to read manifest: {}", e))?;

    let manifest: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse manifest: {}", e))?;

    let allowed_origins = manifest.get("allowed_origins")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    let mut chrome_extension_id = None;
    let mut edge_extension_id = None;

    for origin in allowed_origins {
        if let Some(origin_str) = origin.as_str() {
            if origin_str.contains("chrome-extension://") {
                // Extract extension ID from "chrome-extension://xxxxx/"
                let id = origin_str
                    .replace("chrome-extension://", "")
                    .trim_end_matches('/')
                    .to_string();

                // We can't distinguish Chrome vs Edge by the origin format
                // Assume first one is Chrome, second is Edge
                if chrome_extension_id.is_none() {
                    chrome_extension_id = Some(id);
                } else {
                    edge_extension_id = Some(id);
                }
            }
        }
    }

    Ok(ExtensionConfig {
        chrome_extension_id,
        edge_extension_id,
    })
}

#[tauri::command]
pub fn save_extension_config(config: ExtensionConfig) -> Result<(), String> {
    let manifest_path = get_manifest_path();
    let data_dir = get_data_dir();

    // Create directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    // Read existing manifest or create new one
    let mut manifest = if manifest_path.exists() {
        let content = fs::read_to_string(&manifest_path)
            .map_err(|e| format!("Failed to read manifest: {}", e))?;
        serde_json::from_str::<serde_json::Value>(&content)
            .unwrap_or_else(|_| serde_json::json!({}))
    } else {
        serde_json::json!({
            "name": "com.browser_sync.cli",
            "description": "Browser Sync CLI Native Messaging Host",
            "type": "stdio"
        })
    };

    // Build allowed_origins array
    let mut allowed_origins: Vec<String> = Vec::new();

    if let Some(id) = config.chrome_extension_id {
        if !id.is_empty() {
            allowed_origins.push(format!("chrome-extension://{}/", id));
        }
    }

    if let Some(id) = config.edge_extension_id {
        if !id.is_empty() {
            allowed_origins.push(format!("chrome-extension://{}/", id));
        }
    }

    // Update manifest
    if let Some(obj) = manifest.as_object_mut() {
        obj.insert("allowed_origins".to_string(), serde_json::json!(allowed_origins));
    }

    // Write manifest
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

    fs::write(&manifest_path, manifest_json)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;

    Ok(())
}