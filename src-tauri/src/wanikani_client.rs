use tauri_plugin_http::reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION}};
use tauri::AppHandle;
use std::sync::Arc;

use crate::config;

const API_BASE_URL: &str = "https://api.wanikani.com/v2";

/// A reusable WaniKani API client that maintains the base URL and authentication.
pub struct WaniKaniClient {
    client: Client,
    base_url: String,
    api_token: Arc<String>,
}

impl WaniKaniClient {
    /// Create a new WaniKani client with the given API token
    pub fn new(api_token: String) -> Self {
        Self {
            client: Client::new(),
            base_url: API_BASE_URL.to_string(),
            api_token: Arc::new(api_token),
        }
    }

    /// Create the authorization headers for API requests
    fn create_auth_headers(&self) -> Result<HeaderMap, String> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.api_token))
                .map_err(|e| format!("Invalid API token format: {}", e))?,
        );
        Ok(headers)
    }

    /// Make a GET request to a WaniKani API endpoint
    pub async fn get(&self, endpoint: &str) -> Result<String, String> {
        let url = format!("{}{}", self.base_url, endpoint);
        let headers = self.create_auth_headers()?;

        let response = self.client
            .get(url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| format!("Failed to make request: {}", e))?;

        if !response.status().is_success() {
            return Err(format!(
                "API request failed with status: {}",
                response.status()
            ));
        }

        response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))
    }

    /// Get user information from the /user endpoint
    pub async fn get_user(&self) -> Result<String, String> {
        self.get("/user").await
    }
}

/// Creates a WaniKani client instance and returns user information
#[tauri::command]
pub async fn get_user(app: AppHandle) -> Result<String, String> {
    let api_token = config::get_api_token(&app).await;
    
    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    let client = WaniKaniClient::new(api_token);
    client.get_user().await
}