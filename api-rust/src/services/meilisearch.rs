//! Meilisearch search service
//!
//! WASM-compatible Meilisearch client using worker fetch

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Meilisearch service for full-text search
#[derive(Clone)]
pub struct SearchService {
    url: String,
    api_key: String,
}

impl SearchService {
    pub fn new(url: &str, api_key: &str) -> Self {
        Self {
            url: url.trim_end_matches('/').to_string(),
            api_key: api_key.to_string(),
        }
    }

    /// Search an index
    pub async fn search<T: for<'de> Deserialize<'de>>(
        &self,
        index: &str,
        query: &str,
        options: SearchOptions,
    ) -> Result<SearchResponse<T>, ApiError> {
        let payload = SearchRequest {
            q: query.to_string(),
            limit: options.limit,
            offset: options.offset,
            filter: options.filter,
            sort: options.sort,
            attributes_to_retrieve: options.attributes_to_retrieve,
            attributes_to_highlight: options.attributes_to_highlight,
            highlight_pre_tag: options.highlight_pre_tag,
            highlight_post_tag: options.highlight_post_tag,
        };

        self.post(&format!("/indexes/{}/search", index), &payload).await
    }

    /// Index a single document
    pub async fn index_document<T: Serialize>(
        &self,
        index: &str,
        document: &T,
    ) -> Result<TaskInfo, ApiError> {
        let docs = vec![document];
        self.post_json(&format!("/indexes/{}/documents", index), &docs).await
    }

    /// Index multiple documents
    pub async fn index_documents<T: Serialize>(
        &self,
        index: &str,
        documents: Vec<T>,
    ) -> Result<TaskInfo, ApiError> {
        self.post_json(&format!("/indexes/{}/documents", index), &documents).await
    }

    /// Delete a document
    pub async fn delete_document(&self, index: &str, id: &str) -> Result<TaskInfo, ApiError> {
        self.delete(&format!("/indexes/{}/documents/{}", index, id)).await
    }

    /// Delete multiple documents
    pub async fn delete_documents(&self, index: &str, ids: Vec<String>) -> Result<TaskInfo, ApiError> {
        self.post_json(&format!("/indexes/{}/documents/delete-batch", index), &ids).await
    }

    /// Create an index
    pub async fn create_index(&self, uid: &str, primary_key: Option<&str>) -> Result<TaskInfo, ApiError> {
        let payload = serde_json::json!({
            "uid": uid,
            "primaryKey": primary_key
        });
        self.post("/indexes", &payload).await
    }

    /// Update index settings
    pub async fn update_settings(&self, index: &str, settings: IndexSettings) -> Result<TaskInfo, ApiError> {
        self.patch(&format!("/indexes/{}/settings", index), &settings).await
    }

    /// Get task status
    pub async fn get_task(&self, task_uid: i64) -> Result<Task, ApiError> {
        self.get(&format!("/tasks/{}", task_uid)).await
    }

    /// Make a GET request using worker fetch
    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.url, path);
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_key)).ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Get);
        init.with_headers(headers);
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Meilisearch request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a POST request using worker fetch
    async fn post<T: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        self.post_json(path, body).await
    }

    /// Make a POST request with JSON body using worker fetch
    async fn post_json<T: for<'de> Deserialize<'de>, B: Serialize + ?Sized>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.url, path);
        let body_json = serde_json::to_string(body)
            .map_err(|e| ApiError::Internal(format!("Failed to serialize body: {}", e)))?;
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_key)).ok();
        headers.set("Content-Type", "application/json").ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body_json)));
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Meilisearch request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a PATCH request using worker fetch
    async fn patch<T: for<'de> Deserialize<'de>, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", self.url, path);
        let body_json = serde_json::to_string(body)
            .map_err(|e| ApiError::Internal(format!("Failed to serialize body: {}", e)))?;
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_key)).ok();
        headers.set("Content-Type", "application/json").ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Patch);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body_json)));
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Meilisearch request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Make a DELETE request using worker fetch
    async fn delete<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", self.url, path);
        
        let mut headers = worker::Headers::new();
        headers.set("Authorization", &format!("Bearer {}", self.api_key)).ok();
        
        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Delete);
        init.with_headers(headers);
        
        let request = worker::Request::new_with_init(&url, &init)
            .map_err(|e| ApiError::ExternalService(format!("Failed to create request: {:?}", e)))?;
        
        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::ExternalService(format!("Meilisearch request failed: {:?}", e)))?;
        
        self.handle_response(&mut response).await
    }

    /// Handle API response
    async fn handle_response<T: for<'de> Deserialize<'de>>(
        &self,
        response: &mut worker::Response,
    ) -> Result<T, ApiError> {
        let status = response.status_code();
        let text = response.text().await
            .map_err(|e| ApiError::ExternalService(format!("Failed to read response: {:?}", e)))?;
        
        if status >= 400 {
            let error: MeilisearchError = serde_json::from_str(&text)
                .unwrap_or(MeilisearchError {
                    message: text.clone(),
                    code: "unknown".to_string(),
                });
            return Err(ApiError::ExternalService(format!(
                "Meilisearch error [{}]: {}",
                error.code, error.message
            )));
        }

        serde_json::from_str(&text)
            .map_err(|e| ApiError::ExternalService(format!("Failed to parse Meilisearch response: {}", e)))
    }
}

#[derive(Debug, Deserialize)]
struct MeilisearchError {
    message: String,
    code: String,
}

/// Search options
#[derive(Debug, Default)]
pub struct SearchOptions {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub filter: Option<String>,
    pub sort: Option<Vec<String>>,
    pub attributes_to_retrieve: Option<Vec<String>>,
    pub attributes_to_highlight: Option<Vec<String>>,
    pub highlight_pre_tag: Option<String>,
    pub highlight_post_tag: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SearchRequest {
    q: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes_to_retrieve: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes_to_highlight: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight_pre_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight_post_tag: Option<String>,
}

/// Search response
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResponse<T> {
    pub hits: Vec<T>,
    pub query: String,
    pub processing_time_ms: i64,
    pub estimated_total_hits: Option<i64>,
    pub offset: Option<i32>,
    pub limit: Option<i32>,
}

/// Task info returned after indexing operations
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskInfo {
    pub task_uid: i64,
    pub index_uid: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub task_type: String,
}

/// Full task details
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub uid: i64,
    pub index_uid: Option<String>,
    pub status: String,
    #[serde(rename = "type")]
    pub task_type: String,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub error: Option<MeilisearchError>,
}

/// Index settings
#[derive(Debug, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct IndexSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub searchable_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterable_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable_attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_rules: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_words: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<std::collections::HashMap<String, Vec<String>>>,
}
