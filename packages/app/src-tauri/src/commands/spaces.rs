use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use super::bookmarks::BookmarkSyncData;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Space {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub space_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpaceConfig {
    pub version: String,
    pub spaces: Vec<Space>,
    pub active_space_id: String,
}

fn get_spaces_file_path() -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data)
        .join("browser-sync-cli")
        .join("spaces.json")
}

fn get_data_dir() -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data).join("browser-sync-cli")
}

fn create_default_space_config() -> SpaceConfig {
    let now = Utc::now().to_rfc3339();
    SpaceConfig {
        version: "1.0".to_string(),
        spaces: vec![
            Space {
                id: "default-chrome".to_string(),
                name: "Chrome".to_string(),
                space_type: "local".to_string(),
                api_url: None,
                api_key: None,
                browser: Some("chrome".to_string()),
                last_sync: None,
                created_at: now.clone(),
                updated_at: now.clone(),
            },
            Space {
                id: "default-edge".to_string(),
                name: "Edge".to_string(),
                space_type: "local".to_string(),
                api_url: None,
                api_key: None,
                browser: Some("edge".to_string()),
                last_sync: None,
                created_at: now.clone(),
                updated_at: now,
            },
        ],
        active_space_id: "default-chrome".to_string(),
    }
}

#[tauri::command]
pub fn get_spaces() -> Result<SpaceConfig, String> {
    let file_path = get_spaces_file_path();

    if !file_path.exists() {
        // Create default config
        let default_config = create_default_space_config();
        let dir = get_data_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir).map_err(|e| format!("Failed to create data directory: {}", e))?;
        }
        save_spaces_internal(&default_config)?;
        return Ok(default_config);
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read spaces file: {}", e))?;

    let config: SpaceConfig = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse spaces config: {}", e))?;

    Ok(config)
}

fn save_spaces_internal(config: &SpaceConfig) -> Result<(), String> {
    let file_path = get_spaces_file_path();
    let default_path = PathBuf::from(".");
    let dir = file_path.parent().unwrap_or(&default_path);

    if !dir.exists() {
        fs::create_dir_all(dir).map_err(|e| format!("Failed to create directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize spaces config: {}", e))?;

    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write spaces file: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn save_spaces(config: SpaceConfig) -> Result<(), String> {
    save_spaces_internal(&config)
}

#[tauri::command]
pub fn create_space(name: String, space_type: String, api_url: Option<String>, api_key: Option<String>, browser: Option<String>) -> Result<Space, String> {
    let mut config = get_spaces()?;

    let now = Utc::now().to_rfc3339();
    let id = Uuid::new_v4().to_string();

    let space = Space {
        id: id.clone(),
        name,
        space_type,
        api_url,
        api_key,
        browser,
        last_sync: None,
        created_at: now.clone(),
        updated_at: now,
    };

    config.spaces.push(space.clone());
    save_spaces_internal(&config)?;

    Ok(space)
}

#[tauri::command]
pub fn update_space(space: Space) -> Result<(), String> {
    let mut config = get_spaces()?;

    let index = config.spaces.iter().position(|s| s.id == space.id)
        .ok_or_else(|| format!("Space not found: {}", space.id))?;

    let mut updated_space = space;
    updated_space.updated_at = Utc::now().to_rfc3339();

    config.spaces[index] = updated_space;
    save_spaces_internal(&config)?;

    Ok(())
}

#[tauri::command]
pub fn delete_space(id: String) -> Result<(), String> {
    // Prevent deleting default spaces
    if id == "default-chrome" || id == "default-edge" {
        return Err("Cannot delete the default browser space".to_string());
    }

    let mut config = get_spaces()?;

    let initial_len = config.spaces.len();
    config.spaces.retain(|s| s.id != id);

    if config.spaces.len() == initial_len {
        return Err(format!("Space not found: {}", id));
    }

    // If the deleted space was active, switch to Chrome default
    if config.active_space_id == id {
        config.active_space_id = "default-chrome".to_string();
    }

    save_spaces_internal(&config)?;

    Ok(())
}

#[tauri::command]
pub fn set_active_space(id: String) -> Result<(), String> {
    let mut config = get_spaces()?;

    // Verify space exists
    config.spaces.iter().find(|s| s.id == id)
        .ok_or_else(|| format!("Space not found: {}", id))?;

    config.active_space_id = id;
    save_spaces_internal(&config)?;

    Ok(())
}

/// 获取空间缓存目录
fn get_space_cache_dir(space_id: &str) -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data)
        .join("browser-sync-cli")
        .join("spaces")
        .join(space_id)
}

/// 获取空间缓存文件路径
fn get_space_cache_file_path(space_id: &str) -> PathBuf {
    get_space_cache_dir(space_id).join("cache.json")
}

/// 从远程 API 获取书签数据
#[tauri::command]
pub async fn fetch_remote_bookmarks(api_url: String, api_key: Option<String>) -> Result<BookmarkSyncData, String> {
    let client = reqwest::Client::new();
    let mut request = client.get(&api_url);

    if let Some(key) = api_key {
        request = request.header("Authorization", format!("Bearer {}", key));
    }

    let response = request
        .send()
        .await
        .map_err(|e| format!("Failed to fetch remote bookmarks: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("API returned status: {}", response.status()));
    }

    let data: BookmarkSyncData = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(data)
}

/// 保存远程书签到缓存
#[tauri::command]
pub fn save_space_cache(space_id: String, data: BookmarkSyncData) -> Result<(), String> {
    let dir = get_space_cache_dir(&space_id);

    if !dir.exists() {
        fs::create_dir_all(&dir).map_err(|e| format!("Failed to create cache directory: {}", e))?;
    }

    let content = serde_json::to_string_pretty(&data)
        .map_err(|e| format!("Failed to serialize cache data: {}", e))?;

    let file_path = get_space_cache_file_path(&space_id);
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write cache file: {}", e))?;

    // 更新空间的最后同步时间
    let mut config = get_spaces()?;
    if let Some(space) = config.spaces.iter_mut().find(|s| s.id == space_id) {
        space.last_sync = Some(Utc::now().to_rfc3339());
        space.updated_at = Utc::now().to_rfc3339();
    }
    save_spaces_internal(&config)?;

    Ok(())
}

/// 获取空间缓存的书签数据
#[tauri::command]
pub fn get_space_cache(space_id: String) -> Result<Option<BookmarkSyncData>, String> {
    let file_path = get_space_cache_file_path(&space_id);

    if !file_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read cache file: {}", e))?;

    let data: BookmarkSyncData = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse cache data: {}", e))?;

    Ok(Some(data))
}