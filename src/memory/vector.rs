use async_trait::async_trait;
use crate::memory::MemoryProvider;
use std::fmt::Debug;
use serde_json::json;

#[derive(Debug)]
pub struct VectorMemory {
    base_url: String,
    collection: String,
}

impl VectorMemory {
    pub fn new(base_url: &str, collection: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            collection: collection.to_string(),
        }
    }
}

#[async_trait]
impl MemoryProvider for VectorMemory {
    async fn store(&self, key: &str, value: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        // This is a simplified ChromaDB API call (upsert)
        let url = format!("{}/api/v1/collections/{}/upsert", self.base_url, self.collection);
        
        let body = json!({
            "ids": [key],
            "documents": [value],
            // In a real app, we'd add embeddings here
        });

        client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn retrieve(&self, key: &str) -> Result<Option<String>, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/collections/{}/get", self.base_url, self.collection);
        
        let body = json!({
            "ids": [key]
        });

        let res = client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        if let Some(docs) = json.get("documents").and_then(|v| v.as_array()) {
            if !docs.is_empty() {
                return Ok(docs[0].as_str().map(|s| s.to_string()));
            }
        }

        Ok(None)
    }

    async fn search(&self, query: &str, limit: usize) -> Result<Vec<String>, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/v1/collections/{}/query", self.base_url, self.collection);
        
        let body = json!({
            "query_texts": [query],
            "n_results": limit
        });

        let res = client.post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        
        let mut results = Vec::new();
        if let Some(docs_outer) = json.get("documents").and_then(|v| v.as_array()) {
            if let Some(docs_inner) = docs_outer[0].as_array() {
                for doc in docs_inner {
                    if let Some(s) = doc.as_str() {
                        results.push(s.to_string());
                    }
                }
            }
        }

        Ok(results)
    }
}
