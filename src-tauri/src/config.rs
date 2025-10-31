use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri_plugin_android_fs::{AndroidFsExt, PrivateDir};

#[derive(Serialize, Deserialize)]
struct TomoConfig {
    api_key: String,
}

fn get_config_path(app: &AppHandle) -> PathBuf {
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

/// Get the API token from config. Returns placeholder if not set.
pub async fn get_api_token(app: &AppHandle) -> String {
    let config_path = get_config_path(app);
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
                format!("{{\"api_key\":\"{}\"}}", placeholder).into_bytes()
            });

            if let Err(_write_err) = write_file_blocking(config_path.clone(), serialized).await {
                return placeholder;
            }

            return placeholder;
        }
        Err(_) => {
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
        Err(_) => placeholder
    }
}

/// Write (or replace) the api_key field in tomoconfig.json in the app's private storage.
/// Returns `Ok(String)` with success message or `Err(String)` with an error message.
#[tauri::command]
pub async fn set_api_key(
    api_key: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let config_path = get_config_path(&app);

    let cfg = TomoConfig { api_key };
    let serialized = serde_json::to_vec_pretty(&cfg)
        .map_err(|e| format!("failed to serialize config: {}", e))?;

    write_file_blocking(config_path.clone(), serialized)
        .await
        .map_err(|e| format!("failed to write config: {}", e))?;

    Ok(format!("Set API key successfully at path {}", config_path.display()))
}