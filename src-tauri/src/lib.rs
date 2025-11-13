pub mod domain;
pub mod lib {
    pub mod config;
    pub mod wanikani_client;
}

pub use lib::config::{set_api_key, get_api_key};
pub use lib::wanikani_client::{get_user, get_summary};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        // Android file-system plugin (used on Android for private/public storage access)
        .plugin(tauri_plugin_android_fs::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            lib::config::set_api_key,
            lib::config::get_api_key,
            lib::wanikani_client::get_user,
            lib::wanikani_client::get_summary,
            lib::wanikani_client::get_subjects_by_ids
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
