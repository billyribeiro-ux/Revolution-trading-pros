//! Neon PostgreSQL database connection via HTTP
//! 
//! Uses Neon's serverless driver for edge-compatible database access

use serde::{Deserialize, Serialize};
use crate::error::ApiError;

/// Database client for Neon PostgreSQL
#[derive(Clone)]
pub struct Database {
    connection_string: String,
    http_client: reqwest::Client,
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
        Self {
            connection_string: connection_string.to_string(),
            http_client: reqwest::Client::new(),
        }
    }

    /// Execute a query and return rows
    pub async fn query<T: for<'de> Deserialize<'de>>(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<Vec<T>, ApiError> {
        let response = self.execute_query(sql, params).await?;
        
        let rows: Vec<T> = response.rows
            .into_iter()
            .map(|row| serde_json::from_value(row))
            .collect::<Result<Vec<T>, _>>()
            .map_err(|e| ApiError::Database(format!("Failed to deserialize: {}", e)))?;
        
        Ok(rows)
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

    /// Internal query execution
    async fn execute_query(
        &self,
        sql: &str,
        params: Vec<serde_json::Value>,
    ) -> Result<NeonResponse, ApiError> {
        // Parse connection string to get the HTTP endpoint
        let endpoint = self.get_http_endpoint()?;
        
        let query = NeonQuery {
            query: sql.to_string(),
            params,
        };

        let response = self.http_client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .header("Neon-Connection-String", &self.connection_string)
            .json(&query)
            .send()
            .await
            .map_err(|e| ApiError::Database(format!("HTTP error: {}", e)))?;

        if !response.status().is_success() {
            let error: NeonError = response.json().await
                .unwrap_or(NeonError { message: "Unknown database error".to_string() });
            return Err(ApiError::Database(error.message));
        }

        response.json().await
            .map_err(|e| ApiError::Database(format!("Failed to parse response: {}", e)))
    }

    /// Get the Neon HTTP endpoint from connection string
    fn get_http_endpoint(&self) -> Result<String, ApiError> {
        // Extract host from postgres://user:pass@host/db
        let parts: Vec<&str> = self.connection_string.split('@').collect();
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
