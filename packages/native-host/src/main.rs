use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BookmarkNode {
    id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_added: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    date_group_modified: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<BookmarkNode>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookmarkSyncData {
    bookmarks: Vec<BookmarkNode>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
enum BrowserType {
    Chrome,
    Edge,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BrowserBookmarks {
    bookmarks: Vec<BookmarkNode>,
    last_sync: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MultiBrowserData {
    chrome: Option<BrowserBookmarks>,
    edge: Option<BrowserBookmarks>,
}

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    #[serde(rename = "type")]
    msg_type: String,
    data: serde_json::Value,
    #[serde(default)]
    browser: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn get_data_file_path() -> PathBuf {
    // On Windows, use %LOCALAPPDATA%\browser-sync-cli\
    let app_data = env::var("LOCALAPPDATA")
        .or_else(|_| env::var("APPDATA"))
        .unwrap_or_else(|_| ".".to_string());

    let data_dir = PathBuf::from(app_data).join("browser-sync-cli");

    // Create directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).ok();
    }

    data_dir.join("bookmarks.json")
}

fn load_multi_browser_data() -> MultiBrowserData {
    let file_path = get_data_file_path();

    if !file_path.exists() {
        return MultiBrowserData {
            chrome: None,
            edge: None,
        };
    }

    match fs::read_to_string(&file_path) {
        Ok(content) => {
            // Try to parse as multi-browser format first
            if let Ok(data) = serde_json::from_str::<MultiBrowserData>(&content) {
                return data;
            }
            // Try to parse as legacy single-browser format
            if let Ok(legacy_data) = serde_json::from_str::<BookmarkSyncData>(&content) {
                return MultiBrowserData {
                    chrome: Some(BrowserBookmarks {
                        bookmarks: legacy_data.bookmarks,
                        last_sync: None,
                    }),
                    edge: None,
                };
            }
        }
        Err(_) => {}
    }

    MultiBrowserData {
        chrome: None,
        edge: None,
    }
}

fn save_multi_browser_data(data: &MultiBrowserData) -> Result<(), String> {
    let file_path = get_data_file_path();

    let json_content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize bookmarks: {}", e))?;

    fs::write(&file_path, json_content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(())
}

fn handle_full_sync(data: &BookmarkSyncData, browser: Option<String>) -> Response {
    let mut multi_data = load_multi_browser_data();
    let now = chrono::Utc::now().to_rfc3339();

    let browser_bookmarks = BrowserBookmarks {
        bookmarks: data.bookmarks.clone(),
        last_sync: Some(now),
    };

    match browser.as_deref() {
        Some("chrome") | None => {
            multi_data.chrome = Some(browser_bookmarks);
        }
        Some("edge") => {
            multi_data.edge = Some(browser_bookmarks);
        }
        _ => {
            // Unknown browser, default to chrome
            multi_data.chrome = Some(browser_bookmarks);
        }
    }

    match save_multi_browser_data(&multi_data) {
        Ok(_) => Response {
            success: true,
            message: Some(format!("Bookmarks saved for browser: {:?}", browser)),
            error: None,
        },
        Err(e) => Response {
            success: false,
            message: None,
            error: Some(format!("Failed to save bookmarks: {}", e)),
        },
    }
}

fn read_message() -> Option<IncomingMessage> {
    // Read message length (4 bytes, little-endian)
    let mut length_bytes = [0u8; 4];
    match io::stdin().read_exact(&mut length_bytes) {
        Ok(_) => (),
        Err(_) => return None,
    }

    let length = u32::from_le_bytes(length_bytes) as usize;

    // Read message content
    let mut message_bytes = vec![0u8; length];
    match io::stdin().read_exact(&mut message_bytes) {
        Ok(_) => (),
        Err(_) => return None,
    }

    // Parse JSON
    match serde_json::from_slice(&message_bytes) {
        Ok(msg) => Some(msg),
        Err(e) => {
            eprintln!("Failed to parse message: {}", e);
            None
        }
    }
}

fn send_response(response: &Response) {
    let json = match serde_json::to_string(response) {
        Ok(j) => j,
        Err(_) => return,
    };

    // Write response length (4 bytes, little-endian)
    let length = json.len() as u32;
    let length_bytes = length.to_le_bytes();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(&length_bytes).ok();
    handle.write_all(json.as_bytes()).ok();
    handle.flush().ok();
}

fn main() {
    // Process each incoming message
    while let Some(message) = read_message() {
        let browser = message.browser.clone();
        let response = match message.msg_type.as_str() {
            "full_sync" => {
                // Parse the data as BookmarkSyncData
                match serde_json::from_value::<BookmarkSyncData>(message.data) {
                    Ok(sync_data) => handle_full_sync(&sync_data, browser),
                    Err(e) => Response {
                        success: false,
                        message: None,
                        error: Some(format!("Invalid sync data: {}", e)),
                    },
                }
            }
            "incremental_update" => {
                // For incremental updates, we still do a full write for simplicity
                Response {
                    success: true,
                    message: Some("Incremental update received".to_string()),
                    error: None,
                }
            }
            "ping" => Response {
                success: true,
                message: Some("pong".to_string()),
                error: None,
            },
            _ => Response {
                success: false,
                message: None,
                error: Some(format!("Unknown message type: {}", message.msg_type)),
            },
        };

        send_response(&response);
    }
}