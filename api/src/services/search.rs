//! Search service - Meilisearch

use anyhow::Result;
use meilisearch_sdk::client::Client;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SearchService {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchableCourse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub slug: String,
    pub instructor_name: String,
    pub price_cents: i32,
    pub is_published: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchablePost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub slug: String,
    pub author_name: String,
    pub excerpt: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct SearchResult<T> {
    pub hits: Vec<T>,
    pub query: String,
    pub processing_time_ms: u64,
    pub total_hits: usize,
}

impl SearchService {
    pub fn new(host: &str, api_key: &str) -> Result<Self> {
        let client = Client::new(host, Some(api_key))?;
        Ok(Self { client })
    }

    /// Initialize indexes with settings
    pub async fn setup_indexes(&self) -> Result<()> {
        // Courses index
        let courses_index = self.client.index("courses");
        courses_index
            .set_searchable_attributes(["title", "description", "instructor_name"])
            .await?;
        courses_index
            .set_filterable_attributes(["is_published", "price_cents"])
            .await?;
        courses_index
            .set_sortable_attributes(["price_cents", "title"])
            .await?;

        // Posts index
        let posts_index = self.client.index("posts");
        posts_index
            .set_searchable_attributes(["title", "content", "excerpt", "author_name"])
            .await?;
        posts_index
            .set_filterable_attributes(["is_published"])
            .await?;

        Ok(())
    }

    /// Index a course
    pub async fn index_course(&self, course: SearchableCourse) -> Result<()> {
        let index = self.client.index("courses");
        index.add_documents(&[course], Some("id")).await?;
        Ok(())
    }

    /// Index multiple courses
    pub async fn index_courses(&self, courses: Vec<SearchableCourse>) -> Result<()> {
        let index = self.client.index("courses");
        index.add_documents(&courses, Some("id")).await?;
        Ok(())
    }

    /// Search courses
    pub async fn search_courses(&self, query: &str, limit: usize) -> Result<Vec<SearchableCourse>> {
        let index = self.client.index("courses");
        let results = index
            .search()
            .with_query(query)
            .with_limit(limit)
            .with_filter("is_published = true")
            .execute::<SearchableCourse>()
            .await?;

        Ok(results.hits.into_iter().map(|h| h.result).collect())
    }

    /// Index a post
    pub async fn index_post(&self, post: SearchablePost) -> Result<()> {
        let index = self.client.index("posts");
        index.add_documents(&[post], Some("id")).await?;
        Ok(())
    }

    /// Search posts
    pub async fn search_posts(&self, query: &str, limit: usize) -> Result<Vec<SearchablePost>> {
        let index = self.client.index("posts");
        let results = index
            .search()
            .with_query(query)
            .with_limit(limit)
            .execute::<SearchablePost>()
            .await?;

        Ok(results.hits.into_iter().map(|h| h.result).collect())
    }

    /// Global search across all indexes
    pub async fn search_all(&self, query: &str) -> Result<serde_json::Value> {
        let courses = self.search_courses(query, 5).await.unwrap_or_default();
        let posts = self.search_posts(query, 5).await.unwrap_or_default();

        Ok(serde_json::json!({
            "courses": courses,
            "posts": posts,
            "query": query
        }))
    }

    /// Delete a document from an index
    pub async fn delete_document(&self, index_name: &str, document_id: &str) -> Result<()> {
        let index = self.client.index(index_name);
        index.delete_document(document_id).await?;
        Ok(())
    }
}
