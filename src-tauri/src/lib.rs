// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Define the LevelData struct
#[derive(serde::Serialize)]
struct LevelData {
    level_number: u32,
    description: String,
    difficulty: String,
}

#[tauri::command]
fn retrieve_level_data() -> LevelData {
    // In a real application, this data might be fetched from a database or file
    LevelData {
        level_number: 1,
        description: "Welcome to Level 1".to_string(),
        difficulty: "Easy".to_string(),
    }
}