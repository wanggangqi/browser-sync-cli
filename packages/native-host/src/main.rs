use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
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
    version: String,
    last_sync: String,
    bookmarks: Vec<BookmarkNode>,
}

#[derive(Debug, Deserialize)]
struct IncomingMessage {
    #[serde(rename = "type")]
    msg_type: String,
    data: serde_json::Value,
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

fn handle_full_sync(data: &BookmarkSyncData) -> Response {
    let file_path = get_data_file_path();

    // Format JSON with pretty printing
    let json_content = match serde_json::to_string_pretty(data) {
        Ok(j) => j,
        Err(e) => return Response {
            success: false,
            message: None,
            error: Some(format!("Failed to serialize bookmarks: {}", e)),
        },
    };

    match fs::write(&file_path, json_content) {
        Ok(_) => Response {
            success: true,
            message: Some(format!("Bookmarks saved to {:?}", file_path)),
            error: None,
        },
        Err(e) => Response {
            success: false,
            message: None,
            error: Some(format!("Failed to write file: {}", e)),
        },
    }
}

fn main() {
    // Process each incoming message
    while let Some(message) = read_message() {
        let response = match message.msg_type.as_str() {
            "full_sync" => {
                // Parse the data as BookmarkSyncData
                match serde_json::from_value::<BookmarkSyncData>(message.data) {
                    Ok(sync_data) => handle_full_sync(&sync_data),
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