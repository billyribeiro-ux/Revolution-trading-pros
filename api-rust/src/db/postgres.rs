//! Neon PostgreSQL database connection via HTTP
//! 
//! Uses Neon's serverless driver for edge-compatible database access
//! WASM-compatible using worker::Fetch

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Database client for Neon PostgreSQL
#[derive(Clone)]
pub struct Database {
    connection_string: String,
    endpoint: String,
}

#[derive(Serialize)]
struct NeonQuery {
    query: String,
    params: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct NeonResponse {
    rows: Vec<serde_json::Value>,
    #[serde(default)]
    row_count: Option<i64>,
}

#[derive(Deserialize)]
struct NeonError {
    message: String,
}

impl Database {
    /// Create a new database connection
    pub fn new(connection_string: &str) -> Self {
        let endpoint = Self::parse_endpoint(connection_string)
            .unwrap_or_else(|_| "https://neon.tech/sql".to_string());
        
        Self {
            connection_string: connection_string.to_string(),
            endpoint,
        }
    }

    /// Parse the HTTP endpoint from connection string
    fn parse_endpoint(connection_string: &str) -> Result<String, ApiError> {
        // Extract host from postgres://user:pass@host/db
        let parts: Vec<&str> = connection_string.split('@').collect();
        if parts.len() != 2 {
            return Err(ApiError::Database("Invalid connection string".to_string()));
        }
        
        let host_db: Vec<&str> = parts[1].split('/').collect();
        if host_db.is_empty() {
            return Err(ApiError::Database("Invalid connection string".to_string()));
        }
        
        let host = host_db[0];
        Ok(format!("https://{}/sql", host))
    }

    /// Execute a query and return rows
    pub async fn query<T: for<'de> Deserialize<'de>>(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<Vec<T>, ApiError> {
        let response = self.execute_query(sql, params).await?;
        
        let mut results = Vec::new();
        for (i, row) in response.rows.into_iter().enumerate() {
            match serde_json::from_value::<T>(row.clone()) {
                Ok(item) => results.push(item),
                Err(e) => {
                    worker::console_log!("Row {} deserialize error: {} - Row data: {}", i, e, row);
                    return Err(ApiError::Database(format!("Failed to deserialize row {}: {}", i, e)));
                }
            }
        }
        
        Ok(results)
    }

    /// Execute a query and return a single row
    pub async fn query_one<T: for<'de> Deserialize<'de>>(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<Option<T>, ApiError> {
        let rows: Vec<T> = self.query(sql, params).await?;
        Ok(rows.into_iter().next())
    }

    /// Execute a query without returning rows (INSERT, UPDATE, DELETE)
    pub async fn execute(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<i64, ApiError> {
        let response = self.execute_query(sql, params).await?;
        Ok(response.row_count.unwrap_or(0))
    }

    /// Internal query execution using worker::Fetch
    async fn execute_query(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<NeonResponse, ApiError> {
        let query = NeonQuery {
            query: sql.to_string(),
            params,
        };

        let body = serde_json::to_string(&query)
            .map_err(|e| ApiError::Database(format!("Failed to serialize query: {}", e)))?;

        let headers = worker::Headers::new();
        headers.set("Content-Type", "application/json").ok();
        headers.set("Neon-Connection-String", &self.connection_string).ok();

        let mut init = worker::RequestInit::new();
        init.with_method(worker::Method::Post);
        init.with_headers(headers);
        init.with_body(Some(wasm_bindgen::JsValue::from_str(&body)));

        let request = worker::Request::new_with_init(&self.endpoint, &init)
            .map_err(|e| ApiError::Database(format!("Failed to create request: {:?}", e)))?;

        let mut response = worker::Fetch::Request(request)
            .send()
            .await
            .map_err(|e| ApiError::Database(format!("HTTP error: {:?}", e)))?;

        let status = response.status_code();
        let text = response.text().await
            .map_err(|e| ApiError::Database(format!("Failed to read response: {:?}", e)))?;

        if status >= 400 {
            let error: NeonError = serde_json::from_str(&text)
                .unwrap_or(NeonError { message: text.clone() });
            return Err(ApiError::Database(error.message));
        }

        serde_json::from_str(&text)
            .map_err(|e| ApiError::Database(format!("Failed to parse response: {}", e)))
    }

    /// Begin a transaction
    pub async fn begin_transaction(&self) -> Result<Transaction, ApiError> {
        Ok(Transaction {
            db: self.clone(),
            queries: Vec::new(),
        })
    }
}

/// Transaction wrapper for batch operations
pub struct Transaction {
    db: Database,
    queries: Vec<(String, Vec<serde_json::Value>)>,
}

impl Transaction {
    /// Add a query to the transaction
    pub fn add(&mut self, sql: &str, params: Vec<serde_json::Value>) {
        self.queries.push((sql.to_string(), params));
    }

    /// Commit the transaction
    pub async fn commit(self) -> Result<(), ApiError> {
        // Execute all queries in sequence
        // Note: For true transactions, use Neon's transaction API
        for (sql, params) in self.queries {
            self.db.execute(&sql, params).await?;
        }
        Ok(())
    }

    /// Rollback (no-op for HTTP queries, but included for API completeness)
    pub fn rollback(self) {
        // HTTP queries are auto-committed, so this is a no-op
        // For true rollback support, use Neon's transaction API
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_endpoint_extraction() {
        let db = Database::new("postgres://user:pass@ep-cool-name-123456.us-east-2.aws.neon.tech/neondb");
        let endpoint = db.get_http_endpoint().unwrap();
        assert_eq!(endpoint, "https://ep-cool-name-123456.us-east-2.aws.neon.tech/sql");
    }
}
