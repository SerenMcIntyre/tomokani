use tauri_plugin_http::reqwest::{self, header::{AUTHORIZATION, HeaderMap, HeaderValue}};
use tauri::AppHandle;
use std::sync::Arc;
use serde_json;

use super::config;
use crate::domain::api_responses::{ApiResponse, Assignment, ReviewPayload, Subject, Summary, User};

const API_BASE_URL: &str = "https://api.wanikani.com/v2";

/// A reusable WaniKani API client that maintains the base URL and authentication.
pub struct WaniKaniClient {
    client: reqwest::Client,
    base_url: String,
    api_token: Arc<String>,
}

impl WaniKaniClient {
    /// Create a new WaniKani client with the given API token
    pub fn new(api_token: String) -> Self {
        Self {
            client: reqwest::Client::new(),
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
    /// Make a POST request with a JSON body
    pub async fn post(&self, endpoint: &str, body: String) -> Result<String, String> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut headers = self.create_auth_headers()?;
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        println!("POST {} with body: {}", endpoint, body);
        let response = self.client
            .post(url)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| format!("Failed to make request: {}", e))?;

        let status = response.status();
        let resp_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        // Log status and response body
        println!("POST {} returned status: {}", endpoint, status);
        println!("POST {} response body: {}", endpoint, resp_text);

        if !status.is_success() {
            return Err(format!(
                "API request failed with status: {} and body: {}",
                status, resp_text
            ));
        }

        Ok(resp_text)
    }

    /// Make a PUT request with a JSON body
    pub async fn put(&self, endpoint: &str, body: String) -> Result<String, String> {
        let url = format!("{}{}", self.base_url, endpoint);
        let headers = self.create_auth_headers()?;

        let response = self.client
            .put(url)
            .headers(headers)
            .body(body)
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
}

/// Creates a WaniKani client instance and returns user information
#[tauri::command]
pub async fn get_user(app: AppHandle) -> Result<User, String> {
    let api_token = config::get_api_key(app.clone()).await;
    
    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    let client = WaniKaniClient::new(api_token);
    let response = client.get("/user").await?;
    
    let api_response: ApiResponse<User> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse user response: {}", e))?;
        
    Ok(api_response.data)
}

/// Retrieves the WaniKani summary information
#[tauri::command]
pub async fn get_summary(app: AppHandle) -> Result<Summary, String> {
    let api_token = config::get_api_key(app.clone()).await;
    
    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    let client = WaniKaniClient::new(api_token);
    let response = client.get("/summary").await?;

    let api_response: ApiResponse<Summary> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse summary response: {}", e))?;

    Ok(api_response.data)
}

/// Fetch subjects by IDs and return the parsed Subject objects
#[tauri::command]
pub async fn get_subjects_by_ids(app: AppHandle, ids: Vec<u32>) -> Result<Vec<Subject>, String> {
    let api_token = config::get_api_key(app.clone()).await;

    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    if ids.is_empty() {
        return Ok(Vec::new());
    }

    let client = WaniKaniClient::new(api_token);
    // WaniKani API accepts comma-separated ids: /subjects?ids=1,2,3
    let ids_query = ids.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    let endpoint = format!("/subjects?ids={}", ids_query);

    let response = client.get(&endpoint).await?;

    let api_response: ApiResponse<Vec<Subject>> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse subjects response: {} from object of json {}", e, &response))?;

    Ok(api_response.data)
}

#[tauri::command]
pub async fn get_assignments(app: AppHandle) -> Result<Vec<Assignment>, String> {
    let api_token = config::get_api_key(app.clone()).await;

    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    let client = WaniKaniClient::new(api_token);
    let response = client.get("/assignments").await?;

    let api_response: ApiResponse<Vec<Assignment>> = serde_json::from_str(&response)
        .map_err(|e| format!("Failed to parse assignments response: {}", e))?;

    Ok(api_response.data)
}

#[tauri::command]
pub async fn submit_review(app: AppHandle, review: ReviewPayload) -> Result<(), String> {
    let api_token = config::get_api_key(app.clone()).await;

    if api_token == "REPLACE_ME_API_KEY" {
        return Err("API key not configured. Please set your WaniKani API key first.".to_string());
    }

    let client = WaniKaniClient::new(api_token);
    let body = serde_json::to_string(&review)
        .map_err(|e| format!("Failed to serialize review payload: {}", e))?;

    // Log the JSON body before sending
    println!("Submitting review payload: {}", body);

    client.post("/reviews", body).await?;

    Ok(())
}