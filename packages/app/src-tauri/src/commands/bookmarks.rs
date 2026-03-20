use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkNode {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BookmarkNode>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookmarkSyncData {
    pub bookmarks: Vec<BookmarkNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BrowserBookmarks {
    pub bookmarks: Vec<BookmarkNode>,
    pub last_sync: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiBrowserData {
    pub chrome: Option<BrowserBookmarks>,
    pub edge: Option<BrowserBookmarks>,
}

/// 浏览器类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserType {
    Chrome,
    Edge,
    All,
}

fn get_data_file_path() -> PathBuf {
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(app_data)
        .join("browser-sync-cli")
        .join("bookmarks.json")
}

/// 加载多浏览器数据，兼容旧格式
fn load_bookmarks_data() -> MultiBrowserData {
    let file_path = get_data_file_path();

    if !file_path.exists() {
        return MultiBrowserData {
            chrome: None,
            edge: None,
        };
    }

    let content = match fs::read_to_string(&file_path) {
        Ok(c) => c,
        Err(_) => return MultiBrowserData { chrome: None, edge: None },
    };

    // 尝试解析为多浏览器格式
    if let Ok(data) = serde_json::from_str::<MultiBrowserData>(&content) {
        return data;
    }

    // 尝试解析为旧格式（兼容）
    if let Ok(legacy) = serde_json::from_str::<BookmarkSyncData>(&content) {
        return MultiBrowserData {
            chrome: Some(BrowserBookmarks {
                bookmarks: legacy.bookmarks,
                last_sync: None,
            }),
            edge: None,
        };
    }

    MultiBrowserData {
        chrome: None,
        edge: None,
    }
}

/// 获取指定浏览器的书签
#[tauri::command]
pub fn get_bookmarks(browser: Option<String>) -> Result<BookmarkSyncData, String> {
    let data = load_bookmarks_data();

    let result = match browser.as_deref() {
        Some("chrome") => data.chrome,
        Some("edge") => data.edge,
        _ => data.chrome.or(data.edge), // 默认返回 chrome，如果没有则返回 edge
    };

    Ok(BookmarkSyncData {
        bookmarks: result.map(|b| b.bookmarks).unwrap_or_default(),
    })
}

#[tauri::command]
pub fn search_bookmarks(query: String, browser: Option<String>) -> Result<Vec<BookmarkNode>, String> {
    let data = load_bookmarks_data();

    let result = match browser.as_deref() {
        Some("chrome") => data.chrome,
        Some("edge") => data.edge,
        _ => data.chrome.or(data.edge),
    };

    let bookmarks = result.map(|b| b.bookmarks).unwrap_or_default();

    fn search_in_tree(nodes: &[BookmarkNode], query: &str, results: &mut Vec<BookmarkNode>) {
        for node in nodes {
            if node.url.is_some() {
                let title_match = node.title.to_lowercase().contains(&query.to_lowercase());
                let url_match = node.url.as_ref()
                    .map(|u| u.to_lowercase().contains(&query.to_lowercase()))
                    .unwrap_or(false);

                if title_match || url_match {
                    results.push(node.clone());
                }
            }

            if let Some(children) = &node.children {
                search_in_tree(children, query, results);
            }
        }
    }

    let mut results = Vec::new();
    search_in_tree(&bookmarks, &query, &mut results);

    Ok(results)
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", &url])
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&url)
            .spawn()
            .map_err(|e| format!("Failed to open URL: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn export_bookmarks() -> Result<String, String> {
    let source_path = get_data_file_path();

    if !source_path.exists() {
        return Err("No bookmarks file exists".to_string());
    }

    // Generate export path with timestamp
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let export_path = get_data_file_path()
        .parent()
        .unwrap_or(&PathBuf::from("."))
        .join(format!("bookmarks_export_{}.json", timestamp));

    fs::copy(&source_path, &export_path)
        .map_err(|e| format!("Failed to export bookmarks: {}", e))?;

    Ok(export_path.to_string_lossy().to_string())
}

#[tauri::command]
pub fn export_bookmarks_to_path(path: String) -> Result<(), String> {
    let source_path = get_data_file_path();

    if !source_path.exists() {
        return Err("No bookmarks file exists".to_string());
    }

    let export_path = PathBuf::from(&path);

    fs::copy(&source_path, &export_path)
        .map_err(|e| format!("Failed to export bookmarks: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn import_bookmarks(path: String) -> Result<(), String> {
    let source_path = PathBuf::from(&path);

    if !source_path.exists() {
        return Err(format!("Import file not found: {}", path));
    }

    // Validate the import file
    let content = fs::read_to_string(&source_path)
        .map_err(|e| format!("Failed to read import file: {}", e))?;

    let _: BookmarkSyncData = serde_json::from_str(&content)
        .map_err(|e| format!("Invalid bookmark file format: {}", e))?;

    // Copy to the data file
    let dest_path = get_data_file_path();
    fs::copy(&source_path, &dest_path)
        .map_err(|e| format!("Failed to import bookmarks: {}", e))?;

    Ok(())
}