// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// Android-only: import the Android FS plugin types unconditionally since the
// app targets Android only.
use tauri_plugin_android_fs::{AndroidFsExt, PrivateDir};

#[derive(Serialize, Deserialize)]
struct TomoConfig {
    api_key: String,
}


fn get_path(app: tauri::AppHandle) -> PathBuf {
    let fsapi = app.android_fs();
    let data_dir = fsapi.private_storage().resolve_path(PrivateDir::Data)
        .expect("failed to resolve android private data dir");

    data_dir.join("tomoconfig.json")
}

/// Read a file from disk using a blocking thread. Returns bytes or an io::Error.
async fn read_file_blocking(path: PathBuf) -> std::io::Result<Vec<u8>> {
    tauri::async_runtime::spawn_blocking(move || std::fs::read(path))
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("join error: {}", e)))?
}

/// Write bytes to a file (creating parent directories). Runs in a blocking thread.
async fn write_file_blocking(path: PathBuf, bytes: Vec<u8>) -> std::io::Result<()> {
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, bytes)
    })
    .await
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("join error: {}", e)))?
}

/// Read `tomoconfig.json` from Android private storage, ensure `api_key` exists
/// (set placeholder if empty), and return the `api_key` string. Uses blocking threads
/// for filesystem IO so the async runtime isn't blocked.
#[tauri::command]
async fn retrieve_level_data(app: tauri::AppHandle) -> String {
    let config_path = get_path(app.clone());

    // Preferred approach for Android private storage: use std::fs (in a blocking thread).
    // Read the file if it exists, otherwise create a placeholder JSON with the placeholder
    // API key and return that.
    let placeholder = "REPLACE_ME_API_KEY".to_string();

    // Try to read the file
    let bytes = match read_file_blocking(config_path.clone()).await {
        Ok(b) => b,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            // Create a default config with placeholder key and write it back.
            let cfg = TomoConfig {
                api_key: placeholder.clone(),
            };
            let serialized = serde_json::to_vec_pretty(&cfg).unwrap_or_else(|_| {
                // Fallback minimal JSON
                format!("{{\"api_key\":\"{}\"}}", placeholder).into_bytes()
            });

            if let Err(_write_err) = write_file_blocking(config_path.clone(), serialized).await {
                // If write fails, return placeholder anyway (we don't want to panic in command)
                return placeholder;
            }

            return placeholder;
        }
        Err(_) => {
            // On other IO errors return placeholder to keep UI responsive.
            return placeholder;
        }
    };

    // Parse the JSON and return api_key if available
    match serde_json::from_slice::<TomoConfig>(&bytes) {
        Ok(cfg) => {
            if cfg.api_key.trim().is_empty() {
                placeholder
            } else {
                cfg.api_key
            }
        }
        Err(_) => {
            // If parsing fails, return placeholder
            placeholder
        }
    }
}

/// Write (or replace) the api_key field in tomoconfig.json in the app's private storage.
/// Returns `Ok(String)` with success message or `Err(String)` with an error message.
#[tauri::command]
async fn set_api_key(
    api_key: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let config_path = get_path(app.clone());

    let cfg = TomoConfig { api_key };
    let serialized = serde_json::to_vec_pretty(&cfg)
        .map_err(|e| format!("failed to serialize config: {}", e))?;

    write_file_blocking(config_path.clone(), serialized)
        .await
        .map_err(|e| format!("failed to write config: {}", e))?;



    Ok(format!("Set API key successfully at path {}", config_path.display()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Android file-system plugin (used on Android for private/public storage access)
        .plugin(tauri_plugin_android_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![retrieve_level_data, set_api_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
