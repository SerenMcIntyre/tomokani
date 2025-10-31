use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use tauri::AppHandle;

use crate::lib::config;

/// Calls the WaniKani API /v2/user endpoint and returns the raw JSON response.
/// Returns error message if the API call fails or if the API key isn't set.
#[tauri::command]
pub async fn get_user(app: AppHandle) -> Result<String, String> {
    let api_token = config::get_api_token(&app).await;

    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    // Create a headers map with the authorization token
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_token))
            .map_err(|e| format!("Invalid API token format: {}", e))?,
    );

    // Create a client and make the request
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.wanikani.com/v2/user")
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to make request: {}", e))?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(format!(
            "API request failed with status: {}",
            response.status()
        ));
    }

    // Get the response text
    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(body)
}
