use async_trait::async_trait;
use crate::tools::Tool;
use serde_json::json;

#[derive(Debug)]
pub enum VectorProvider {
    Chroma,
    Qdrant,
    Milvus,
}

#[derive(Debug)]
pub struct VectorLookup {
    provider: VectorProvider,
    base_url: String,
}

impl VectorLookup {
    pub fn new(provider: VectorProvider, url: &str) -> Self {
        Self { provider, base_url: url.to_string() }
    }
}

#[async_trait]
impl Tool for VectorLookup {
    fn name(&self) -> &str {
        "vector_lookup"
    }

    fn description(&self) -> &str {
        "Retrieve relevant context from external vector databases like Chroma, Qdrant, or Milvus."
    }

    async fn execute(&self, query_text: &str) -> Result<String, String> {
        let client = reqwest::Client::new();
        match self.provider {
            VectorProvider::Chroma => {
                let url = format!("{}/api/v1/collections/default/query", self.base_url);
                let body = json!({"query_texts": [query_text], "n_results": 5});
                let res = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
                Ok(res.text().await.unwrap_or_default())
            },
            VectorProvider::Qdrant => {
                let url = format!("{}/collections/default/points/search", self.base_url);
                let body = json!({"vector": [0.0], "limit": 5}); // Simplified
                let res = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
                Ok(res.text().await.unwrap_or_default())
            },
            VectorProvider::Milvus => {
                let url = format!("{}/v1/vector/search", self.base_url);
                let body = json!({"collectionName": "default", "vector": [0.0], "limit": 5});
                let res = client.post(&url).json(&body).send().await.map_err(|e| e.to_string())?;
                Ok(res.text().await.unwrap_or_default())
            }
        }
    }
}
